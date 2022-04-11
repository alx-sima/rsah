use std::{
    io::{Read, Write},
    net::TcpStream,
};

use ggez::event::MouseButton;

use crate::{tabla::MatchState, GameState, State};

use super::{input, miscari, notatie, Culoare, Patratel, Piesa, Tabla, TipPiesa};

/// Muta piesa de pe *src_poz* pe *dest_poz*,
/// recalculeaza noile pozitii atacate,
/// schimba randul jucatorilor si
/// returneaza notatia algebrica a miscarii
// FIXME: face prea multe lucruri
pub(crate) fn muta(
    tabla: &mut Tabla,
    src_poz: (usize, usize),
    dest_poz: (usize, usize),
) -> String {
    // Vechea pozitie a piesei
    let p_old = tabla.mat[src_poz.0][src_poz.1].clone();
    // Viitoarea pozitie a piesei
    let p_new = tabla.mat[dest_poz.0][dest_poz.1].clone();

    // Tuturor pieselor care ataca vechea sau noua pozitie
    // le sunt sterse celulele atacate si recalculate dupa mutare
    let mut pcs_to_reset = [p_old.clone().afecteaza, p_new.afecteaza].concat();

    // Mutare en passant (scris noaptea tarziu, nsh ce face)
    if let Some(piesa) = p_old.clone().piesa {
        if piesa.tip == TipPiesa::Pion {
            // Pionul se muta doar pe o singura linie (cand nu ataca).
            if src_poz.1 != dest_poz.1 && p_new.piesa.is_none() {
                let pion_luat = (src_poz.0, dest_poz.1);
                tabla.mat[dest_poz.0][dest_poz.1].piesa = tabla.mat[pion_luat.0][pion_luat.1].piesa.clone();

                pcs_to_reset.append(&mut tabla.mat[pion_luat.0][pion_luat.1].afecteaza);
                pcs_to_reset.append(&mut tabla.mat[pion_luat.0][pion_luat.1].atacat);

                tabla.mat[pion_luat.0][pion_luat.1].piesa = None;
            }
        }
    }

    // FIXME: dc trebuie mutable?
    let mutare = notatie::encode_move(&mut tabla.mat, src_poz, dest_poz);

    // Vechea pozitie a piesei nu va mai ataca
    miscari::clear_influenta(tabla, src_poz);

    // Stergerea pozitiilor atacate
    for poz in &pcs_to_reset {
        miscari::clear_influenta(tabla, *poz);
    }

    // Muta piesa
    tabla.mat[dest_poz.0][dest_poz.1].piesa = Some(Piesa {
        mutat: true,
        ..p_old.clone().piesa.unwrap()
    });
    tabla.mat[src_poz.0][src_poz.1] = Patratel::default();

    // Adauga pozitia precedenta la lista istoricul *piesei*.
    tabla.mat[dest_poz.0][dest_poz.1]
        .piesa
        .as_mut()
        .unwrap()
        .pozitii_anterioare
        .push(src_poz);

    // Cauta miscarile disponibile ale piesei proaspat mutate
    miscari::set_influenta(tabla, dest_poz);
    // Actualizeaza miscarile disponibile pentru piesele care trebuie updatate
    for (i, j) in pcs_to_reset {
        miscari::set_influenta(tabla, (i, j));
    }

    // Verificare pentru rocada
    if p_old.piesa.unwrap().tip == TipPiesa::Rege {
        let dist = dest_poz.1 as i32 - src_poz.1 as i32;
        if dist.abs() == 2 {
            // Daca regele a fost mutat 2 patratele, ori e hacker, ori face rocada
            let dir = dist / 2;
            // Cauta intai tura la distanta de rocada mica, apoi rocada mare.
            // Merg pe incredere ca nu poti face mai multe rocade in acelasi timp
            // si in aceeasi directie.
            for i in [1, 2] {
                let poz_tura = dest_poz.1 as i32 + i * dir;
                if input::in_board(dest_poz.0 as i32, poz_tura) {
                    if let Some(tura) = tabla.mat[dest_poz.0][poz_tura as usize].piesa.clone() {
                        if tura.tip == TipPiesa::Tura {
                            println!("{} {}", dest_poz.0, poz_tura);
                            muta(
                                tabla,
                                (dest_poz.0, poz_tura as usize),
                                (dest_poz.0, (dest_poz.1 as i32 - dir) as usize),
                            );
                        }
                    }
                }
            }
        }
    }

    mutare
}

pub(crate) fn turn_handler(ctx: &mut ggez::Context, state: &mut State) {
    // pe multiplayer, asteaptandu-se randul celuilalt jucator
    if state.game_state == GameState::Multiplayer && (state.turn == Culoare::Negru) != state.guest {
        await_move(state);
    // FIXME: rezolva clickurile
    } else if ggez::input::mouse::button_pressed(ctx, MouseButton::Left) {
        // Clickul se ia in considerare doar daca este in interiorul tablei
        if let Some((click_x, click_y)) = input::get_square_under_mouse(ctx, state.guest) {
            player_turn(
                &mut state.tabla,
                &mut state.piesa_sel,
                &mut state.miscari_disponibile,
                &mut state.turn,
                &mut state.istoric,
                &mut state.stream,
                (click_y, click_x),
            );
        }
    }
}

/// Handler pt randul celuilalt jucator (pe multiplayer).
fn await_move(state: &mut State) {
    let mut tcp_buffer = [0; 16];
    if let Ok(len) = state.stream.as_ref().unwrap().read(&mut tcp_buffer) {
        let msg = std::str::from_utf8(&tcp_buffer[..len]).unwrap();
        if let Some((src_poz, dest_poz)) = notatie::decode_move(&state.tabla.mat, msg, state.turn) {
            // FIXME: CRED ca nu se actualizeaza celulele atacate de pioni
            muta(&mut state.tabla, src_poz, dest_poz);
            state.tabla.ultima_miscare = Some((src_poz, dest_poz));

            // Randul urmatorului jucator
            // Schimba turn din alb in negru si din negru in alb
            state.turn = match state.turn {
                Culoare::Alb => Culoare::Negru,
                Culoare::Negru => Culoare::Alb,
            };

            verif_continua_jocul(&mut state.tabla, &state.turn);
        }
    }
}

// FIXME: prea multe argumente
fn player_turn(
    tabla: &mut Tabla,
    piesa_sel: &mut Option<(usize, usize)>,
    miscari_disponibile: &mut Vec<(usize, usize)>,
    turn: &mut Culoare,
    istoric: &mut Vec<String>,
    stream: &mut Option<TcpStream>,
    dest: (usize, usize),
) {
    // Daca exista o piesa selectata...
    if let Some(src) = *piesa_sel {
        // Daca miscarea este valida, efectueaza mutarea
        if miscari_disponibile.contains(&dest) {
            let mut mov = muta(tabla, src, dest);
            tabla.ultima_miscare = Some((src, dest));

            // Randul urmatorului jucator
            // Schimba turn din alb in negru si din negru in alb
            *turn = match *turn {
                Culoare::Alb => Culoare::Negru,
                Culoare::Negru => Culoare::Alb,
            };

            verif_continua_jocul(tabla, turn);

            if tabla.match_state == MatchState::AlbEMat
                || tabla.match_state == MatchState::NegruEMat
            {
                mov += "#";
            }
            // TODO: mov += "+" daca e sah

            istoric.push(mov.clone());

            if let Some(stream) = stream {
                stream.write_all(mov.as_bytes()).unwrap();
            }
        }
        // Deselecteaza piesa (indiferent daca s-a facut mutarea sau nu)
        *piesa_sel = None;
        *miscari_disponibile = vec![];

    // ...daca nu, o selecteaza (daca e de aceeasi culoare)
    } else if let Some(piesa) = &tabla.mat[dest.0][dest.1].piesa {
        if *turn == piesa.culoare {
            *piesa_sel = Some(dest);
            let miscari = miscari::get_miscari(&tabla, dest, false);

            *miscari_disponibile = miscari::nu_provoaca_sah(&tabla, miscari, dest, *turn);
        }
    }
}

// TODO:
/// Verifica daca jocul mai continua. Returneaza:
/// - None: jocul continua normal;
/// - Some([Alb | Negru]EMat): jocul s-a terminat, jucatorul e in mat;
/// - Some(Pat): jocul s-a terminat cu egalitate;
/// - Some(Playing): jocul continua, dar jucatorul e in sah (nu merita sa fac alt state doar pt asta).
fn verif_continua_jocul(tabla: &mut Tabla, turn: &Culoare) {
    if miscari::verif_sah(&tabla.mat, *turn) {
        // Daca e sah si nu exista miscari, e mat.
        if !miscari::exista_miscari(&tabla, *turn) {
            if *turn == Culoare::Alb {
                tabla.match_state = MatchState::AlbEMat;
            } else {
                tabla.match_state = MatchState::NegruEMat;
            }

            // ...altfel, e sah normal.
        } else {
            //mov += "+";
        }
    // Daca nu e sah si nu exista miscari, e pat.
    } else if !miscari::exista_miscari(&tabla, *turn) {
        // TODO:
        tabla.match_state = MatchState::Pat;
    }
}

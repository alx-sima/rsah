use std::io::{Read, Write};

use ggez::event::MouseButton;

use crate::{GameState, State};

use super::{
    input::{self, in_board},
    miscari, notatie,
    sah::{self, verif_sah},
    Culoare, MatchState, Patratel, Piesa, Pozitie, Tabla, TipPiesa,
};

/// Muta piesa de pe *src_poz* pe *dest_poz*,
/// recalculeaza noile pozitii atacate,
/// schimba randul jucatorilor si
/// returneaza notatia algebrica a miscarii
// FIXME: face prea multe lucruri
pub(crate) fn muta(tabla: &mut Tabla, src: Pozitie, dest: Pozitie) -> String {
    // Vechea pozitie a piesei
    let p_old = tabla.at(src).clone();
    let piesa = p_old.piesa.clone().unwrap();
    let culoare = piesa.culoare;
    // Viitoarea pozitie a piesei
    let p_new = tabla.at(dest).clone();

    // Tuturor pieselor care ataca vechea sau noua pozitie
    // le sunt sterse celulele atacate si recalculate dupa mutare
    let mut pcs_to_reset = [p_old.afecteaza, p_new.afecteaza.clone()].concat();

    // Daca pionul se muta pe diagonala fara sa ia o piesa, e en passant.
    let pion_enpassant = if piesa.tip == TipPiesa::Pion && src.1 != dest.1 && p_new.piesa.is_none()
    {
        let pion_luat = (src.0, dest.1);
        tabla.get(dest).piesa = tabla.at(pion_luat).piesa.clone();

        // Pentru en passant pot exista 2 piese care sa-l faca.
        // Ambele P uri pot lua p prin en passant.
        //   x
        // P p P
        for dir in [-1, 1] {
            let j = dest.1 as i32 + dir;
            if in_board(src.0 as i32, j) {
                let poz = (src.0, j as usize);
                if let Some(piesa) = &tabla.at(poz).piesa {
                    if piesa.tip == TipPiesa::Pion && piesa.culoare == culoare {
                        tabla.get(dest).afecteaza.push(poz);
                    }
                }
            }
        }

        pcs_to_reset.append(&mut tabla.get(pion_luat).afecteaza);
        pcs_to_reset.append(&mut tabla.get(pion_luat).atacat);
        Some(pion_luat)
    } else {
        None
    };

    let mut mutare = notatie::encode_move(&tabla.mat, src, dest);

    if let Some(pion) = pion_enpassant {
        // TODO: sa nu ramana miscrari reziduale.
        tabla.get(pion).piesa = None;
        mutare += " e. p.";
    }

    // Vechea pozitie a piesei nu va mai ataca
    miscari::clear_influenta(tabla, src);

    // Stergerea pozitiilor atacate
    for poz in &pcs_to_reset {
        miscari::clear_influenta(tabla, *poz);
    }

    // Muta piesa
    tabla.get(dest).piesa = Some(Piesa {
        mutat: true,
        ..piesa
    });

    tabla.mat[src.0][src.1] = Patratel::default();

    // Adauga pozitia precedenta la lista istoricul *piesei*.
    tabla
        .get(dest)
        .piesa
        .as_mut()
        .unwrap()
        .pozitii_anterioare
        .push(src);

    // Cauta miscarile disponibile ale piesei proaspat mutate
    miscari::set_influenta(tabla, dest);
    // Actualizeaza miscarile disponibile pentru piesele care trebuie updatate
    for (i, j) in pcs_to_reset {
        miscari::set_influenta(tabla, (i, j));
    }

    if piesa.tip == TipPiesa::Rege {
        // Se updateaza pozitia regelui din `tabla.regi`.
        if culoare == Culoare::Alb {
            tabla.regi.0 = dest;
        } else {
            tabla.regi.1 = dest;
        };

        // Verificare pentru rocada
        // FIXME: da pat dupa rocada
        // Daca regele a fost mutat 2 patratele, ori e hacker, ori face rocada.
        let dist = dest.1 as i32 - src.1 as i32;
        if dist.abs() == 2 {
            let poz_tura = if dist > 0 {
                // Rocada mica
                mutare = String::from("O-O");
                3
            } else {
                // Rocada mare
                mutare = String::from("O-O-O");
                -4
            };
            let poz_tura = src.1 as i32 + poz_tura;

            if input::in_board(dest.0 as i32, poz_tura) {
                if let Some(tura) = tabla.at((dest.0, poz_tura as usize)).piesa.clone() {
                    if tura.tip == TipPiesa::Tura {
                        muta(
                            tabla,
                            (dest.0, poz_tura as usize),
                            (dest.0, (src.1 + dest.1) / 2),
                        );
                    }
                }
            }
        }
    } else if piesa.tip == TipPiesa::Pion {
        let top = if culoare == Culoare::Alb { 0 } else { 7 };
        if dest.0 == top {
            tabla.match_state = MatchState::Promote(dest);
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
        if state.click.is_none() {
            if let Some(pos) = input::get_mouse_square(ctx, state.guest) {
                state.click = Some(pos);
            }
        }
    } else {
        // Clickul se ia in considerare doar daca este in interiorul tablei
        if let Some(click_start) = state.click {
            if let Some(click_end) = input::get_mouse_square(ctx, state.guest) {
                if click_start == click_end {
                    player_turn(state, (click_end.1, click_end.0));
                }
                // else drag & drop??
            }
        }
        state.click = None;
    }
}

/// Handler pt randul celuilalt jucator (pe multiplayer).
fn await_move(state: &mut State) {
    let mut tcp_buffer = [0; 16];
    if let Ok(len) = state.stream.as_ref().unwrap().read(&mut tcp_buffer) {
        let msg = std::str::from_utf8(&tcp_buffer[..len]).unwrap();
        if let Some((src_poz, dest_poz)) = notatie::decode_move(&state.tabla, msg, state.turn) {
            // FIXME: CRED ca nu se actualizeaza celulele atacate de pioni
            muta(&mut state.tabla, src_poz, dest_poz);
            state.tabla.ultima_miscare = Some((src_poz, dest_poz));

            // Randul urmatorului jucator
            // Schimba turn din alb in negru si din negru in alb
            state.turn = match state.turn {
                Culoare::Alb => Culoare::Negru,
                Culoare::Negru => Culoare::Alb,
            };

            // FIXME: ????
            sah::verif_continua_jocul(&state.tabla, state.turn);
        }
    }
}

// FIXME: prea multe argumente
fn player_turn(
    state: &mut State,
    //tabla: &mut Tabla,
    //piesa_sel: &mut Option<(usize, usize)>,
    //miscari_disponibile: &mut Vec<(usize, usize)>,
    //turn: &mut Culoare,
    //stream: &mut Option<TcpStream>,
    click: Pozitie,
) {
    // Daca exista o piesa selectata...
    if let Some(src) = state.piesa_sel {
        // Daca miscarea este valida, efectueaza mutarea
        if state.miscari_disponibile.iter().any(|x| x.dest == click) {
            let mut mov = muta(&mut state.tabla, src, click);
            state.tabla.ultima_miscare = Some((src, click));

            // Randul urmatorului jucator
            // Schimba turn din alb in negru si din negru in alb
            state.turn = state.turn.invert();

            if let Some(end) = sah::verif_continua_jocul(&state.tabla, state.turn) {
                state.tabla.match_state = end;
            }

            if let MatchState::Mat(_) = state.tabla.match_state {
                mov += "#";
            } else if verif_sah(&state.tabla, state.turn) {
                mov += "+";
            }

            state.tabla.istoric.push(mov.clone());

            if let Some(stream) = &mut state.stream {
                stream.write_all(mov.as_bytes()).unwrap();
            }
        }
        // Deselecteaza piesa (indiferent daca s-a facut mutarea sau nu)
        state.piesa_sel = None;
        state.miscari_disponibile = vec![];

    // ...daca nu, o selecteaza (daca e de aceeasi culoare)
    } else if let Some(piesa) = &state.tabla.at(click).piesa {
        if state.turn == piesa.culoare {
            state.piesa_sel = Some(click);
            let miscari = miscari::get_miscari(&state.tabla, click, false);

            state.miscari_disponibile =
                miscari::nu_provoaca_sah(&state.tabla, miscari, click, state.turn);
        }
    }
}

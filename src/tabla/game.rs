use std::{
    io::{Read, Write},
    net::TcpStream,
};

use ggez::event::MouseButton;

use crate::{GameState, State};

use super::{
    input::{self, in_board},
    miscari::{self, verif_sah},
    notatie, Culoare, MatchState, Patratel, Piesa, Pozitie, Tabla, TipPiesa,
};

/// Muta piesa de pe *src_poz* pe *dest_poz*,
/// recalculeaza noile pozitii atacate,
/// schimba randul jucatorilor si
/// returneaza notatia algebrica a miscarii
// FIXME: face prea multe lucruri
pub(crate) fn muta(tabla: &mut Tabla, src: Pozitie, dest: Pozitie) -> String {
    // Vechea pozitie a piesei
    let p_old = tabla.at(src).clone();
    let piesa = p_old.piesa.as_ref().unwrap();
    // Viitoarea pozitie a piesei
    let p_new = tabla.at(dest).clone();

    // Tuturor pieselor care ataca vechea sau noua pozitie
    // le sunt sterse celulele atacate si recalculate dupa mutare
    let mut pcs_to_reset = [p_old.afecteaza.clone(), p_new.afecteaza.clone()].concat();

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
                    if piesa.tip == TipPiesa::Pion
                        && piesa.culoare == p_old.clone().piesa.unwrap().culoare
                    {
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
        ..p_old.clone().piesa.unwrap()
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

    // Verificare pentru rocada
    // FIXME: da pat dupa rocada
    if p_old.piesa.unwrap().tip == TipPiesa::Rege {
        // Daca regele a fost mutat 2 patratele, ori e hacker, ori face rocada
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
                    player_turn(
                        &mut state.tabla,
                        &mut state.piesa_sel,
                        &mut state.miscari_disponibile,
                        &mut state.turn,
                        &mut state.stream,
                        (click_end.1, click_end.0),
                    );
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

            verif_continua_jocul(&state.tabla, state.turn);
        }
    }
}

// FIXME: prea multe argumente
fn player_turn(
    tabla: &mut Tabla,
    piesa_sel: &mut Option<(usize, usize)>,
    miscari_disponibile: &mut Vec<(usize, usize)>,
    turn: &mut Culoare,
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

            tabla.match_state = verif_continua_jocul(tabla, *turn);

            if tabla.match_state == MatchState::AlbEMat
                || tabla.match_state == MatchState::NegruEMat
            {
                mov += "#";
            } else if verif_sah(tabla, *turn) {
                mov += "+";
            }

            tabla.istoric.push(mov.clone());

            if let Some(stream) = stream {
                stream.write_all(mov.as_bytes()).unwrap();
            }
        }
        // Deselecteaza piesa (indiferent daca s-a facut mutarea sau nu)
        *piesa_sel = None;
        *miscari_disponibile = vec![];

    // ...daca nu, o selecteaza (daca e de aceeasi culoare)
    } else if let Some(piesa) = &tabla.at(dest).piesa {
        if *turn == piesa.culoare {
            *piesa_sel = Some(dest);
            let miscari = miscari::get_miscari(tabla, dest, false);

            *miscari_disponibile = miscari::nu_provoaca_sah(tabla, miscari, dest, *turn);
        }
    }
}

/// FIXME: Doamne fereste
/// Verifica daca piesele ramase pot termina meciul cu mat.
fn verif_piese_destule(tabla: &Tabla) -> bool {
    let (
        mut cal_alb,
        mut cal_negru,
        mut nebun_alb_alb,
        mut nebun_alb_negru,
        mut nebun_negru_alb,
        mut nebun_negru_negru,
        mut ok,
    ) = (0, 0, 0, 0, 0, 0, true);
    // Cautam piesele ramase pe toata tabla.
    // Daca se gasesc alte piese decat cal,
    // nebun si rege sau mai multe de acelasi
    // fel, meciul poate fi terminat cu mat.
    'check_piese: for i in 0..8 {
        for j in 0..8 {
            if let Some(piesa) = &tabla.at((i, j)).piesa {
                if piesa.tip == TipPiesa::Cal {
                    if piesa.culoare == Culoare::Alb {
                        cal_alb += 1;
                        if cal_alb > 1 {
                            ok = false;
                            break 'check_piese;
                        }
                    } else {
                        cal_negru += 1;
                        if cal_negru > 1 {
                            ok = false;
                            break 'check_piese;
                        }
                    }
                } else if piesa.tip == TipPiesa::Nebun {
                    if piesa.culoare == Culoare::Alb {
                        if (i + j) % 2 == 0 {
                            nebun_alb_alb += 1;
                            if nebun_alb_alb > 1 {
                                ok = false;
                                break 'check_piese;
                            }
                        } else {
                            nebun_alb_negru += 1;
                            if nebun_alb_negru > 1 {
                                ok = false;
                                break 'check_piese;
                            }
                        }
                    } else if (i + j) % 2 == 0 {
                        nebun_negru_alb += 1;
                        if nebun_negru_alb > 1 {
                            ok = false;
                            break 'check_piese;
                        }
                    } else {
                        nebun_negru_negru += 1;
                        if nebun_negru_negru > 1 {
                            ok = false;
                            break 'check_piese;
                        }
                    }
                } else if piesa.tip != TipPiesa::Rege {
                    ok = false;
                    break 'check_piese;
                }
            }
        }
    }

    // Verificam daca exista unul din cazurile de pat (rege + cal vs rege;    rege + nebun vs rege;    rege + nebun vs rege + nebun unde nebunii sunt pe patrat de acelasi culoare)
    if ok {
        // Cal + nicio alta piesa
        if cal_alb == 1
            && (cal_negru == 1
                || nebun_alb_alb == 1
                || nebun_alb_negru == 1
                || nebun_negru_alb == 1
                || nebun_negru_negru == 1)
        {
            ok = false;
        }
        if cal_negru == 1
            && (cal_alb == 1
                || nebun_alb_alb == 1
                || nebun_alb_negru == 1
                || nebun_negru_alb == 1
                || nebun_negru_negru == 1)
        {
            ok = false;
        }

        // Nebun + nicio alta piesa / nebun pe patrat de aceeasi culoare
        if nebun_alb_alb == 1
            && (cal_alb == 1 || cal_negru == 1 || nebun_alb_negru == 1 || nebun_negru_negru == 1)
        {
            ok = false;
        }
        if nebun_alb_negru == 1
            && (cal_alb == 1 || cal_negru == 1 || nebun_alb_alb == 1 || nebun_negru_alb == 1)
        {
            ok = false;
        }
        if nebun_negru_alb == 1
            && (cal_alb == 1 || cal_negru == 1 || nebun_alb_negru == 1 || nebun_negru_negru == 1)
        {
            ok = false;
        }
        if nebun_negru_negru == 1
            && (cal_alb == 1 || cal_negru == 1 || nebun_alb_alb == 1 || nebun_negru_alb == 1)
        {
            ok = false;
        }
    }
    ok
}

/// Verifica daca aceeasi pozitie a avut loc de 3 ori consecutiv.
fn threefold(tabla: &Tabla) -> bool {
    let istoric = &tabla.istoric;

    // Trebuie sa existe minimum 9 miscari pentru a avea 3 pozitii la fel consecutive
    if istoric.len() < 9 {
        return false;
    }
    let last = istoric.len() - 1;
    let mut ok = true;

    // Pozitia curenta trebuie sa se fi repetat de 2 ori pana acum
    if istoric[last] == istoric[last - 4] && istoric[last] == istoric[last - 8] {
        ok = false;
        // Pozitiile precedente au avut loc doar de 2 ori
        for i in 1..4 {
            if istoric[last - i] != istoric[last - i - 4] {
                ok = true;
            }
        }
    }

    ok
}

/// TODO: testeaza
// Verificam daca in ultimele 50 de miscari (ale fiecarui jucator) nu au fost capturate piese
fn fifty_move(tabla: &Tabla) -> bool {
    let istoric = &tabla.istoric;

    // Trebuie sa existe minimum 100 de miscari
    if istoric.len() < 100 {
        return false;
    }

    let last = istoric.len() - 1;

    // Pentru fiecare jucator, verificam daca in ultimele 50 de miscari au fost capturate piese
    for i in 0..100 {
        if istoric[last - i].contains('x') {
            return false;
        }
    }

    true
}

/// Verifica daca jocul mai continua. Returneaza:
/// - `(Alb|Negru)EMat`: jocul s-a terminat, jucatorul e in mat;
/// - `Pat`: jocul s-a terminat cu egalitate;
/// - `Playing`: jocul continua.
pub(crate) fn verif_continua_jocul(tabla: &Tabla, turn: Culoare) -> MatchState {
    if !miscari::exista_miscari(tabla, turn) {
        // Daca e sah si nu exista miscari, e mat.
        if miscari::verif_sah(tabla, turn) {
            if turn == Culoare::Alb {
                return MatchState::AlbEMat;
            }
            return MatchState::NegruEMat;
        }
        return MatchState::Pat;
    }

    // Verificam cele trei conditii de egalitate
    if verif_piese_destule(tabla) || threefold(tabla) || fifty_move(tabla) {
        return MatchState::Pat;
    }
    MatchState::Playing
}

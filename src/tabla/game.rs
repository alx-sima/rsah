use std::io::{Read, Write};

use ggez::event::MouseButton;

use crate::{GameState, Mutare, State, TipMutare};

use super::{
    input,
    miscari, notatie,
    sah::{self, verif_sah},
    Culoare, MatchState, Patratel, Piesa, Pozitie, Tabla, TipPiesa,
};

/// Muta piesa de pe *src_poz* pe *dest_poz*,
/// recalculeaza noile pozitii atacate,
/// schimba randul jucatorilor si
/// returneaza notatia algebrica a miscarii
// FIXME: face prea multe lucruri
pub(crate) fn muta(tabla: &mut Tabla, src: Pozitie, mutare: &Mutare) -> String {
    // Vechea pozitie a piesei
    let p_old = tabla.at(src).clone();
    let piesa = p_old.piesa.clone().unwrap();
    let culoare = piesa.culoare;
    // Viitoarea pozitie a piesei
    let p_new = tabla.at(mutare.dest).clone();

    // Tuturor pieselor care ataca vechea sau noua pozitie
    // le sunt sterse celulele atacate si recalculate dupa mutare
    let pcs_to_reset = [p_old.afecteaza, p_new.afecteaza].concat();

    // Daca pionul se muta pe diagonala fara sa ia o piesa, e en passant.
    if piesa.tip == TipPiesa::Pion {
        if let TipMutare::EnPassant(victima) = mutare.tip {
            tabla.get(mutare.dest).piesa = tabla.at(victima).piesa.clone();

            tabla.mat[victima.0][victima.1].piesa = None;
        }
    }

    let mut notatie = notatie::encode_move(&tabla.mat, src, mutare);

    // Vechea pozitie a piesei nu va mai ataca
    miscari::clear_influenta(tabla, src);

    // Stergerea pozitiilor atacate
    for poz in &pcs_to_reset {
        miscari::clear_influenta(tabla, *poz);
    }

    // Muta piesa
    tabla.get(mutare.dest).piesa = Some(Piesa {
        mutat: true,
        ..piesa
    });

    tabla.mat[src.0][src.1] = Patratel::default();

    // Adauga pozitia precedenta la lista istoricul `piesei`.
    tabla
        .get(mutare.dest)
        .piesa
        .as_mut()
        .unwrap()
        .pozitii_anterioare
        .push(src);

    // Cauta miscarile disponibile ale piesei proaspat mutate
    miscari::set_influenta(tabla, mutare.dest);
    // Actualizeaza miscarile disponibile pentru piesele care trebuie updatate
    for (i, j) in pcs_to_reset {
        miscari::set_influenta(tabla, (i, j));
    }

    if piesa.tip == TipPiesa::Rege {
        // Se updateaza pozitia regelui din `tabla.regi`.
        if culoare == Culoare::Alb {
            tabla.regi.0 = mutare.dest;
        } else {
            tabla.regi.1 = mutare.dest;
        };

        // Verificare pentru rocada.
        // FIXME: da pat dupa rocada
        if let TipMutare::Rocada(tura) = mutare.tip {
            // Directia in care se face rocada
            if tura.1 > src.1 {
                // Rocada mica
                notatie = String::from("O-O");
            } else {
                // Rocada mare
                notatie = String::from("O-O-O");
            }

            muta(
                tabla,
                tura,
                &Mutare {
                    tip: TipMutare::Normal,
                    dest: (src.0, (src.1 + mutare.dest.1) / 2),
                },
            );
        }
    } else if piesa.tip == TipPiesa::Pion {
        let top = if culoare == Culoare::Alb { 0 } else { 7 };
        if mutare.dest.0 == top {
            tabla.match_state = MatchState::Promote(mutare.dest);
        }
    }

    notatie
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
        if let Some((src, mutare)) = notatie::decode_move(&state.tabla, msg, state.turn) {
            // FIXME:
            muta(&mut state.tabla, src, &mutare);
            state.tabla.ultima_miscare = Some((src, mutare.dest));

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
        if let Some(mutare) = state.miscari_disponibile.iter().find(|x| x.dest == click) {
            let mut mov = muta(&mut state.tabla, src, mutare);
            state.tabla.ultima_miscare = Some((src, mutare.dest));

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

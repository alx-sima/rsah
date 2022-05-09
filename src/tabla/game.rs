use std::io::{Read, Write};

use ggez::event::MouseButton;

use crate::{tabla::miscari::TipMutare, GameState, Mutare, State};

use super::{
    input, miscari, notatie, sah, Culoare, MatchState, Patratel, Piesa, Pozitie, Tabla, TipPiesa,
};

/// Handlerul principal pt. joc (singleplayer + multiplayer).
pub(crate) fn turn_handler(ctx: &mut ggez::Context, state: &mut State) {
    // Pe multiplayer, se asteapta miscarea celuilalt jucator.
    if state.game_state == GameState::Multiplayer && (state.turn == Culoare::Negru) != state.guest {
        await_move(state);

        // Clickul se ia in considerare doar daca este in interiorul tablei.
    } else if let Some(click) = input::get_mouse_square(ctx, state.guest) {
        if ggez::input::mouse::button_pressed(ctx, MouseButton::Left) {
            // Clickul abia a fost inceput, asa ca se cauta
            // miscarile daca nu e deja o piesa selectata.
            if state.click.is_none() {
                state.click = Some(click);
            }
        }
        // Clickul s-a terminat; daca e pe
        // acelasi patratel, se face mutarea.
        else if let Some(click_start) = state.click {
            if click_start == click {
                your_turn(state, click);
            }
            state.click = None;
        }
    }
}

/// Handler pt. asteptat mutarea celuilalt jucator (pe multiplayer).
fn await_move(state: &mut State) {
    let mut tcp_buffer = [0; 16];
    if let Ok(len) = state.stream.as_ref().unwrap().read(&mut tcp_buffer) {
        let notatie = std::str::from_utf8(&tcp_buffer[..len]).unwrap();

        if let Some((src, mutare)) = notatie::decode_move(&state.tabla, notatie, state.turn) {
            // Mutarea este adaugata in istoric
            state.tabla.istoric.push(notatie.to_string());

            muta(&mut state.tabla, src, &mutare);
            state.tabla.ultima_miscare = Some((src, mutare.dest));

            // Daca pionul este promovat, se updateaza aici.
            if let TipMutare::Promovare(piesa) = mutare.tip {
                state.tabla.get(mutare.dest).piesa = Some(Piesa::new(piesa, state.turn));
                set_piesa(&mut state.tabla, mutare.dest, Some(piesa), state.turn);
                state.tabla.match_state = MatchState::Playing;
            }

            // Randul urmatorului jucator
            state.turn.invert();
        } else {
            println!("{notatie}");
        }
    }
}

/// Handler pt. randul oricarui jucatorul (singleplayer),
/// sau al celui de pe acest device (multiplayer).
fn your_turn(state: &mut State, pos: Pozitie) {
    if let Some(src) = state.piesa_sel {
        // Daca miscarea este valida, efectueaza mutarea.
        if let Some(mutare) = state.miscari_disponibile.iter().find(|x| x.dest == pos) {
            let mut notatie = muta(&mut state.tabla, src, mutare);
            state.tabla.ultima_miscare = Some((src, mutare.dest));

            // Randul urmatorului jucator
            state.turn.invert();

            if let Some(end_state) = sah::verif_continua_jocul(&state.tabla, state.turn) {
                state.tabla.match_state = end_state;
            }

            if let MatchState::Mat(_) = state.tabla.match_state {
                notatie += "#";
            } else if sah::in_sah(&state.tabla, state.turn) {
                notatie += "+";
            }

            if let MatchState::Promote(_) = state.tabla.match_state {
                state.mutare_buf = notatie;
                return;
            }

            // Daca nu se asteapta promovarea unui pion, se scrie mutarea.
            state.tabla.istoric.push(notatie.clone());
            if let Some(stream) = &mut state.stream {
                stream.write_all(notatie.as_bytes()).unwrap();
            }
        }

        // Deselecteaza piesa.
        state.piesa_sel = None;
        state.miscari_disponibile = vec![];
        // Daca nu e deja selectata, se
        // va selecta piesa de sub mouse.
    } else if let Some(piesa) = &state.tabla.at(pos).piesa {
        if piesa.culoare == state.turn {
            state.miscari_disponibile = miscari::nu_provoaca_sah(&state.tabla, pos, state.turn);
            state.piesa_sel = Some(pos);
        }
    }
}

/// Muta piesa de pe `src_poz` pe `dest_poz`,
/// recalculeaza noile pozitii atacate,
/// schimba randul jucatorilor si
/// returneaza notatia algebrica a miscarii
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
            // Piesa 'victima' este mutata 1 patrat
            // deasupra, pe pozitia unde va fi
            // mutat pionul 'agresor'.
            set_piesa(tabla, mutare.dest, Some(TipPiesa::Pion), culoare);
            set_piesa(tabla, victima, None, culoare);
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

///  - Some(`piesa`) => Amplaseaza `piesa` de
/// `culoare` pe pozitia `poz`, resetand
/// celulele influentate.
///
///  - `None` => Sterge `poz` din tabla,
/// resetand celulele influentate.
///
/// Nu verifica rocade sau en passant, fiind
/// intentionata doar pt promovarea pionului.
pub(crate) fn set_piesa(
    tabla: &mut Tabla,
    poz: Pozitie,
    piesa: Option<TipPiesa>,
    culoare: Culoare,
) {
    // Patratul de modificat.
    let patr = tabla.at(poz).clone();

    let mut pcs_to_reset = patr.afecteaza;
    pcs_to_reset.push(poz);

    for i in &pcs_to_reset {
        miscari::clear_influenta(tabla, *i);
    }

    if let Some(piesa) = piesa {
        tabla.get(poz).piesa = Some(Piesa {
            pozitii_anterioare: vec![],
            mutat: true,
            tip: piesa,
            culoare,
        });
    } else {
        tabla.get(poz).piesa = None;
    }

    for i in pcs_to_reset {
        miscari::set_influenta(tabla, i);
    }
}

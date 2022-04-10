use std::{io::Write, net::TcpStream};

use crate::tabla::MatchState;

use super::{input, miscari, notatie, Culoare, MatTabla, Patratel, Piesa, Tabla, TipPiesa};

/// Muta piesa de pe *src_poz* pe *dest_poz*,
/// recalculeaza noile pozitii atacate,
/// schimba randul jucatorilor si
/// returneaza notatia algebrica a miscarii
pub(crate) fn muta(
    tabla: &mut MatTabla,
    src_poz: (usize, usize),
    dest_poz: (usize, usize),
) -> String {
    // Vechea pozitie a piesei
    let p_old = tabla[src_poz.0][src_poz.1].clone();
    // Viitoarea pozitie a piesei
    let p_new = tabla[dest_poz.0][dest_poz.1].clone();

    // Tuturor pieselor care ataca vechea sau noua pozitie
    // le sunt sterse celulele atacate si recalculate dupa mutare
    let mut pcs_to_reset = [p_old.clone().afecteaza, p_new.afecteaza].concat();

    // Mutare en passant (scris noaptea tarziu, nsh ce face)
    if let Some(piesa) = p_old.clone().piesa {
        if piesa.tip == TipPiesa::Pion {
            // Pionul se muta doar pe o singura linie (cand nu ataca).
            if src_poz.1 != dest_poz.1 && p_new.piesa.is_none() {
                let pion_luat = (src_poz.0, dest_poz.1);
                tabla[dest_poz.0][dest_poz.1].piesa = tabla[pion_luat.0][pion_luat.1].piesa.clone();

                pcs_to_reset.append(&mut tabla[pion_luat.0][pion_luat.1].afecteaza);
                pcs_to_reset.append(&mut tabla[pion_luat.0][pion_luat.1].atacat);

                tabla[pion_luat.0][pion_luat.1].piesa = None;
            }
        }
    }

    let mutare = notatie::encode_move(tabla, src_poz, dest_poz);

    // Vechea pozitie a piesei nu va mai ataca
    miscari::clear_influenta(tabla, src_poz);

    // Stergerea pozitiilor atacate
    for poz in &pcs_to_reset {
        miscari::clear_influenta(tabla, *poz);
    }

    // Muta piesa
    tabla[dest_poz.0][dest_poz.1].piesa = Some(Piesa {
        mutat: true,
        ..p_old.clone().piesa.unwrap()
    });
    tabla[src_poz.0][src_poz.1] = Patratel::default();

    // Adauga pozitia precedenta la lista istoricul *piesei*.
    tabla[dest_poz.0][dest_poz.1]
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
                    if let Some(tura) = tabla[dest_poz.0][poz_tura as usize].piesa.clone() {
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

pub(crate) fn player_turn(
    ctx: &mut ggez::Context,
    tabla: &mut Tabla,
    piesa_sel: &mut Option<(usize, usize)>,
    miscari_disponibile: &mut Vec<(usize, usize)>,
    turn: &mut Culoare,
    istoric: &mut Vec<String>,
    stream: &mut Option<TcpStream>,
    reversed: bool,
) {
    // Daca clickul este in interiorul tablei
    if let Some((dest_j, dest_i)) = input::get_square_under_mouse(ctx, reversed) {
        // Daca exista o piesa selectata...
        if let Some(src_poz) = *piesa_sel {
            // Daca miscarea este valida, efectueaza mutarea
            if miscari_disponibile.contains(&(dest_i, dest_j)) {
                let mut mov = muta(&mut tabla.mat, src_poz, (dest_i, dest_j));
                tabla.ultima_miscare = Some((src_poz, (dest_i, dest_j)));

                // Randul urmatorului jucator
                // Schimba turn din alb in negru si din negru in alb
                *turn = match *turn {
                    Culoare::Alb => Culoare::Negru,
                    Culoare::Negru => Culoare::Alb,
                };

                if miscari::verif_sah(&tabla.mat, *turn) {
                    // Daca e sah si nu exista miscari, e mat
                    if !miscari::exista_miscari(&tabla.mat, *turn) {
                        if *turn == Culoare::Alb {
                            tabla.match_state = MatchState::AlbEMat;
                        } else {
                            tabla.match_state = MatchState::NegruEMat;
                        }
                        println!("{:?} e in mat", *turn);
                        mov += "#";

                    // altfel, e sah normal
                    } else {
                        println!("{:?} e in sah", *turn);
                        mov += "+";
                    }
                // Daca nu e sah si nu exista miscari, e pat
                } else if !miscari::exista_miscari(&tabla.mat, *turn) {
                    // TODO
                    tabla.match_state = MatchState::Pat;
                    println!("{:?} e in pat", *turn);
                }

                istoric.push(mov.clone());

                if let Some(stream) = stream {
                    stream.write_all(mov.as_bytes()).unwrap();
                }
            }
            // Deselecteaza piesa (indiferent daca s-a facut mutarea sau nu)
            *piesa_sel = None;
            *miscari_disponibile = vec![];

        // ...daca nu, o selecteaza (daca e de aceeasi culoare)
        } else if let Some(piesa) = &tabla.mat[dest_i][dest_j].piesa {
            if *turn == piesa.culoare {
                *piesa_sel = Some((dest_i, dest_j));
                let miscari = miscari::get_miscari(&tabla.mat, (dest_i, dest_j), false);

                *miscari_disponibile =
                    miscari::nu_provoaca_sah(&tabla.mat, miscari, (dest_i, dest_j), *turn);
            }
        }
    }
}

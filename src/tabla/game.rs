use std::{io::Write, net::TcpStream};

use super::{istoric, set_atacat_field, Culoare, Patratel};

use crate::{
    cautare_miscari,
    tabla::{in_board, Piesa, TipPiesa},
};

/// Muta piesa de pe *src_poz* pe *dest_poz*,
/// recalculeaza noile pozitii atacate,
/// schimba randul jucatorilor si
/// returneaza notatia algebrica a miscarii
pub(crate) fn muta(
    tabla: &mut [[Patratel; 8]; 8],
    src_poz: (usize, usize),
    dest_poz: (usize, usize),
) -> String {
    // Vechea pozitie a piesei
    let p_old = tabla[src_poz.0][src_poz.1].clone();
    // Viitoarea pozitie a piesei
    let p_new = tabla[dest_poz.0][dest_poz.1].clone();

    // Tuturor pieselor care ataca vechea sau noua pozitie
    // le sunt sterse celulele atacate si recalculate dupa mutare
    let pcs_to_reset = [p_old.clone().atacat, p_new.atacat].concat();

    let mutare = istoric::encode_move(tabla, src_poz, dest_poz);

    // Vechea pozitie a piesei nu va mai ataca
    cautare_miscari::clear_attack(tabla, src_poz.0, src_poz.1);

    // Stergerea pozitiilor atacate
    for (i, j) in &pcs_to_reset {
        cautare_miscari::clear_attack(tabla, *i, *j);
    }

    // Muta piesa
    tabla[dest_poz.0][dest_poz.1].piesa = Some(Piesa {
        mutat: true,
        ..p_old.clone().piesa.unwrap()
    });
    tabla[src_poz.0][src_poz.1] = Patratel::default();

    // Cauta miscarile disponibile ale piesei proaspat mutate
    set_atacat_field(tabla, dest_poz.0, dest_poz.1);
    // Actualizeaza miscari disponibile pentru piesele care atacau pozitiile
    for (i, j) in pcs_to_reset {
        set_atacat_field(tabla, i, j);
    }

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
                if in_board(dest_poz.0 as i32, poz_tura) {
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
    cautare_miscari::verif_sah(tabla);

    // FIXME: match_state nu ar tb sa tina sahul pt fiecare jucator?
    // Adauga la istoric pentru miscarea curenta:
    // - "+" in caz ca se afla in sah;
    // - "#" in caz ca se afla in mat.
    //if *match_state == MatchState::Sah {
    //    mutare += "+";
    //}
    //if *match_state == MatchState::Mat {
    //    mutare += "#";
    //}

    println!("{}", &mutare);

    for i in tabla {
        println!("{:?}", i);
    }
    mutare
}

pub(crate) fn player_turn(
    ctx: &mut ggez::Context,
    tabla: &mut [[Patratel; 8]; 8],
    piesa_sel: &mut Option<(usize, usize)>,
    miscari_disponibile: &mut Vec<(usize, usize)>,
    turn: &mut Culoare,
    istoric: &mut Vec<String>,
    stream: &mut Option<TcpStream>,
    reversed: bool,
) {
    // Daca clickul este in interiorul tablei
    if let Some((dest_j, dest_i)) = super::get_square_under_mouse(ctx, reversed) {
        // Daca exista o piesa selectata...
        if let Some(src_poz) = *piesa_sel {
            // Daca miscarea este valida, efectueaza mutarea
            if miscari_disponibile.contains(&(dest_i, dest_j)) {
                let mov = muta(tabla, src_poz, (dest_i, dest_j));
                istoric.push(mov.clone());

                if let Some(stream) = stream {
                    stream.write(mov.as_bytes()).unwrap();
                }

                // Randul urmatorului jucator
                // Schimba turn din alb in negru si din negru in alb
                *turn = match *turn {
                    Culoare::Alb => Culoare::Negru,
                    Culoare::Negru => Culoare::Alb,
                };
            }
            // Deselecteaza piesa (indiferent daca s-a facut mutarea sau nu)
            *piesa_sel = None;
            *miscari_disponibile = vec![];

        // ...daca nu, o selecteaza (daca e de aceeasi culoare)
        } else {
            if let Some(piesa) = &tabla[dest_i][dest_j].piesa {
                if *turn == piesa.culoare {
                    *piesa_sel = Some((dest_i, dest_j));
                    *miscari_disponibile =
                        cautare_miscari::get_miscari(tabla, dest_i as i32, dest_j as i32, false);
                }
            }
        }
    }
}

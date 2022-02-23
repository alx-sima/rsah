use super::{set_atacat_field, Culoare, Patratel};

use crate::{miscari, MatchState};

/// Muta piesa de pe (src_poz) pe (dest_poz),
/// recalculeaza noile pozitii atacate,
/// adauga in 'istoric' miscarea
/// si schimba randul jucatorilor
pub(crate) fn muta(
    tabla: &mut [[Patratel; 8]; 8],
    turn: &mut Culoare,
    match_state: &mut MatchState,
    istoric: &mut Vec<String>,
    src_poz: (usize, usize),
    dest_poz: (usize, usize),
) {
    // Vechea pozitie a piesei
    let p_old = tabla[src_poz.0][src_poz.1].clone();
    // Viitoarea pozitie a piesei
    let p_new = tabla[dest_poz.0][dest_poz.1].clone();

    // Tuturor pieselor care ataca vechea sau noua pozitie
    // le sunt sterse celulele atacate si recalculate dupa mutare
    let pcs_to_reset = [p_old.clone().atacat, p_new.atacat].concat();

    let mut mutare = encode_move(tabla, src_poz, dest_poz);

    // Vechea pozitie a piesei nu va mai ataca
    miscari::clear_attack(tabla, src_poz.0 as i32, src_poz.1 as i32);

    // Stergerea pozitiilor atacate
    for (i, j) in &pcs_to_reset {
        miscari::clear_attack(tabla, *i, *j);
    }

    // Muta piesa
    tabla[dest_poz.0][dest_poz.1] = p_old.clone();
    tabla[src_poz.0][src_poz.1] = Patratel::default();

    // Cauta miscarile disponibile ale piesei proaspat mutate
    set_atacat_field(tabla, dest_poz.0 as i32, dest_poz.1 as i32);
    // Actualizeaza miscari disponibile pentru piesele care atacau pozitiile
    for (i, j) in pcs_to_reset {
        set_atacat_field(tabla, i, j);
    }

    miscari::verif_sah(tabla);

    // FIXME: match_state nu ar tb sa tina sahul pt fiecare jucator?
    // Adauga la istoric pentru miscarea curenta:
    // - "+" in caz ca se afla in sah;
    // - "#" in caz ca se afla in mat.
    if *match_state == MatchState::Sah {
        mutare += "+";
    }
    if *match_state == MatchState::Mat {
        mutare += "#";
    }

    println!("{}", &mutare);
    istoric.push(mutare);

    // Randul urmatorului jucator
    // Schimba turn din alb in negru si din negru in alb
    *turn = match *turn {
        Culoare::Alb => Culoare::Negru,
        Culoare::Negru => Culoare::Alb,
    };
}

pub(crate) fn player_turn(
    ctx: &mut ggez::Context,
    tabla: &mut [[Patratel; 8]; 8],
    piesa_sel: &mut Option<(usize, usize)>,
    miscari_disponibile: &mut Vec<(usize, usize)>,
    turn: &mut Culoare,
    match_state: &mut MatchState,
    istoric: &mut Vec<String>,
) {
    // Daca clickul este in interiorul tablei
    if let Some((dest_j, dest_i)) = super::get_square_under_mouse(ctx) {
        // Daca exista o piesa selectata...
        if let Some((src_i, src_j)) = *piesa_sel {
            // Daca miscarea este valida, efectueaza mutarea
            if miscari_disponibile.contains(&(dest_i, dest_j)) {
                muta(
                    tabla,
                    turn,
                    match_state,
                    istoric,
                    (src_i, src_j),
                    (dest_i, dest_j),
                );
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
                        miscari::get_miscari(tabla, dest_i as i32, dest_j as i32, false);
                }
            }
        }
    }
}

fn encode_move(
    tabla: &mut [[Patratel; 8]; 8],
    src_poz: (usize, usize),
    dest_poz: (usize, usize),
) -> String {
    let p_old = tabla[src_poz.0][src_poz.1].clone();
    let p_new = tabla[dest_poz.0][dest_poz.1].clone();
    // Adaugare miscare in istoric
    // FIXME: scapa de unwrap
    let mut mutare = format!("{}", p_old.clone().piesa.unwrap().tip);

    let mut scrie_lin = false;
    let mut scrie_col = false;
    for k in &tabla[dest_poz.0][dest_poz.1].atacat {
        // Exista o alta piesa care ataca celula destinatie...
        if k.0 != src_poz.0 as i32 || k.1 != src_poz.1 as i32 {
            // ... si e de acelasi tip
            if tabla[k.0 as usize][k.1 as usize].piesa == p_old.clone().piesa {
                // Daca sunt pe aceeasi coloana se va afisa linia
                if k.1 == src_poz.1 as i32 {
                    scrie_lin = true;
                // Daca nu, se scrie coloana, indiferent daca sunt pe aceeasi linie sau nu (se poate intampla la cai de ex.)
                } else {
                    scrie_col = true;
                }
            }
        }
    }

    if scrie_col {
        // Wall of shame: aduni un string cu un numar si vrei sa-ti dea un caracter
        // Ce voiai sa faci: 'a' + 1 = 'b'
        // Ce ai facut: 1 + "a" = "1a"

        // 65 = 'a'
        mutare += ((src_poz.1 as u8 + 97u8) as char).to_string().as_str();
    }
    if scrie_lin {
        mutare += (8 - src_poz.0).to_string().as_str();
    }

    if p_new.piesa.is_some() {
        mutare += "x"
    }

    mutare += ((dest_poz.1 as u8 + 97u8) as char).to_string().as_str();
    mutare += (8 - dest_poz.0).to_string().as_str();
    mutare
}

fn decode_move(mov: &str) -> (usize, usize, usize, usize) {
    todo!()
}

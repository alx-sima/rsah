use super::{istoric, set_atacat_field, Culoare, Patratel};

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

    let mut mutare = istoric::encode_move(tabla, src_poz, dest_poz);

    // Vechea pozitie a piesei nu va mai ataca
    miscari::clear_attack(tabla, src_poz.0, src_poz.1);

    // Stergerea pozitiilor atacate
    for (i, j) in &pcs_to_reset {
        miscari::clear_attack(tabla, *i, *j);
    }

    // Muta piesa
    tabla[dest_poz.0][dest_poz.1] = p_old.clone();
    tabla[src_poz.0][src_poz.1] = Patratel::default();

    // Cauta miscarile disponibile ale piesei proaspat mutate
    set_atacat_field(tabla, dest_poz.0, dest_poz.1);
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

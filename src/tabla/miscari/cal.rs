use crate::tabla::{Tabla, input::in_board, PozitieVerificata};

/// Genereaza o lista cu miscarile posibile (linie, coloana) pentru calul de la (i, j)
pub(super) fn get(tabla: &Tabla, i: i32, j: i32, tot_ce_afecteaza: bool) -> Vec<PozitieVerificata> {
    let mut rez = Vec::new();
    let ui = i as usize;
    let uj = j as usize;

    // folosim vectorii de pozitie dx si dy pentru a genera toate miscarile posibile in forma de L
    // (2 patrate pe orizontala, 1 patrate pe verticala / 2 patrate pe verticala si 1 patrate pe orizontala)
    let di = [-2, -2, -1, -1, 1, 1, 2, 2];
    let dj = [1, -1, 2, -2, 2, -2, 1, -1];

    for k in 0..8 {
        if in_board(i + di[k], j + dj[k]) {
            let sumi = (i + di[k]) as usize;
            let sumj = (j + dj[k]) as usize;

            if tabla[sumi][sumj].piesa.is_some() {
                if tabla[ui][uj].piesa.clone().unwrap().culoare
                    != tabla[sumi][sumj].piesa.clone().unwrap().culoare
                    || tot_ce_afecteaza
                {
                    rez.push((sumi, sumj));
                }
            } else {
                rez.push((sumi, sumj));
            }
        }
    }
    rez
}

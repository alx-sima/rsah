use crate::tabla::{input, MatTabla, Pozitie};

use super::Mutare;

/// Genereaza o lista cu miscarile posibile (linie, coloana) pentru calul de pe `poz`
pub(super) fn get(tabla: &MatTabla, poz: Pozitie, check_all: bool) -> Vec<Mutare> {
    let mut rez = vec![];

    let (ui, uj) = poz;
    let i = ui as i32;
    let j = uj as i32;

    // folosim vectorii de pozitie dx si dy pentru a genera toate miscarile posibile in forma de L
    // (2 patrate pe orizontala, 1 patrate pe verticala / 2 patrate pe verticala si 1 patrate pe orizontala)
    let di = [-2, -2, -1, -1, 1, 1, 2, 2];
    let dj = [1, -1, 2, -2, 2, -2, 1, -1];

    for k in 0..8 {
        if input::in_board(i + di[k], j + dj[k]) {
            let sumi = (i + di[k]) as usize;
            let sumj = (j + dj[k]) as usize;

            if let Some(mutare) = super::mutare_imediata(tabla, poz, (sumi, sumj), check_all) {
                rez.push(mutare);
            }
        }
    }
    rez
}

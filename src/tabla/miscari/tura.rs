use crate::tabla::{Pozitie, MatTabla};

use super::cautare_in_linie;

/// Genereaza o lista cu miscarile posibile (linie, coloana) pentru tura de la (i, j)
pub(super) fn get(tabla: &MatTabla, poz: Pozitie, tot_ce_afecteaza: bool) -> Vec<Pozitie> {
    cautare_in_linie(
        tabla,
        poz,
        &[(-1, 0), (0, 1), (1, 0), (0, -1)],
        tot_ce_afecteaza,
    )
}

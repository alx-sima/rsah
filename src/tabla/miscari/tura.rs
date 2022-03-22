use crate::tabla::{PozitieSafe, Tabla};

use super::cautare_in_linie;

/// Genereaza o lista cu miscarile posibile (linie, coloana) pentru tura de la (i, j)
pub(super) fn get(tabla: &Tabla, i: i32, j: i32, tot_ce_afecteaza: bool) -> Vec<PozitieSafe> {
    cautare_in_linie(
        tabla,
        i,
        j,
        &[(-1, 0), (0, 1), (1, 0), (0, -1)],
        tot_ce_afecteaza,
    )
}

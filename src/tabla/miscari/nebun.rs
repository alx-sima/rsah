use crate::tabla::{Tabla, PozitieSafe};

use super::cautare_in_linie;

/// Genereaza o lista cu miscarile posibile (linie, coloana) pentru nebunul de la (i, j)
pub(super) fn get(tabla: &Tabla, poz: PozitieSafe, tot_ce_afecteaza: bool) -> Vec<PozitieSafe> {
    cautare_in_linie(
        tabla,
        poz,
        &[(-1, -1), (-1, 1), (1, 1), (1, -1)],
        tot_ce_afecteaza,
    )
}

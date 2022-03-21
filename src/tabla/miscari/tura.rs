use crate::tabla::Tabla;

use super::cautare_in_linie;

/// Genereaza o lista cu miscarile posibile (linie, coloana) pentru tura de la (i, j)
pub(super) fn get(tabla: &Tabla, i: i32, j: i32, full_scan: bool) -> Vec<(usize, usize)> {
    cautare_in_linie(tabla, i, j, &[(-1, 0), (0, 1), (1, 0), (0, -1)], full_scan)
}

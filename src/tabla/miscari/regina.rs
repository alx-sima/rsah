use crate::tabla::Tabla;

use super::{nebun, tura};

/// Genereaza o lista cu miscarile posibile (linie, coloana) pentru regina de la (i, j)
pub(super) fn get(tabla: &Tabla, i: i32, j: i32, full_scan: bool) -> Vec<(usize, usize)> {
    [
        nebun::get(tabla, i, j, full_scan),
        tura::get(tabla, i, j, full_scan),
    ]
    .concat()
}

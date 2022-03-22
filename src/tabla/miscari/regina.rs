use crate::tabla::{Tabla, PozitieVerificata};

use super::{nebun, tura};

/// Genereaza o lista cu miscarile posibile (linie, coloana) pentru regina de la (i, j)
pub(super) fn get(tabla: &Tabla, i: i32, j: i32, tot_ce_afecteaza: bool) -> Vec<PozitieVerificata> {
    [
        nebun::get(tabla, i, j, tot_ce_afecteaza),
        tura::get(tabla, i, j, tot_ce_afecteaza),
    ]
    .concat()
}

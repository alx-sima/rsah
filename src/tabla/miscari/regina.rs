use crate::tabla::{PozitieSafe, MatTabla};

use super::{nebun, tura};

/// Genereaza o lista cu miscarile posibile (linie, coloana) pentru regina de la (i, j)
pub(super) fn get(tabla: &MatTabla, poz: PozitieSafe, tot_ce_afecteaza: bool) -> Vec<PozitieSafe> {
    [
        nebun::get(tabla, poz, tot_ce_afecteaza),
        tura::get(tabla, poz, tot_ce_afecteaza),
    ]
    .concat()
}

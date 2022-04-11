use crate::tabla::{Pozitie, MatTabla};

use super::{nebun, tura};

/// Genereaza o lista cu miscarile posibile (linie, coloana) pentru regina de la (i, j)
pub(super) fn get(tabla: &MatTabla, poz: Pozitie, tot_ce_afecteaza: bool) -> Vec<Pozitie> {
    [
        nebun::get(tabla, poz, tot_ce_afecteaza),
        tura::get(tabla, poz, tot_ce_afecteaza),
    ]
    .concat()
}

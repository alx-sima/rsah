use crate::{
    tabla::{MatTabla, Pozitie},
    Mutare,
};

use super::{nebun, tura};

/// Genereaza o lista cu miscarile posibile (linie, coloana) pentru regina de la (i, j)
pub(super) fn get(tabla: &MatTabla, poz: Pozitie, check_all: bool) -> Vec<Mutare> {
    [
        nebun::get(tabla, poz, check_all),
        tura::get(tabla, poz, check_all),
    ]
    .concat()
}

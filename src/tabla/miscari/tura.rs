use crate::{
    tabla::{MatTabla, Pozitie},
    Mutare,
};

use super::cautare_in_linie;

/// Genereaza o lista cu miscarile posibile (linie, coloana) pentru tura de la (i, j)
pub(super) fn get(tabla: &MatTabla, poz: Pozitie, check_all: bool) -> Vec<Mutare> {
    cautare_in_linie(tabla, poz, &[(-1, 0), (0, 1), (1, 0), (0, -1)], check_all)
}

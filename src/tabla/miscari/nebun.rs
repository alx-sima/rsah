use crate::{
    tabla::{MatTabla, Pozitie},
    Mutare,
};

use super::cautare_in_linie;

/// Genereaza o lista cu mutarile
/// posibile pentru nebunul de la `poz`.
pub(super) fn get(tabla: &MatTabla, poz: Pozitie, tot_ce_afecteaza: bool) -> Vec<Mutare> {
    cautare_in_linie(
        tabla,
        poz,
        &[(-1, -1), (-1, 1), (1, 1), (1, -1)],
        tot_ce_afecteaza,
    )
}

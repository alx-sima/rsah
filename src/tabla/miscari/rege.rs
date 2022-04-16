use crate::tabla::{input::in_board, MatTabla, Pozitie, Tabla, TipPiesa};

use super::e_atacat;

/// Genereaza o lista cu miscarile posibile (linie, coloana) pentru regele de la (i, j)
pub(super) fn get(tabla: &MatTabla, i: i32, j: i32, tot_ce_afecteaza: bool) -> Vec<Pozitie> {
    let mut rez = vec![];
    let ui = i as usize;
    let uj = j as usize;

    for di in -1..=1 {
        for dj in -1..=1 {
            if in_board(i + di, j + dj) {
                let sumi = (i + di) as usize;
                let sumj = (j + dj) as usize;

                if tabla[sumi][sumj].piesa.is_some() {
                    if tabla[ui][uj].piesa.clone().unwrap().culoare
                        != tabla[sumi][sumj].piesa.clone().unwrap().culoare
                        || tot_ce_afecteaza
                    {
                        rez.push((sumi, sumj));
                    }
                } else {
                    rez.push((sumi, sumj));
                }
            }
        }
    }

    rez
}

// FIXME: verifica daca dupa rocada se intampla ceva nasty, gen sah
/// Se cauta pozitiile unde, daca este mutat regele, se face rocada.
pub(super) fn rocada(tabla: &Tabla, i: i32, j: i32) -> Vec<Pozitie> {
    let mut rez = vec![];
    let rege = tabla.at((i as usize, j as usize)).piesa.clone().unwrap();

    // Daca regele a fost mutat sau e in sah, rocada nu se poate face.
    if rege.mutat || e_atacat(tabla, (i as usize, j as usize), rege.culoare) {
        return rez;
    }

    // Cauta rocada intai in stanga, apoi in dreapta.
    for dir in [-1, 1] {
        // Distanta pana la tura (pt rocada mica, apoi mare)
        for dtura in [3, 4] {
            // Daca pozitia turei nu este in tabla, nu are sens cautarea rocadei
            if in_board(i, j + dir * dtura) {
                if let Some(tura) = tabla
                    .at((i as usize, (j + dir * dtura) as usize))
                    .piesa
                    .clone()
                {
                    if tura.culoare == rege.culoare && tura.tip == TipPiesa::Tura && !tura.mutat {
                        let mut ok = true;
                        for k in 1..dtura {
                            let destj = (j + dir * k) as usize;
                            if tabla.at((i as usize, destj)).piesa.is_some()
                                || e_atacat(tabla, (i as usize, destj), rege.culoare)
                            {
                                ok = false;
                                break;
                            }
                        }
                        if ok {
                            rez.push((i as usize, (j + dir * 2) as usize));
                            // Daca se gaseste rocada mica, e clar
                            // ca cea mare nu se poate face
                            break;
                        }
                    }
                }
            }
        }
    }
    rez
}

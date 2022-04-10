use crate::tabla::{input::in_board, MatTabla, PozitieSafe, TipPiesa};

/// Genereaza o lista cu miscarile posibile (linie, coloana) pentru regele de la (i, j)
pub(super) fn get(tabla: &MatTabla, i: i32, j: i32, tot_ce_afecteaza: bool) -> Vec<PozitieSafe> {
    let mut rez = Vec::new();
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

/// pozitiile unde, daca este mutat regele, se face rocada
pub(super) fn rocada(tabla: &MatTabla, i: i32, j: i32) -> Vec<PozitieSafe> {
    let mut rez = Vec::new();
    // Daca regele nu a fost mutat, urmatoarele 2 patratele in stanga/dreapta sunt libere
    // si neatacate si la inca 1/2 patratele se afla o tura, nemutata, se poate face rocada.
    if !tabla[i as usize][j as usize].piesa.clone().unwrap().mutat {
        // Directiile de cautare (stanga/dreapta)
        for dir in [-1, 1] {
            // Distanta pana la tura (pt rocada mica, respectiv mare)
            for dtura in [3, 4] {
                // Daca pozitia turei nu este in tabla, nu are sens cautarea rocadei
                if in_board(i, j + dir * dtura) {
                    if let Some(tura) = tabla[i as usize][(j + dir * dtura) as usize].piesa.clone()
                    {
                        if tura.culoare
                            == tabla[i as usize][j as usize].piesa.clone().unwrap().culoare
                            && tura.tip == TipPiesa::Tura
                            && !tura.mutat
                        {
                            let mut ok = true;
                            for k in 1..dtura {
                                // Explica asta daca poti
                                if tabla[i as usize][(j + dir * k) as usize].piesa.is_some()
                                // FIXME
                                    || !tabla[i as usize][(j + dir * k) as usize].atacat.is_empty()
                                {
                                    ok = false;
                                    break;
                                }
                            }
                            if ok {
                                rez.push((i as usize, (j + dir * 2) as usize));
                            }
                        }
                    }
                }
            }
        }
    }
    rez
}

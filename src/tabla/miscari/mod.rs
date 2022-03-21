use super::{game::muta, input::in_board, Culoare, Tabla, TipPiesa};

mod cal;
mod nebun;
mod pion;
mod rege;
mod regina;
mod tura;

/// Returneaza o lista cu pozitiile in care poate ajunge un patratel de la (i, j)
/// deplasat liniar, in functie de versori
// FIXME: nume mai bune pt parametru?
fn cautare_in_linie(
    tabla: &Tabla,
    i: i32,
    j: i32,
    versori: &[(i32, i32)],
    full_scan: bool,
) -> Vec<(usize, usize)> {
    let mut rez = Vec::new();

    for k in versori {
        let mut dist = 1;
        while in_board(i + k.0 * dist, j + k.1 * dist) {
            let sumi = (i + k.0 * dist) as usize;
            let sumj = (j + k.1 * dist) as usize;

            if let Some(patrat_atacat) = &tabla[sumi][sumj].piesa {
                if tabla[i as usize][j as usize].piesa.clone().unwrap().culoare
                    != patrat_atacat.culoare
                    || full_scan
                {
                    rez.push((sumi, sumj));
                }
                break;
            } else {
                rez.push((sumi, sumj));
            }
            dist += 1;
        }
    }
    rez
}

/// Returneaza o *lista[(linie, coloana)]* cu toate patratele in care se poate muta piesa de la *(i, j)*.
/// Daca *full_scan* e setat, se returneaza **toate** celulele ale carui update ar putea afecta piesa.
pub(crate) fn get_miscari(tabla: &Tabla, i: i32, j: i32, full_scan: bool) -> Vec<(usize, usize)> {
    if let Some(piesa) = &tabla[i as usize][j as usize].piesa {
        match piesa.tip {
            TipPiesa::Pion => pion::get(tabla, i, j, full_scan),
            TipPiesa::Tura => tura::get(tabla, i, j, full_scan),
            TipPiesa::Cal => cal::get(tabla, i, j, full_scan),
            TipPiesa::Nebun => nebun::get(tabla, i, j, full_scan),
            TipPiesa::Regina => regina::get(tabla, i, j, full_scan),
            TipPiesa::Rege => rege::get(tabla, i, j, full_scan),
        }
    // Daca patratul e gol, returneaza o lista goala
    } else {
        Vec::new()
    }
}

/// Verifica daca regele jucatorului *culoare* se afla in sah
/// (campul `atacat` de pe celula lui nu e nul pentru culoarea inamica)
pub(crate) fn verif_sah(tabla: &Tabla, culoare: Culoare) -> bool {
    let (i_rege, j_rege) = get_poz_rege(tabla, culoare);
    if tabla[i_rege][j_rege].atacat.iter().any(|(i, j)| {
        if let Some(piesa) = &tabla[*i][*j].piesa {
            piesa.culoare != culoare
        } else {
            false
        }
    }) {
        println!("{:?} este in sah!", culoare);
        return true;
    }
    false
}

/// Verifica daca jucatorul *culoare* mai are miscari disponibile.
/// IMPORTANT: functia **nu** verifica daca jucatorul este in sah.
pub(crate) fn exista_miscari(tabla: &Tabla, culoare: Culoare) -> bool {
    for i in 0..8 {
        for j in 0..8 {
            if let Some(piesa) = &tabla[i][j].piesa {
                if piesa.culoare == culoare {
                    let miscari = get_miscari(tabla, i as i32, j as i32, false);
                    let miscari = nu_provoaca_sah(tabla, miscari, (i, j), culoare);
                    if miscari.len() > 0 {
                        return true;
                    }
                }
            }
        }
    }
    return false;
}

// TODO:
pub(crate) fn nu_provoaca_sah(
    tabla: &Tabla,
    miscari: Vec<(usize, usize)>,
    piesa: (usize, usize),
    culoare: Culoare,
) -> Vec<(usize, usize)> {
    let mut rez = vec![];
    for (i, j) in miscari {
        // "Muta piesa pe pozitia de verificat, pentru a vedea daca pune regele in sah"
        let mut backup = tabla.clone();
        muta(&mut backup, (piesa.0, piesa.1), (i, j));
        let poz_rege = get_poz_rege(&backup, culoare);
        if !backup[poz_rege.0][poz_rege.1].atacat.iter().any(|(i, j)| {
            if let Some(piesa) = &backup[*i][*j].piesa {
                piesa.culoare != culoare
            } else {
                false
            }
        }) {
            rez.push((i, j));
        }
    }
    rez
}

/// Sterge din lista `atacat` piesa de pe pozitia (i, j),
/// pentru fiecare celula la care poate ajunge (i, j)
pub(crate) fn clear_attack(tabla: &mut Tabla, i: usize, j: usize) {
    if let Some(_) = &tabla[i][j].piesa {
        for (x, y) in get_miscari(&tabla, i as i32, j as i32, true) {
            // Sterge atacul piesei de pe celula (i, j)
            tabla[x][y].atacat.retain(|(a, b)| !(*a == i && *b == j));
        }
    }
}

pub(crate) fn get_poz_rege(tabla: &Tabla, culoare: Culoare) -> (usize, usize) {
    // FIXME: Se poate retine pozitia fiecarui rege, din moment ce exista cate unul singur.
    // Totusi, acest fapt este trivial si este lasat ca un exercitiu pentru cititor.
    for i in 0..8 {
        for j in 0..8 {
            if let Some(piesa) = &tabla[i][j].piesa {
                if piesa.tip == TipPiesa::Rege && piesa.culoare == culoare {
                    return (i, j);
                }
            }
        }
    }
    unreachable!();
}

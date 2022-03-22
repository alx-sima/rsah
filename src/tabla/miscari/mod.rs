use super::{game::muta, input::in_board, Culoare, PozitieVerificata, Tabla, TipPiesa};

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
    tot_ce_afecteaza: bool,
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
                    || tot_ce_afecteaza
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

/// Returneaza o *lista de (linie, coloana)* cu toate patratele in care se poate muta piesa de la *(i, j)*
/// (**include piesele pe care le ataca**).
/// Daca *tot_ce_afecteaza* e setat, se returneaza **toate** celulele ale caror modificare ar putea afecta piesa.
pub(crate) fn get_miscari(
    tabla: &Tabla,
    i: i32,
    j: i32,
    tot_ce_afecteaza: bool,
) -> Vec<PozitieVerificata> {
    if !in_board(i, j) {
        return Vec::new();
    }

    let i = i as usize;
    let j = j as usize;

    if let Some(piesa) = &tabla[i][j].piesa {
        match piesa.tip {
            // Pionul poate sa avanseze sau sa captureze pe diagonala
            TipPiesa::Pion => [
                pion::get(tabla, i as i32, j as i32, tot_ce_afecteaza),
                pion::ataca(tabla, (i, j)),
            ]
            .concat(),
            TipPiesa::Tura => tura::get(tabla, i as i32, j as i32, tot_ce_afecteaza),
            TipPiesa::Cal => cal::get(tabla, i as i32, j as i32, tot_ce_afecteaza),
            TipPiesa::Nebun => nebun::get(tabla, i as i32, j as i32, tot_ce_afecteaza),
            TipPiesa::Regina => regina::get(tabla, i as i32, j as i32, tot_ce_afecteaza),
            TipPiesa::Rege => [
                rege::rocada(tabla, i as i32, j as i32),
                rege::ataca(tabla, i as i32, j as i32, tot_ce_afecteaza),
            ]
            .concat(),
        }
    // Daca patratul e gol, returneaza o lista goala
    } else {
        Vec::new()
    }
}

/// Pentru piesa de la (i, j), daca exista, returneaza o lista cu toate pozitiile pe care le ataca.
pub(crate) fn get_atacat(tabla: &Tabla, i: i32, j: i32) -> Vec<PozitieVerificata> {
    if !in_board(i, j) {
        return Vec::new();
    }

    let i = i as usize;
    let j = j as usize;

    if let Some(piesa) = &tabla[i][j].piesa {
        match piesa.tip {
            TipPiesa::Pion => pion::ataca(tabla, (i, j)),
            // Regele ataca doar patratele din jurul lui,
            // iar get_miscari ar fi dat si pozitiile pentru rocada.
            TipPiesa::Rege => rege::ataca(tabla, i as i32, j as i32, false),
            // Restul pieselor ataca toate pozitiile pe care se pot misca.
            _ => get_miscari(tabla, i as i32, j as i32, false),
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
    tabla[i_rege][j_rege].atacat.len() > 0
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

/// Filtreaza vectorul miscari, ramanand doar cele care nu provoaca sah pentru regele propriu,
/// sau, daca acesta e deja in sah, doar cele care il scot.
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

/// Marcheaza celulele atacate de (i, j) si care afecteaza piesa.
pub(crate) fn set_influenta(tabla: &mut Tabla, i: usize, j: usize) {
    // Seteaza patratele care pot afecta piesa prin modificarea lor.
    for (x, y) in get_miscari(tabla, i as i32, j as i32, true) {
        tabla[x][y].afecteaza.push((i, j));
    }

    // Seteaza patratele care sunt atacate de piesa.
    for (x, y) in get_atacat(tabla, i as i32, j as i32) {
        tabla[x][y].atacat.push((i, j));
    }
}

/// Inversul la `set_influenta`.
pub(crate) fn clear_influenta(tabla: &mut Tabla, poz: PozitieVerificata) {
    // Sterge din lista de piese afectate a celulei (x, y) piesa.
    for (x, y) in get_miscari(&tabla, poz.0 as i32, poz.1 as i32, true) {
        tabla[x][y].afecteaza.retain(|k| *k != poz);
    }

    // Analog din lista de piese atacate.
    for (x, y) in get_atacat(&tabla, poz.0 as i32, poz.0 as i32) {
        tabla[x][y].afecteaza.retain(|k| *k != poz);
    }
}

/// Returneaza pozitia regelui de culoare *culoare*.
/// DEPRECATED: e doar pt ca a fost mai usor decat sa retinem pozitia regelui intr-un alt field.
pub(crate) fn get_poz_rege(tabla: &Tabla, culoare: Culoare) -> PozitieVerificata {
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

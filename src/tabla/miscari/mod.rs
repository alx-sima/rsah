use super::{game::muta, input::in_board, Culoare, MatTabla, Pozitie, Tabla, TipPiesa};

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
    tabla: &MatTabla,
    poz: Pozitie,
    versori: &[(i32, i32)],
    tot_ce_afecteaza: bool,
) -> Vec<Pozitie> {
    // FIXME
    let i = poz.0 as i32;
    let j = poz.1 as i32;

    let mut rez = vec![];

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
pub(crate) fn get_miscari(tabla: &Tabla, poz: Pozitie, tot_ce_afecteaza: bool) -> Vec<Pozitie> {
    let (i, j) = poz;

    if let Some(piesa) = &tabla.at((i, j)).piesa {
        match piesa.tip {
            // Pionul poate sa avanseze sau sa captureze pe diagonala
            TipPiesa::Pion => [
                pion::get(&tabla.mat, i as i32, j as i32, tot_ce_afecteaza),
                pion::ataca(tabla, (i, j), true),
            ]
            .concat(),
            TipPiesa::Tura => tura::get(&tabla.mat, poz, tot_ce_afecteaza),
            TipPiesa::Cal => cal::get(&tabla.mat, poz, tot_ce_afecteaza),
            TipPiesa::Nebun => nebun::get(&tabla.mat, poz, tot_ce_afecteaza),
            TipPiesa::Regina => regina::get(&tabla.mat, poz, tot_ce_afecteaza),
            TipPiesa::Rege => [
                rege::rocada(tabla, i as i32, j as i32),
                rege::get(&tabla.mat, i as i32, j as i32, tot_ce_afecteaza),
            ]
            .concat(),
        }
    // Daca patratul e gol, returneaza o lista goala
    } else {
        vec![]
    }
}

/// Pentru piesa de la (i, j) returneaza o lista cu toate pozitiile pe care le ataca.
/// Daca piesa nu exista, se returneaza un vector gol.
pub(crate) fn get_atacat(tabla: &Tabla, i: i32, j: i32) -> Vec<Pozitie> {
    if !in_board(i, j) {
        return vec![];
    }

    let i = i as usize;
    let j = j as usize;

    if let Some(piesa) = &tabla.at((i, j)).piesa {
        match piesa.tip {
            TipPiesa::Pion => pion::ataca(tabla, (i, j), false),
            // Regele ataca doar patratele din jurul lui,
            // iar get_miscari ar fi dat si pozitiile pentru rocada.
            TipPiesa::Rege => rege::get(&tabla.mat, i as i32, j as i32, false),
            // Restul pieselor ataca toate pozitiile pe care se pot misca.
            _ => get_miscari(tabla, (i, j), false),
        }
    // Daca patratul e gol, returneaza o lista goala
    } else {
        vec![]
    }
}

/// Verifica daca regele jucatorului *culoare* se afla in sah
pub(crate) fn verif_sah(tabla: &Tabla, culoare: Culoare) -> bool {
    let poz_rege = get_poz_rege(&tabla.mat, culoare);
    e_atacat(tabla, poz_rege, culoare)
}

/// Verifica daca patratul de pe `poz` este atacat de cealalta culoare.
fn e_atacat(tabla: &Tabla, poz: Pozitie, culoare: Culoare) -> bool {
    tabla.at(poz).atacat.iter().any(|x| {
        if let Some(piesa) = tabla.at(*x).piesa.clone() {
            return piesa.culoare != culoare;
        }
        unreachable!()
    })
}

/// Verifica daca jucatorul *culoare* mai are miscari disponibile.
/// IMPORTANT: functia **nu** verifica daca jucatorul este in sah.
pub(crate) fn exista_miscari(tabla: &Tabla, culoare: Culoare) -> bool {
    for i in 0..8 {
        for j in 0..8 {
            if let Some(piesa) = &tabla.at((i, j)).piesa {
                if piesa.culoare == culoare {
                    let miscari = get_miscari(tabla, (i, j), false);
                    let miscari = nu_provoaca_sah(tabla, miscari, (i, j), culoare);
                    if !miscari.is_empty() {
                        return true;
                    }
                }
            }
        }
    }

    false
}

/// Filtreaza vectorul miscari, ramanand doar cele care nu provoaca sah pentru regele propriu,
/// sau, daca acesta e deja in sah, doar cele care il scot.
pub(crate) fn nu_provoaca_sah(
    tabla: &Tabla,
    miscari: Vec<Pozitie>,
    piesa: Pozitie,
    culoare: Culoare,
) -> Vec<Pozitie> {
    let mut rez = vec![];

    for mutare in miscari {
        // "Muta piesa pe pozitia de verificat, pentru a vedea daca pune regele in sah"
        let mut backup = tabla.clone();
        muta(&mut backup, piesa, mutare);
        if !verif_sah(&backup, culoare) {
            rez.push(mutare);
        }
    }

    rez
}

/// Marcheaza celulele atacate de (i, j) si care afecteaza piesa.
pub(crate) fn set_influenta(tabla: &mut Tabla, poz: Pozitie) {
    // Seteaza patratele care pot fi afectate de mutarea piesei.
    for i in get_miscari(tabla, poz, true) {
        tabla.get(i).afecteaza.push(poz);
    }

    // Seteaza patratele care sunt atacate de piesa.
    for i in get_atacat(tabla, poz.0 as i32, poz.1 as i32) {
        tabla.get(i).atacat.push(poz);
    }
}

/// Inversul la `set_influenta`.
pub(crate) fn clear_influenta(tabla: &mut Tabla, poz: Pozitie) {
    // Sterge din lista de piese afectate a celulei (x, y) piesa.
    for i in get_miscari(tabla, poz, true) {
        tabla.get(i).afecteaza.retain(|k| *k != poz);
    }

    // Analog din lista de piese atacate.
    for i in get_atacat(tabla, poz.0 as i32, poz.1 as i32) {
        tabla.get(i).atacat.retain(|k| *k != poz);
    }
}

/// Returneaza pozitia regelui de culoare *culoare*.
/// DEPRECATED: e doar pt ca a fost mai usor decat sa retinem pozitia regelui intr-un alt field.
pub(crate) fn get_poz_rege(tabla: &MatTabla, culoare: Culoare) -> Pozitie {
    // FIXME: Se poate retine pozitia fiecarui rege, din moment ce exista cate unul singur.
    // Totusi, acest fapt este trivial si este lasat ca un exercitiu pentru cititor.
    for (i, line) in tabla.iter().enumerate().take(8) {
        for (j, patrat) in line.iter().enumerate().take(8) {
            if let Some(piesa) = &patrat.piesa {
                if piesa.tip == TipPiesa::Rege && piesa.culoare == culoare {
                    return (i, j);
                }
            }
        }
    }
    unreachable!();
}

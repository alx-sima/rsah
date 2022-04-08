use super::{game::muta, input::in_board, Culoare, PozitieSafe, Tabla, TipPiesa};

mod cal;
mod nebun;
pub(crate) mod pion;
mod rege;
mod regina;
mod tura;

/// Returneaza o lista cu pozitiile in care poate ajunge un patratel de la (i, j)
/// deplasat liniar, in functie de versori
// FIXME: nume mai bune pt parametru?
fn cautare_in_linie(
    tabla: &Tabla,
    poz: PozitieSafe,
    versori: &[(i32, i32)],
    tot_ce_afecteaza: bool,
) -> Vec<PozitieSafe> {
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
pub(crate) fn get_miscari(
    tabla: &Tabla,
    poz: PozitieSafe,
    tot_ce_afecteaza: bool,
) -> Vec<PozitieSafe> {
    let (i, j) = poz;

    if let Some(piesa) = &tabla[i][j].piesa {
        match piesa.tip {
            // Pionul poate sa avanseze sau sa captureze pe diagonala
            TipPiesa::Pion => [
                pion::get(tabla, i as i32, j as i32, tot_ce_afecteaza),
                pion::ataca(tabla, (i, j), true),
            ]
            .concat(),
            TipPiesa::Tura => tura::get(tabla, poz, tot_ce_afecteaza),
            TipPiesa::Cal => cal::get(tabla, poz, tot_ce_afecteaza),
            TipPiesa::Nebun => nebun::get(tabla, poz, tot_ce_afecteaza),
            TipPiesa::Regina => regina::get(tabla, poz, tot_ce_afecteaza),
            TipPiesa::Rege => [
                rege::rocada(tabla, i as i32, j as i32),
                rege::get(tabla, i as i32, j as i32, tot_ce_afecteaza),
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
pub(crate) fn get_atacat(tabla: &Tabla, i: i32, j: i32) -> Vec<PozitieSafe> {
    if !in_board(i, j) {
        return vec![];
    }

    let i = i as usize;
    let j = j as usize;

    if let Some(piesa) = &tabla[i][j].piesa {
        match piesa.tip {
            TipPiesa::Pion => pion::ataca(tabla, (i, j), false),
            // Regele ataca doar patratele din jurul lui,
            // iar get_miscari ar fi dat si pozitiile pentru rocada.
            TipPiesa::Rege => rege::get(tabla, i as i32, j as i32, false),
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
    let (i_rege, j_rege) = get_poz_rege(tabla, culoare);
    !regele_nu_e_in_sah(tabla, (i_rege, j_rege))
}

// FIXME: fff urat
fn regele_nu_e_in_sah(tabla: &Tabla, poz: PozitieSafe) -> bool {
    let culoare = tabla[poz.0][poz.1].piesa.clone().unwrap().culoare;
    !tabla[poz.0][poz.1]
        .atacat
        .clone()
        .into_iter()
        .any(|(i, j)| tabla[i][j].piesa.as_ref().unwrap().culoare != culoare)
}

/// Verifica daca jucatorul *culoare* mai are miscari disponibile.
/// IMPORTANT: functia **nu** verifica daca jucatorul este in sah.
pub(crate) fn exista_miscari(tabla: &Tabla, culoare: Culoare) -> bool {
    for i in 0..8 {
        for j in 0..8 {
            if let Some(piesa) = &tabla[i][j].piesa {
                if piesa.culoare == culoare {
                    let miscari = get_miscari(tabla, (i, j), false);
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
    miscari: Vec<PozitieSafe>,
    piesa: PozitieSafe,
    culoare: Culoare,
) -> Vec<PozitieSafe> {
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
pub(crate) fn set_influenta(tabla: &mut Tabla, poz: PozitieSafe) {
    // Seteaza patratele care pot fi afectate de mutarea piesei.
    for (x, y) in get_miscari(tabla, poz, true) {
        tabla[x][y].afecteaza.push(poz);
    }

    // Seteaza patratele care sunt atacate de piesa.
    for (x, y) in get_atacat(tabla, poz.0 as i32, poz.1 as i32) {
        tabla[x][y].atacat.push(poz);
    }
}

/// Inversul la `set_influenta`.
pub(crate) fn clear_influenta(tabla: &mut Tabla, poz: PozitieSafe) {
    // Sterge din lista de piese afectate a celulei (x, y) piesa.
    for (x, y) in get_miscari(&tabla, poz, true) {
        tabla[x][y].afecteaza.retain(|k| *k != poz);
    }

    // Analog din lista de piese atacate.
    for (x, y) in get_atacat(&tabla, poz.0 as i32, poz.1 as i32) {
        tabla[x][y].atacat.retain(|k| *k != poz);
    }
}

/// Returneaza pozitia regelui de culoare *culoare*.
/// DEPRECATED: e doar pt ca a fost mai usor decat sa retinem pozitia regelui intr-un alt field.
pub(crate) fn get_poz_rege(tabla: &Tabla, culoare: Culoare) -> PozitieSafe {
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
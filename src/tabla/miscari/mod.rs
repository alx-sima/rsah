use super::{
    game::muta, input::in_board, sah::in_sah, Culoare, MatTabla, Pozitie, Tabla, TipPiesa,
};

#[derive(Clone, Debug)]
pub(crate) struct Mutare {
    /// Pozitia pe care va ajunge piesa mutata.
    pub(crate) dest: Pozitie,
    /// In ce consta mutarea.
    pub(crate) tip: TipMutare,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum TipMutare {
    /// Mutare normala.
    Normal,
    /// O piesa este capturata.
    Captura,
    /// Miscarea presupune un en passant.
    /// Se retine pozitia **efectiva** a
    /// pionului care va fi capturat,
    /// nu destinatia, aceea va fi in `Mutare.dest`.
    EnPassant(Pozitie),
    /// Miscarea presupune o rocada.
    /// Se retine pozitia turei care participa.
    Rocada(Pozitie),
    /// Pionul este promovat in [TipPiesa].
    /// Fieldul e folosit doar pt decodarea
    /// notatiilor algebrice (pe multiplayer).
    Promovare(TipPiesa),
}

/// Returneaza o lista cu pozitiile in care poate ajunge un patratel de la (i, j)
/// deplasat liniar, in functie de versori
fn cautare_in_linie(
    tabla: &MatTabla,
    poz: Pozitie,
    versori: &[(i32, i32)],
    tot_ce_afecteaza: bool,
) -> Vec<Mutare> {
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
                    rez.push(Mutare {
                        tip: TipMutare::Captura,
                        dest: (sumi, sumj),
                    });
                }
                break;
            } else {
                rez.push(Mutare {
                    tip: TipMutare::Normal,
                    dest: (sumi, sumj),
                });
            }
            dist += 1;
        }
    }
    rez
}

/// Returneaza o `lista de (linie, coloana)` cu
/// toate patratele in care se poate muta `piesa`
/// (**include piesele pe care le ataca**).
///
/// Daca `tot_ce_afecteaza` e setat, se returneaza **toate**
/// celulele ale caror modificare ar putea afecta piesa.
pub(crate) fn get_miscari(tabla: &Tabla, piesa: Pozitie, tot_ce_afecteaza: bool) -> Vec<Mutare> {
    let (i, j) = piesa;

    if let Some(p) = &tabla.at((i, j)).piesa {
        match p.tip {
            // Pionul poate sa avanseze sau sa captureze pe diagonala
            TipPiesa::Pion => [
                pion::get(&tabla.mat, i as i32, j as i32, tot_ce_afecteaza),
                pion::ataca(tabla, (i, j), true),
            ]
            .concat(),
            TipPiesa::Tura => tura::get(&tabla.mat, piesa, tot_ce_afecteaza),
            TipPiesa::Cal => cal::get(&tabla.mat, piesa, tot_ce_afecteaza),
            TipPiesa::Nebun => nebun::get(&tabla.mat, piesa, tot_ce_afecteaza),
            TipPiesa::Regina => regina::get(&tabla.mat, piesa, tot_ce_afecteaza),
            TipPiesa::Rege => [
                rege::rocada(tabla, i, j),
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
pub(crate) fn get_atacat(tabla: &Tabla, i: i32, j: i32) -> Vec<Mutare> {
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

/// Verifica daca jucatorul `culoare` mai are miscari posibile,
/// **fara a verifica daca jucatorul este in sah**.
pub(crate) fn exista_miscari(tabla: &Tabla, culoare: Culoare) -> bool {
    for i in 0..8 {
        for j in 0..8 {
            if let Some(piesa) = &tabla.at((i, j)).piesa {
                if piesa.culoare == culoare {
                    let miscari = nu_provoaca_sah(tabla, (i, j), culoare);
                    if !miscari.is_empty() {
                        return true;
                    }
                }
            }
        }
    }

    false
}

/// Cauta doar miscarile care nu provoaca sah pentru regele propriu,
/// sau, daca acesta e deja in sah, doar pe cele care il scot.
pub(crate) fn nu_provoaca_sah(tabla: &Tabla, piesa: Pozitie, culoare: Culoare) -> Vec<Mutare> {
    let mut rez = vec![];

    for mutare in get_miscari(tabla, piesa, false) {
        // Muta piesa pe pozitia de verificat, pentru a vedea daca pune regele in sah.
        let mut dummy = tabla.clone();

        muta(&mut dummy, piesa, &mutare);
        if !in_sah(&dummy, culoare) {
            rez.push(mutare);
        }
    }

    rez
}

/// Marcheaza celulele atacate de (i, j) si care afecteaza piesa.
pub(crate) fn set_influenta(tabla: &mut Tabla, poz: Pozitie) {
    // Seteaza patratele care pot fi afectate de mutarea piesei.
    for i in get_miscari(tabla, poz, true) {
        tabla.get(i.dest).afecteaza.push(poz);
    }

    // Seteaza patratele care sunt atacate de piesa.
    for i in get_atacat(tabla, poz.0 as i32, poz.1 as i32) {
        tabla.get(i.dest).atacat.push(poz);
    }
}

/// Inversul functiei `set_influenta`.
pub(crate) fn clear_influenta(tabla: &mut Tabla, poz: Pozitie) {
    // Sterge din lista de piese afectate a celulei (x, y) piesa.
    for i in get_miscari(tabla, poz, true) {
        tabla.get(i.dest).afecteaza.retain(|k| *k != poz);
    }

    // Analog din lista de piese atacate.
    for i in get_atacat(tabla, poz.0 as i32, poz.1 as i32) {
        tabla.get(i.dest).atacat.retain(|k| *k != poz);
    }
}

/// Returneaza pozitia regelui de culoare *culoare*.
/// DEPRECATED: e doar pt ca a fost mai usor decat sa retinem pozitia regelui intr-un alt field.
pub(crate) fn get_poz_rege(tabla: &Tabla, culoare: Culoare) -> Pozitie {
    if culoare == Culoare::Alb {
        tabla.regi.0
    } else {
        tabla.regi.1
    }
}

mod cal;
mod nebun;
mod pion;
mod rege;
mod regina;
mod tura;

use super::{miscari, Culoare, MatchState, Pozitie, Tabla, TipPiesa};

/// Verifica daca jocul mai continua. Returneaza:
/// - `Some(Mat(Culoare))`: jocul s-a terminat, jucatorul e in mat;
/// - `Some(Pat)`: jocul s-a terminat cu egalitate;
/// - `None`: jocul continua.
pub(crate) fn verif_continua_jocul(tabla: &Tabla, turn: Culoare) -> Option<MatchState> {
    if !miscari::exista_miscari(tabla, turn) {
        // Daca e sah si nu exista miscari, e mat.
        if in_sah(tabla, turn) {
            return Some(MatchState::Mat(turn));
        }
        return Some(MatchState::Pat);
    }

    // Se verifica cele trei conditii de pat.
    if sunt_piese_destule(tabla) && !threefold(tabla) && capturi_in_ult_50(tabla) {
        None
    } else {
        Some(MatchState::Pat)
    }
}

/// Verifica daca regele jucatorului `culoare` se afla in sah
pub(crate) fn in_sah(tabla: &Tabla, culoare: Culoare) -> bool {
    let poz_rege = miscari::get_poz_rege(tabla, culoare);
    e_atacat(tabla, poz_rege, culoare)
}

/// Verifica daca patratul de pe `poz` este atacat de cealalta culoare.
pub(crate) fn e_atacat(tabla: &Tabla, poz: Pozitie, culoare: Culoare) -> bool {
    tabla.at(poz).atacat.iter().any(|x| {
        if let Some(piesa) = tabla.at(*x).piesa.clone() {
            return piesa.culoare != culoare;
        }
        unreachable!()
        // TODO: verif filmare 26.04 ora 1:26 am (unknown)
        // dupa promovare? cred ca stiu ce e
    })
}

/// Verifica sunt destule piese pentru a se putea da mat.
///  - Daca exista macar un pion, o tura sau o regina, se poate da mat.
///  - Daca a mai ramas doar un cal/nebun, e pat.
///  - Daca au mai ramas doar cate un nebun la fiecare jucator,
/// iar acestia sunt pe aceeasi culoare, e pat.
fn sunt_piese_destule(tabla: &Tabla) -> bool {
    let mut cai = vec![];
    let mut crazys = vec![];

    // Se numara piesele ramase.
    for i in 0..8 {
        for j in 0..8 {
            if let Some(piesa) = &tabla.at((i, j)).piesa {
                match piesa.tip {
                    // Daca se gasesc alte piese decat cal, nebun
                    // si rege, meciul poate fi terminat cu mat.
                    TipPiesa::Pion | TipPiesa::Regina | TipPiesa::Tura => {
                        return true;
                    }
                    TipPiesa::Cal => cai.push((i, j)),
                    TipPiesa::Nebun => crazys.push((i, j)),
                    TipPiesa::Rege => {}
                }
            }
        }
    }

    // Daca mai exista doar un cal/nebun, nu se poate da mat.
    if crazys.len() + cai.len() <= 1 {
        return false;
    }

    // Cazul in care fiecare jucator are doar cate un
    // nebun iar acestia se afla pe aceeasi culoare,
    // caz in care nu se poate da mat.
    if cai.is_empty() && crazys.len() == 2 {
        // Daca nebunii se afla pe culori diferite,
        // suma paritatilor coordonatelor va fi 1,
        // caz in care se poate da mat.
        return crazys.iter().map(|(i, j)| (i + j) % 2).sum::<usize>() == 1;
    }
    true
}

/// Verifica daca aceeasi pozitie a avut loc de 3 ori consecutiv.
fn threefold(tabla: &Tabla) -> bool {
    let istoric = &tabla.istoric;

    // Trebuie sa existe minimum 9 miscari pentru a avea 3 pozitii la fel consecutive
    if istoric.len() < 9 {
        return false;
    }
    let last = istoric.len() - 1;
    let mut ok = false;

    // Pozitia curenta trebuie sa se fi repetat de 2 ori pana acum
    if istoric[last] == istoric[last - 4] && istoric[last] == istoric[last - 8] {
        ok = true;
        // Pozitiile precedente au avut loc doar de 2 ori
        for i in 1..4 {
            if istoric[last - i] != istoric[last - i - 4] {
                ok = false;
            }
        }
    }

    ok
}

/// Verifica daca in ultimele 50 de miscari
/// (ale fiecarui jucator) au fost capturate piese.
///
/// TODO: testeaza
fn capturi_in_ult_50(tabla: &Tabla) -> bool {
    let istoric = &tabla.istoric;

    // Trebuie sa existe macar 100 de miscari
    if istoric.len() < 100 {
        return true;
    }

    // Verifica daca in ultimele 100 de miscari au fost capturate piese.
    istoric.iter().rev().take(100).any(|x| x.contains('x'))
}

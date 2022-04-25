use super::{miscari, Culoare, MatchState, Pozitie, Tabla, TipPiesa};

/// Verifica daca jocul mai continua. Returneaza:
/// - `Some(Mat(Culoare))`: jocul s-a terminat, jucatorul e in mat;
/// - `Some(Pat)`: jocul s-a terminat cu egalitate;
/// - `None`: jocul continua.
pub(crate) fn verif_continua_jocul(tabla: &Tabla, turn: Culoare) -> Option<MatchState> {
    if !miscari::exista_miscari(tabla, turn) {
        // Daca e sah si nu exista miscari, e mat.
        if verif_sah(tabla, turn) {
            return Some(MatchState::Mat(turn));
        }
        return Some(MatchState::Pat);
    }

    // Se verifica cele trei conditii de pat.
    if sunt_piese_destule(tabla) && capturi_in_ult_50(tabla) {
        //if sunt_piese_destule(tabla) && !threefold(tabla) &&  capturi_in_ult_50(tabla) {
        None
    } else {
        Some(MatchState::Pat)
    }
}

/// Verifica daca regele jucatorului `culoare` se afla in sah
pub(crate) fn verif_sah(tabla: &Tabla, culoare: Culoare) -> bool {
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
    })
}

/// Verifica sunt destule piese pentru a se putea da mat.
///  - Daca exista macar un pion, o tura sau o regina, se poate da mat.
///  - Daca a mai ramas doar un cal/nebun, e pat.
///  - Daca au mai ramas doar cate un nebun la fiecare jucator,
/// iar acestia sunt pe aceeasi culoare, e pat.
fn sunt_piese_destule(tabla: &Tabla) -> bool {
    let mut cai = vec![];
    let mut crazy = vec![];

    // Se numara piesele ramase.
    for i in 0..8 {
        for j in 0..8 {
            if let Some(piesa) = &tabla.at((i, j)).piesa {
                match piesa.tip {
                    // Daca se gasesc alte piese decat cal, nebun
                    // si rege, meciul poate fi terminat cu mat.
                    TipPiesa::Pion | TipPiesa::Regina | TipPiesa::Tura => {
                        println!("exista piese");
                        return true;
                    }
                    TipPiesa::Cal => cai.push((i, j)),
                    TipPiesa::Nebun => crazy.push((i, j)),
                    TipPiesa::Rege => {}
                }
            }
        }
    }

    // Daca mai exista doar un cal/nebun, nu se poate da mat.
    if crazy.len() + cai.len() <= 1 {
        return false;
    }

    // Cazul in care fiecare jucator are doar cate un
    // nebun iar acestia se afla pe aceeasi culoare,
    // caz in care nu se poate da mat.
    if cai.is_empty() && crazy.len() == 2 {
        /* Wall of shame: bamse
        if let Some(nebun1) = &tabla.at(crazy[0]).piesa {
            if let Some(nebun2) = &tabla.at(crazy[1]).piesa {
                if nebun1.culoare == nebun2.culoare {
                    return false;
                }
            }
        }
        */

        // TODO: Cursed?
        // Daca nebunii se afla pe culori diferite,
        // suma paritatilor coordonatelor va fi 1,
        // caz in care se poate da mat.
        return crazy.iter().map(|(i, j)| (i + j) % 2).sum::<usize>() == 1;
    }
    println!("sunt piese destule");
    true

    // TODO: cod vechi, verifica daca mai trebuie pastrat ceva
    /*
    let (
        mut cal_alb,
        mut cal_negru,
        mut nebun_alb_alb,
        mut nebun_alb_negru,
        mut nebun_negru_alb,
        mut nebun_negru_negru,
        mut ok,
    ) = (0, 0, 0, 0, 0, 0, true);
    // Cautam piesele ramase pe toata tabla.

    'check_piese: for i in 0..8 {
        for j in 0..8 {
            if let Some(piesa) = &tabla.at((i, j)).piesa {
                if piesa.tip == TipPiesa::Cal {
                    if piesa.culoare == Culoare::Alb {
                        cal_alb += 1;
                        if cal_alb > 1 {
                            ok = false;
                            break 'check_piese;
                        }
                    } else {
                        cal_negru += 1;
                        if cal_negru > 1 {
                            ok = false;
                            break 'check_piese;
                        }
                    }
                } else if piesa.tip == TipPiesa::Nebun {
                    if piesa.culoare == Culoare::Alb {
                        if (i + j) % 2 == 0 {
                            nebun_alb_alb += 1;
                            if nebun_alb_alb > 1 {
                                ok = false;
                                break 'check_piese;
                            }
                        } else {
                            nebun_alb_negru += 1;
                            if nebun_alb_negru > 1 {
                                ok = false;
                                break 'check_piese;
                            }
                        }
                    } else if (i + j) % 2 == 0 {
                        nebun_negru_alb += 1;
                        if nebun_negru_alb > 1 {
                            ok = false;
                            break 'check_piese;
                        }
                    } else {
                        nebun_negru_negru += 1;
                        if nebun_negru_negru > 1 {
                            ok = false;
                            break 'check_piese;
                        }
                    }
                } else if piesa.tip != TipPiesa::Rege {
                    ok = false;
                    break 'check_piese;
                }
            }
        }
    }

    // Verificam daca exista unul din cazurile de pat (rege + cal vs rege;    rege + nebun vs rege;    rege + nebun vs rege + nebun unde nebunii sunt pe patrat de acelasi culoare)
    if ok {
        // Cal + nicio alta piesa
        if cal_alb == 1
            && (cal_negru == 1
                || nebun_alb_alb == 1
                || nebun_alb_negru == 1
                || nebun_negru_alb == 1
                || nebun_negru_negru == 1)
        {
            ok = false;
        }
        if cal_negru == 1
            && (cal_alb == 1
                || nebun_alb_alb == 1
                || nebun_alb_negru == 1
                || nebun_negru_alb == 1
                || nebun_negru_negru == 1)
        {
            ok = false;
        }

        // Nebun + nicio alta piesa / nebun pe patrat de aceeasi culoare
        if nebun_alb_alb == 1
            && (cal_alb == 1 || cal_negru == 1 || nebun_alb_negru == 1 || nebun_negru_negru == 1)
        {
            ok = false;
        }
        if nebun_alb_negru == 1
            && (cal_alb == 1 || cal_negru == 1 || nebun_alb_alb == 1 || nebun_negru_alb == 1)
        {
            ok = false;
        }
        if nebun_negru_alb == 1
            && (cal_alb == 1 || cal_negru == 1 || nebun_alb_negru == 1 || nebun_negru_negru == 1)
        {
            ok = false;
        }
        if nebun_negru_negru == 1
            && (cal_alb == 1 || cal_negru == 1 || nebun_alb_alb == 1 || nebun_negru_alb == 1)
        {
            ok = false;
        }
    }
    !ok
    */
}

/// Verifica daca aceeasi pozitie a avut loc de 3 ori consecutiv.
fn threefold(tabla: &Tabla) -> bool {
    let istoric = &tabla.istoric;

    // Trebuie sa existe minimum 9 miscari pentru a avea 3 pozitii la fel consecutive
    if istoric.len() < 9 {
        return false;
    }
    let last = istoric.len() - 1;
    let mut ok = true;

    // Pozitia curenta trebuie sa se fi repetat de 2 ori pana acum
    if istoric[last] == istoric[last - 4] && istoric[last] == istoric[last - 8] {
        ok = false;
        // Pozitiile precedente au avut loc doar de 2 ori
        for i in 1..4 {
            if istoric[last - i] != istoric[last - i - 4] {
                ok = true;
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
    println!(
        "{:?}",
        istoric.iter().rev().take(100).any(|x| x.contains('x'))
    );

    // Verifica daca in ultimele 100 de miscari au fost capturate piese.
    istoric.iter().rev().take(100).any(|x| x.contains('x'))
}

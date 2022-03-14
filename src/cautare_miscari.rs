use crate::tabla::{in_board, Culoare, Tabla, TipPiesa};

/// Genereaza o lista cu miscarile posibile (linie, coloana) pentru regele de la (i, j)
fn rege(tabla: &Tabla, i: i32, j: i32, full_scan: bool) -> Vec<(usize, usize)> {
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
                        || full_scan
                    {
                        rez.push((sumi, sumj));
                    }
                } else {
                    rez.push((sumi, sumj));
                }
            }
        }
    }

    // Daca regele nu a fost mutat, urmatoarele 2 patratele in stanga/dreapta sunt libere
    // si neatacate si la inca 1/2 patratele se afla o tura, nemutata, se poate face rocada.
    if !tabla[ui][uj].piesa.clone().unwrap().mutat {
        // Directiile de cautare (stanga/dreapta)
        for dir in [-1, 1] {
            // Distanta pana la tura (pt rocada mica, respectiv mare)
            for dtura in [3, 4] {
                // Daca pozitia turei nu este in tabla, nu are sens cautarea rocadei
                if in_board(i, j + dir * dtura) {
                    if let Some(tura) = tabla[ui][(j + dir * dtura) as usize].piesa.clone() {
                        if tura.culoare == tabla[ui][uj].piesa.clone().unwrap().culoare
                            && tura.tip == TipPiesa::Tura
                            && !tura.mutat
                        {
                            let mut ok = true;
                            for k in 1..dtura {
                                // Explica asta daca poti
                                if tabla[ui][(j + dir * k) as usize].piesa.is_some()
                                    || tabla[ui][(j + dir * k) as usize]
                                        .atacat
                                        .clone()
                                        .into_iter()
                                        .any(|(x, y)| {
                                            tabla[x][y].piesa.clone().unwrap().culoare
                                                != tura.culoare
                                        })
                                {
                                    ok = false;
                                    break;
                                }
                            }
                            if ok {
                                rez.push((ui, (j + dir * 2) as usize));
                            }
                        }
                    }
                }
            }
        }
    }

    rez
}

/// Genereaza o lista cu miscarile posibile (linie, coloana) pentru nebunul de la (i, j)
fn nebun(tabla: &Tabla, i: i32, j: i32, full_scan: bool) -> Vec<(usize, usize)> {
    cautare_in_linie(
        tabla,
        i,
        j,
        &[(-1, -1), (-1, 1), (1, 1), (1, -1)],
        full_scan,
    )
}

/// Genereaza o lista cu miscarile posibile (linie, coloana) pentru tura de la (i, j)
fn tura(tabla: &Tabla, i: i32, j: i32, full_scan: bool) -> Vec<(usize, usize)> {
    cautare_in_linie(tabla, i, j, &[(-1, 0), (0, 1), (1, 0), (0, -1)], full_scan)
}

/// Genereaza o lista cu miscarile posibile (linie, coloana) pentru regina de la (i, j)
fn regina(tabla: &Tabla, i: i32, j: i32, full_scan: bool) -> Vec<(usize, usize)> {
    [nebun(tabla, i, j, full_scan), tura(tabla, i, j, full_scan)].concat()
}

/// Genereaza o lista cu miscarile posibile (linie, coloana) pentru calul de la (i, j)
fn cal(tabla: &Tabla, i: i32, j: i32, full_scan: bool) -> Vec<(usize, usize)> {
    let mut rez = Vec::new();
    let ui = i as usize;
    let uj = j as usize;

    // folosim vectorii de pozitie dx si dy pentru a genera toate miscarile posibile in forma de L
    // (2 patrate pe orizontala, 1 patrate pe verticala / 2 patrate pe verticala si 1 patrate pe orizontala)
    let di = [-2, -2, -1, -1, 1, 1, 2, 2];
    let dj = [1, -1, 2, -2, 2, -2, 1, -1];

    for k in 0..8 {
        if in_board(i + di[k], j + dj[k]) {
            let sumi = (i + di[k]) as usize;
            let sumj = (j + dj[k]) as usize;

            if tabla[sumi][sumj].piesa.is_some() {
                if tabla[ui][uj].piesa.clone().unwrap().culoare
                    != tabla[sumi][sumj].piesa.clone().unwrap().culoare
                    || full_scan
                {
                    rez.push((sumi, sumj));
                }
            } else {
                rez.push((sumi, sumj));
            }
        }
    }
    rez
}

/// Genereaza o lista cu miscarile posibile (linie, coloana) pentru pionul de la (i, j)
// TODO: repara mismatch isize-usize pt readability
fn pion(tabla: &Tabla, i: i32, j: i32, full_scan: bool) -> Vec<(usize, usize)> {
    let mut rez = Vec::new();
    let i = i as usize;
    let j = j as usize;

    // Memoreaza directia in care se misca pionul in functie de culoare:
    // alb se misca in sus => scade randul in matrice;
    // negru se misca in jos => creste randul in matrice.
    // TODO: scapa de unwrap
    let cul = if tabla[i][j].piesa.clone().unwrap().culoare == Culoare::Alb {
        -1
    } else {
        1
    };

    // Randul din fata pionului
    let i1 = (i as i32 + cul) as usize;
    // Daca patratul din fata pionului e liber, miscarea e valabila
    if tabla[i1][j].piesa.is_none() || full_scan {
        rez.push((i1, j));

        // Daca urmatoarele 2 patrate din fata pionului sunt libere,
        // iar pionul nu a fost deja mutat, acesta poate avansa 2 patrate.
        let i2 = (i1 as i32 + cul) as usize;
        if in_board(i2 as i32, j as i32) {
            if !tabla[i][j].piesa.clone().unwrap().mutat
                && (tabla[i2 as usize][j].piesa.is_none() || full_scan)
            {
                rez.push((i2, j));
            }
        }
    }

    // Daca patratele din fata-stanga sau fata-dreapta sunt ocupate de o piesa inamica, miscarea e valabila
    for j1 in [j - 1, j + 1] {
        if in_board(i1 as i32, j1 as i32) {
            if let Some(victima) = &tabla[i1][j1].piesa {
                if victima.culoare != tabla[i as usize][j as usize].piesa.clone().unwrap().culoare
                    || full_scan
                {
                    rez.push((i1, j1));
                }
            } else if full_scan {
                rez.push((i1, j1));
            }
        }
    }

    // TODO: adaugare en passant (adaugare istoric miscari -> verificam daca ultima miscare permite en passant)
    rez
}

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
            TipPiesa::Pion => pion(tabla, i, j, full_scan),
            TipPiesa::Tura => tura(tabla, i, j, full_scan),
            TipPiesa::Cal => cal(tabla, i, j, full_scan),
            TipPiesa::Nebun => nebun(tabla, i, j, full_scan),
            TipPiesa::Regina => regina(tabla, i, j, full_scan),
            TipPiesa::Rege => rege(tabla, i, j, full_scan),
        }
    // Daca patratul e gol, returneaza o lista goala
    } else {
        Vec::new()
    }
}

/// Printeaza (deocamdata) daca vreun rege se afla in sah
/// (campul `atacat` de pe celula lui nu e nul pentru culoarea inamica)
pub(crate) fn verif_sah(tabla: &Tabla) {
    for i in 0..8 {
        for j in 0..8 {
            if let Some(piesa) = &tabla[i][j].piesa {
                if piesa.tip == TipPiesa::Rege {
                    for i in tabla[i][j].atacat.iter() {
                        if let Some(agresor) = &tabla[i.0 as usize][i.1 as usize].piesa.clone() {
                            if agresor.culoare != piesa.culoare {
                                println!("Regele {:?} se afla in sah!", piesa.culoare);
                            }
                        }
                    }
                }
            }
        }
    }
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

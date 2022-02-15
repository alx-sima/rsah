use crate::{t::in_board, Culoare, Patratel, Piesa};

/// Genereaza o lista cu miscarile posibile (linie, coloana) pentru regele de la (i, j)
fn rege(tabla: &[[Option<Patratel>; 8]; 8], i: i32, j: i32) -> Vec<(usize, usize)> {
    let mut rez = Vec::new();
    let ui = i as usize;
    let uj = j as usize;

    for di in -1..=1 {
        for dj in -1..=1 {
            if in_board(i + di, j + dj) {
                let sumi = ui + di as usize;
                let sumj = uj + dj as usize;

                if tabla[sumi][sumj].is_some() {
                    if tabla[ui][uj].unwrap().culoare != tabla[sumi][sumj].unwrap().culoare {
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

/// Genereaza o lista cu miscarile posibile (linie, coloana) pentru nebunul de la (i, j)
fn nebun(tabla: &[[Option<Patratel>; 8]; 8], i: i32, j: i32) -> Vec<(usize, usize)> {
    cautare_in_linie(tabla, i, j, &[(-1, -1), (-1, 1), (1, 1), (1, -1)])
}

/// Genereaza o lista cu miscarile posibile (linie, coloana) pentru tura de la (i, j)
fn tura(tabla: &[[Option<Patratel>; 8]; 8], i: i32, j: i32) -> Vec<(usize, usize)> {
    cautare_in_linie(tabla, i, j, &[(-1, 0), (0, 1), (1, 0), (0, -1)])
}

/// Genereaza o lista cu miscarile posibile (linie, coloana) pentru regina de la (i, j)
fn regina(tabla: &[[Option<Patratel>; 8]; 8], i: i32, j: i32) -> Vec<(usize, usize)> {
    [nebun(tabla, i, j), tura(tabla, i, j)].concat()
}

/// Genereaza o lista cu miscarile posibile (linie, coloana) pentru calul de la (i, j)
fn cal(tabla: &[[Option<Patratel>; 8]; 8], i: i32, j: i32) -> Vec<(usize, usize)> {
    let mut rez = Vec::new();
    let ui = i as usize;
    let uj = j as usize;
    
    // folosim vectorii de pozitie dx si dy pentru a genera toate miscarile posibile in forma de L
    // (2 patrate pe orizontala, 1 patrate pe verticala / 2 patrate pe verticala si 1 patrate pe orizontala)
    let di = [-2, -2, -1, -1, 1, 1, 2, 2];
    let dj = [1, -1, 2, -2, 2, -2, 1, -1];

    for k in 0..8 {
        if in_board(i + di[k], j + dj[k]) {
            let sumi = ui + di[k] as usize;
            let sumj = uj + dj[k] as usize;

            if tabla[sumi][sumj].is_some() {
                if tabla[ui][uj].unwrap().culoare != tabla[sumi][sumj].unwrap().culoare {
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
fn pion(tabla: &[[Option<Patratel>; 8]; 8], i: i32, j: i32) -> Vec<(usize, usize)> {
    let mut rez = Vec::new();
    let i = i as usize;
    let j = j as usize;

    // Memoreaza directia in care se misca pionul in functie de culoare:
    // alb se misca in sus => scade randul in matrice;
    // negru se misca in jos => creste randul in matrice.
    // TODO: scapa de unwrap
    let cul: i32 = if tabla[i][j].unwrap().culoare == Culoare::Alb {
        -1
    } else {
        1
    };

    // Randul din fata pionului
    let i1 = i + cul as usize;
    // daca patratul din fata pionului e liber, miscarea e valabila
    if tabla[i1][j].is_none() {
        rez.push((i1, j));

        // daca urmatoarele 2 patrate din fata pionului sunt libere, iar pionul este la prima miscare, acesta poate avansa 2 patrate
        // TODO: fix lazy programming (daca se afla pe primele 2 randuri poate avansa 2 patrate) => doar la prima miscare a pionului poate avansa 2 patrate
        let i2 = i1 + cul as usize;
        if in_board(i2 as i32, j as i32) {
            if ((i <= 1 && cul == 1) || (i >= 6 && cul == -1)) && tabla[i2 as usize][j].is_none() {
                rez.push((i2, j));
            }
        }
    }
    // TODO: urmatoarele 2 if-uri sunt redundante, se poate face cu un for cred?
    // daca patratul din fata-stanga e ocupat de o piesa inamica, miscarea e valabila
    if in_board(i1 as i32, (j - 1) as i32) {
        if let Some(victima) = tabla[i1][(j - 1) as usize] {
            if victima.culoare != tabla[i as usize][j as usize].unwrap().culoare {
                rez.push((i1, j - 1));
            }
        }
    }
    // daca patratul din fata-dreapta e ocupat de o piesa inamica, miscarea e valabila
    if in_board(i1 as i32, (j + 1) as i32) {
        if let Some(victima) = tabla[i1 as usize][(j + 1) as usize] {
            if victima.culoare != tabla[i as usize][j as usize].unwrap().culoare {
                rez.push((i1, j + 1));
            }
        }
    }
    // TODO: adaugare en passant (adaugare istoric miscari -> verificam daca ultima miscare permite en passant)
    rez
}

fn cautare_in_linie(
    tabla: &[[Option<Patratel>; 8]; 8],
    i: i32,
    j: i32,
    versori: &[(i32, i32)],
) -> Vec<(usize, usize)> {
    let mut rez = Vec::new();

    for k in versori {
        let mut dist = 1;
        while in_board(i + k.0 * dist, j + k.1 * dist) {
            let sumi = (i + k.0 * dist) as usize;
            let sumj = (j + k.1 * dist) as usize;

            if let Some(patrat_atacat) = tabla[sumi][sumj] {
                if tabla[i as usize][j as usize].unwrap().culoare != patrat_atacat.culoare {
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

pub(crate) fn get_miscari(
    tabla: &[[Option<Patratel>; 8]; 8],
    piesa: Piesa,
    i: i32,
    j: i32,
) -> Vec<(usize, usize)> {
    match piesa {
        Piesa::Pion => pion(tabla, i, j),
        Piesa::Tura => tura(tabla, i, j),
        Piesa::Cal => cal(tabla, i, j),
        Piesa::Nebun => nebun(tabla, i, j),
        Piesa::Regina => regina(tabla, i, j),
        Piesa::Rege => rege(tabla, i, j),
    }
}

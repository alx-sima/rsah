use crate::{tabla::{in_board, Culoare, Patratel, TipPiesa}};

/// Genereaza o lista cu miscarile posibile (linie, coloana) pentru regele de la (i, j)
fn rege(tabla: &[[Patratel; 8]; 8], i: i32, j: i32) -> Vec<(usize, usize)> {
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

/// Genereaza o lista cu miscarile posibile (linie, coloana) pentru nebunul de la (i, j)
fn nebun(tabla: &[[Patratel; 8]; 8], i: i32, j: i32, a_ales_violenta: bool) -> Vec<(usize, usize)> {
    cautare_in_linie(
        tabla,
        i,
        j,
        &[(-1, -1), (-1, 1), (1, 1), (1, -1)],
        a_ales_violenta,
    )
}

/// Genereaza o lista cu miscarile posibile (linie, coloana) pentru tura de la (i, j)
fn tura(tabla: &[[Patratel; 8]; 8], i: i32, j: i32, a_ales_violenta: bool) -> Vec<(usize, usize)> {
    cautare_in_linie(
        tabla,
        i,
        j,
        &[(-1, 0), (0, 1), (1, 0), (0, -1)],
        a_ales_violenta,
    )
}

/// Genereaza o lista cu miscarile posibile (linie, coloana) pentru regina de la (i, j)
fn regina(
    tabla: &[[Patratel; 8]; 8],
    i: i32,
    j: i32,
    a_ales_violenta: bool,
) -> Vec<(usize, usize)> {
    [
        nebun(tabla, i, j, a_ales_violenta),
        tura(tabla, i, j, a_ales_violenta),
    ]
    .concat()
}

/// Genereaza o lista cu miscarile posibile (linie, coloana) pentru calul de la (i, j)
fn cal(tabla: &[[Patratel; 8]; 8], i: i32, j: i32) -> Vec<(usize, usize)> {
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
fn pion(tabla: &[[Patratel; 8]; 8], i: i32, j: i32) -> Vec<(usize, usize)> {
    let mut rez = Vec::new();
    let i = i as usize;
    let j = j as usize;

    // Memoreaza directia in care se misca pionul in functie de culoare:
    // alb se misca in sus => scade randul in matrice;
    // negru se misca in jos => creste randul in matrice.
    // TODO: scapa de unwrap
    let cul: i32 = if tabla[i][j].piesa.clone().unwrap().culoare == Culoare::Alb {
        -1
    } else {
        1
    };

    // Randul din fata pionului
    let i1 = i + cul as usize;
    // daca patratul din fata pionului e liber, miscarea e valabila
    if tabla[i1][j].piesa.is_none() {
        rez.push((i1, j));

        // daca urmatoarele 2 patrate din fata pionului sunt libere, iar pionul este la prima miscare, acesta poate avansa 2 patrate
        // TODO: fix lazy programming (daca se afla pe primele 2 randuri poate avansa 2 patrate) => doar la prima miscare a pionului poate avansa 2 patrate
        let i2 = i1 + cul as usize;
        if in_board(i2 as i32, j as i32) {
            if ((i <= 1 && cul == 1) || (i >= 6 && cul == -1))
                && tabla[i2 as usize][j].piesa.is_none()
            {
                rez.push((i2, j));
            }
        }
    }
    // TODO: urmatoarele 2 if-uri sunt redundante, se poate face cu un for cred?
    // daca patratul din fata-stanga e ocupat de o piesa inamica, miscarea e valabila
    if in_board(i1 as i32, (j - 1) as i32) {
        if let Some(victima) = &tabla[i1][(j - 1) as usize].piesa {
            if victima.culoare != tabla[i as usize][j as usize].piesa.clone().unwrap().culoare {
                rez.push((i1, j - 1));
            }
        }
    }
    // daca patratul din fata-dreapta e ocupat de o piesa inamica, miscarea e valabila
    if in_board(i1 as i32, (j + 1) as i32) {
        if let Some(victima) = &tabla[i1 as usize][(j + 1) as usize].piesa {
            if victima.culoare != tabla[i as usize][j as usize].piesa.clone().unwrap().culoare {
                rez.push((i1, j + 1));
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
    tabla: &[[Patratel; 8]; 8],
    i: i32,
    j: i32,
    versori: &[(i32, i32)],
    a_ales_violenta: bool,
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
                    || a_ales_violenta
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

/// Returneaza o lista (linie, coloana) cu toate patratele in care se poate muta piesa de la (i, j)
pub(crate) fn get_miscari(
    tabla: &[[Patratel; 8]; 8],
    i: i32,
    j: i32,
    a_ales_violenta: bool,
) -> Vec<(usize, usize)> {
    if let Some(piesa) = &tabla[i as usize][j as usize].piesa {
        match piesa.tip {
            TipPiesa::Pion => pion(tabla, i, j),
            TipPiesa::Tura => tura(tabla, i, j, a_ales_violenta),
            TipPiesa::Cal => cal(tabla, i, j),
            TipPiesa::Nebun => nebun(tabla, i, j, a_ales_violenta),
            TipPiesa::Regina => regina(tabla, i, j, a_ales_violenta),
            TipPiesa::Rege => rege(tabla, i, j),
        }
    // Daca patratul e gol, returneaza o lista goala
    } else {
        Vec::new()
    }
}

/// Printeaza (deocamdata) daca vreun rege se afla in sah
/// (campul `atacat` de pe celula lui nu e nul pentru culoarea inamica)
pub(crate) fn verif_sah(tabla: &[[Patratel; 8]; 8]) {
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
pub(crate) fn clear_attack(tabla: &mut [[Patratel; 8]; 8], i: i32, j: i32) {
    if let Some(_) = &tabla[i as usize][j as usize].piesa {
        for (x, y) in get_miscari(&tabla, i, j, true) {
            // Sterge atacul piesei de pe celula (i, j)
            tabla[x][y].atacat.retain(|(a, b)| !(*a == i && *b == j));
        }
    }
}

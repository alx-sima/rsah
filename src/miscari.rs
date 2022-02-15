use crate::{Patratel, t::in_board, Culoare};

fn miscari_rege(tabla: &mut [[Option<Patratel>; 8]; 8], x: i32, y: i32) -> Vec<(i32, i32)> {
    let mut rez = Vec::new();

    for i in -1..=1 {
        for j in -1..=1 {
            if in_board(x + i, y + j) {
                rez.push((x + i, y + j));
            }
        }
    }
    rez
}

fn miscari_nebun(tabla: &mut [[Option<Patratel>; 8]; 8], x: i32, y: i32) -> Vec<(i32, i32)> {
    let mut rez = Vec::new();

    for i in -7..=7 {
        if in_board(x + i, y + i) {
            rez.push((x + i, y + i));
        }
        if in_board(x + i, y - i) {
            rez.push((x + i, y - i));
        }
    }
    rez
}

fn miscari_tura(tabla: &mut [[Option<Patratel>; 8]; 8], x: i32, y: i32) -> Vec<(i32, i32)> {
    let mut rez = Vec::new();
    for i in -7..=7 {
        if in_board(x + i, y) {
            rez.push((x + i, y));
        }
        if in_board(x, y + i) {
            rez.push((x, y + i));
        }
    }
    rez
}

fn miscari_regina(tabla: &mut [[Option<Patratel>; 8]; 8], x: i32, y: i32) -> Vec<(i32, i32)> {
    [miscari_nebun(tabla, x, y), miscari_rege(tabla, x, y)].concat()
}

fn miscari_cal(tabla: &mut [[Option<Patratel>; 8]; 8], x: i32, y: i32) -> Vec<(i32, i32)> {
    let mut rez = Vec::new();
    // folosim vectorii de pozitie dx si dy pentru a genera toate miscarile posibile in forma de L(2 patrate pe orizontala, 1 patrate pe verticala / 2 patrate pe verticala si 1 patrate pe orizontala)
    let dx = [-2, -2, -1, -1, 1, 1, 2, 2];
    let dy = [1, -1, 2, -2, 2, -2, 1, -1];

    for k in 0..8 {
        if in_board(x + dx[k], y + dy[k]) {
            rez.push((x + dx[k], y + dy[k]));
        }
    }
    rez
}

// TODO: repara mismatch isize-usize pt readability
fn miscari_pion(tabla: &mut [[Option<Patratel>; 8]; 8], x: isize, y: isize) -> Vec<(isize, isize)> {
    let mut rez = Vec::new();

    // cul memoreaza directia in care se misca pionul in functie de culoare (alb se misca in sus => scade randul in matrice / negru se misca in jos => creste randul in matrice)
    // TODO: scapa de unwrap
    let cul: isize = if tabla[x as usize][y as usize].unwrap().culoare == Culoare::Alb {
        -1
    } else {
        1
    };

    // daca patratul din fata pionului e liber, miscarea e valabila
    if tabla[(x + cul) as usize][y as usize].is_none() {
        rez.push((x as isize + cul, y));
        
        // daca urmatoarele 2 patrate din fata pionului sunt libere, iar pionul este la prima miscare, acesta poate avansa 2 patrate
        // TODO: fix lazy programming (daca se afla pe primele 2 randuri poate avansa 2 patrate) => doar la prima miscare a pionului poate avansa 2 patrate
        if in_board((x + 2 * cul) as i32, y as i32) {
            if ((x <= 1 && cul == 1) || (x >= 6 && cul == -1)) && tabla[(x + 2 * cul) as usize][y as usize].is_none() {
                rez.push((x + 2 * cul, y));
            }
        }
    }
    // TODO: urmatoarele 2 if-uri sunt redundante, se poate face cu un for cred?
    // daca patratul din fata-stanga e ocupat de o piesa inamica, miscarea e valabila
    if in_board((x + cul) as i32, (y - 1) as i32) {
        if let Some(victima) = tabla[(x+cul) as usize][(y - 1) as usize] {
            if victima.culoare != tabla[x as usize][y as usize].unwrap().culoare {
                rez.push((x + cul, y - 1));
            }
        }
    }
    // daca patratul din fata-dreapta e ocupat de o piesa inamica, miscarea e valabila
    if in_board((x + cul) as i32, (y + 1) as i32) {
        if let Some(victima) = tabla[(x+cul) as usize][(y + 1) as usize] {
            if victima.culoare != tabla[x as usize][y as usize].unwrap().culoare {
                rez.push((x + cul, y + 1));
            }
        }
    }
    //TODO: adaugare en passant (adaugare istoric miscari -> verificam daca ultima miscare permite en passant)
    rez
}
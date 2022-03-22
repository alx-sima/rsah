use crate::tabla::{input::in_board, Culoare, PozitieVerificata, Tabla};

/// Genereaza o lista cu miscarile posibile (linie, coloana) pentru pionul de la (i, j)
// TODO: repara mismatch isize-usize pt readability
pub(super) fn get(tabla: &Tabla, i: i32, j: i32, tot_ce_afecteaza: bool) -> Vec<PozitieVerificata> {
    let mut rez = Vec::new();
    let i = i as usize;
    let j = j as usize;

    let culoare = tabla[i][j].piesa.clone().unwrap().culoare;

    // Memoreaza directia in care se misca pionul in functie de culoare:
    // alb se misca in sus => scade randul in matrice;
    // negru se misca in jos => creste randul in matrice.
    // TODO: scapa de unwrap
    let dir = if culoare == Culoare::Alb { -1 } else { 1 };
    // Randul din fata pionului
    let i1 = (i as i32 + dir) as usize;

    // Daca patratul din fata pionului exista si e liber, miscarea e valabila.
    if in_board(i1 as i32, j as i32) && (tabla[i1][j].piesa.is_none() || tot_ce_afecteaza) {
        rez.push((i1, j));

        // Daca urmatoarele 2 patrate din fata pionului sunt libere,
        // iar pionul nu a fost deja mutat, acesta poate avansa 2 patrate.
        let i2 = (i1 as i32 + dir) as usize;
        if in_board(i2 as i32, j as i32) {
            if !tabla[i][j].piesa.clone().unwrap().mutat
                && (tabla[i2 as usize][j].piesa.is_none() || tot_ce_afecteaza)
            {
                rez.push((i2, j));
            }
        }
    }

    // TODO: adaugare en passant (adaugare istoric miscari -> verificam daca ultima miscare permite en passant)
    rez
}

/// Returneaza pozitiile pe care le poate ataca pionul de la *(i, j)*.
pub(super) fn ataca(tabla: &Tabla, poz: PozitieVerificata) -> Vec<PozitieVerificata> {
    let mut rez = Vec::new();
    let culoare = tabla[poz.0][poz.1].piesa.clone().unwrap().culoare;
    let i = if culoare == Culoare::Alb {
        poz.0 - 1
    } else {
        poz.0 + 1
    };
    for j in [poz.1 - 1, poz.1 + 1] {
        if in_board(i as i32, j as i32) {
            if let Some(victima) = &tabla[i][j].piesa {
                if victima.culoare != culoare {
                    rez.push((i, j));
                }
            }
        }
    }
    rez
}

use crate::tabla::{Tabla, input::in_board, Culoare};

/// Genereaza o lista cu miscarile posibile (linie, coloana) pentru pionul de la (i, j)
// TODO: repara mismatch isize-usize pt readability
// FIXME: pune regele in sah pe pozitii dubioase
pub(super) fn get(tabla: &Tabla, i: i32, j: i32, full_scan: bool) -> Vec<(usize, usize)> {
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
    if in_board(i1 as i32, j as i32) && (tabla[i1][j].piesa.is_none() || full_scan) {
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

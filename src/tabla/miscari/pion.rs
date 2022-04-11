use crate::tabla::{input::in_board, Culoare, MatTabla, Pozitie, Tabla, TipPiesa};

/// Genereaza o lista cu miscarile posibile (linie, coloana) pentru pionul de la (i, j)
/// TODO: repara mismatch isize-usize pt readability
pub(super) fn get(tabla: &MatTabla, i: i32, j: i32, tot_ce_afecteaza: bool) -> Vec<Pozitie> {
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
        if in_board(i2 as i32, j as i32)
            && !tabla[i][j].piesa.clone().unwrap().mutat
            && (tabla[i2 as usize][j].piesa.is_none() || tot_ce_afecteaza)
        {
            rez.push((i2, j));
        }
    }

    rez
}

/// Returneaza pozitiile pe care le poate ataca pionul.
/// *pe_bune* inseamna ca trebuie sa existe o piesa atacata.
/// TODO: nume mai sugestiv
pub(super) fn ataca(tabla: &Tabla, poz: Pozitie, pe_bune: bool) -> Vec<Pozitie> {
    let mut rez = vec![];

    let culoare = tabla.at(poz).piesa.clone().unwrap().culoare;
    let i = if culoare == Culoare::Alb {
        poz.0 - 1
    } else {
        poz.0 + 1
    };

    for diry in [-1, 1] {
        let j = (poz.1 as i32 + diry) as usize;
        if in_board(i as i32, j as i32) {
            // Daca cautarea nu este pe bune, nu are de ce sa se verifice
            // daca piesa exista sau daca poate face en passant, pt ca *atacat*
            // foloseste doar pt a verifica sahul.
            if !pe_bune {
                rez.push((i, j));
                continue;
            }

            if let Some(victima) = &tabla.at((i, j)).piesa {
                if victima.culoare != culoare {
                    rez.push((i, j));
                }
            } else if let Some(victima) = &tabla.at((poz.0, j)).piesa {
                if victima.culoare != culoare && verif_en_passant(tabla, poz.0, j) {
                    rez.push((i, j));
                }
            }
        }
    }

    rez
}

/// Verifica daca piesa de la pozitia (i, j) e pion si
/// **tocmai** a fost mutat 2 patratele.
/// TODO: trebuie sa fie ultima-ultima miscare i guess, deci
/// merge si o cautare in istoric sau macar ultima mutare.
fn verif_en_passant(tabla: &Tabla, i: usize, j: usize) -> bool {
    if let Some((src, dest)) = &tabla.ultima_miscare {
        if i == dest.0 && j == dest.1 {
            if let Some(piesa) = &tabla.at((i, j)).piesa {
                if piesa.tip == TipPiesa::Pion && (dest.0 as isize - src.0 as isize).abs() == 2 {
                    return true;
                }
            }
        }
    }

    false
}

use crate::tabla::{input, Culoare, MatTabla, Pozitie, Tabla, TipPiesa};

use super::{Mutare, TipMutare};

/// Genereaza o lista cu miscarile posibile (linie, coloana) pentru pionul de la `poz`.
pub(super) fn get(tabla: &MatTabla, poz: Pozitie, check_all: bool) -> Vec<Mutare> {
    let mut rez = Vec::new();

    let culoare = tabla[poz.0][poz.1].piesa.clone().unwrap().culoare;

    // Memoreaza directia in care se misca pionul in functie de culoare:
    // alb se misca in sus => scade randul in matrice;
    // negru se misca in jos => creste randul in matrice.
    let dir = if culoare == Culoare::Alb { -1 } else { 1 };
    // Randul din fata pionului
    let i1 = (poz.0 as i32 + dir) as usize;

    // Daca patratul din fata pionului exista si e liber, miscarea e valabila.
    if input::in_board(i1 as i32, poz.1 as i32) && (tabla[i1][poz.1].piesa.is_none() || check_all) {
        rez.push(Mutare {
            tip: TipMutare::Normal,
            dest: (i1, poz.1),
        });

        // Daca urmatoarele 2 patrate din fata pionului sunt libere,
        // iar pionul nu a fost deja mutat, acesta poate avansa 2 patrate.
        let i2 = (i1 as i32 + dir) as usize;
        if input::in_board(i2 as i32, poz.1 as i32)
            && !tabla[poz.0][poz.1].piesa.clone().unwrap().mutat
            && (tabla[i2 as usize][poz.1].piesa.is_none() || check_all)
        {
            rez.push(Mutare {
                tip: TipMutare::Normal,
                dest: (i2, poz.1),
            });
        }
    }

    rez
}

/// Returneaza pozitiile pe care le poate ataca pionul.
/// `must_exist` inseamna ca trebuie sa existe o piesa atacata.
pub(super) fn ataca(tabla: &Tabla, poz: Pozitie, must_exist: bool) -> Vec<Mutare> {
    let mut rez = vec![];

    let culoare = tabla.at(poz).piesa.clone().unwrap().culoare;
    let i = if culoare == Culoare::Alb {
        poz.0 - 1
    } else {
        poz.0 + 1
    };

    for diry in [-1, 1] {
        let j = (poz.1 as i32 + diry) as usize;
        if input::in_board(i as i32, j as i32) {
            // Daca cautarea nu este pe bune, nu are de ce sa se verifice
            // daca piesa exista sau daca poate face en passant, pt ca *atacat*
            // foloseste doar pt a verifica sahul.
            if !must_exist {
                rez.push(Mutare {
                    tip: TipMutare::Captura,
                    dest: (i, j),
                });
                continue;
            }

            if let Some(victima) = &tabla.at((i, j)).piesa {
                if victima.culoare != culoare {
                    rez.push(Mutare {
                        tip: TipMutare::Captura,
                        dest: (i, j),
                    });
                }
            } else if let Some(victima) = &tabla.at((poz.0, j)).piesa {
                if victima.culoare != culoare && verif_en_passant(tabla, (poz.0, j)) {
                    rez.push(Mutare {
                        tip: TipMutare::EnPassant((poz.0, j)),
                        dest: (i, j),
                    });
                }
            }
        }
    }

    rez
}

/// Verifica daca piesa de la pozitia `poz` e pion si
/// **tocmai** a fost mutat 2 patratele.
fn verif_en_passant(tabla: &Tabla, poz: Pozitie) -> bool {
    if let Some((src, dest)) = &tabla.ultima_miscare {
        if poz == *dest {
            if let Some(piesa) = &tabla.at(poz).piesa {
                if piesa.tip == TipPiesa::Pion && (dest.0 as isize - src.0 as isize).abs() == 2 {
                    return true;
                }
            }
        }
    }

    false
}

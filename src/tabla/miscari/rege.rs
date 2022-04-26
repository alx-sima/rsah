use crate::{
    tabla::{input::in_board, sah::e_atacat, MatTabla, Tabla, TipPiesa},
    Mutare, TipMutare,
};

/// Genereaza o lista cu miscarile posibile (linie, coloana) pentru regele de la (i, j)
pub(super) fn get(tabla: &MatTabla, i: i32, j: i32, tot_ce_afecteaza: bool) -> Vec<Mutare> {
    let mut rez = vec![];
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
                        || tot_ce_afecteaza
                    {
                        rez.push(Mutare {
                            tip: TipMutare::Captura,
                            dest: (sumi, sumj),
                        });
                    }
                } else {
                    rez.push(Mutare {
                        tip: TipMutare::Normal,
                        dest: (sumi, sumj),
                    });
                }
            }
        }
    }

    rez
}

/// Se cauta pozitiile unde, daca este mutat regele, se face rocada.
///
/// Rocada se poate face doar daca:
///  - nici tura, nici regele nu au fost mutate
///  - nu se trece prin alte piese
///  - nu se trece prin sah
///  - tura este la *4 patratele la stanga* (rocada mare),
/// sau *3 la dreapta* (rocada mica) de rege. 
/// Nu conteaza pozitia efectiva, ci doar distanta,
///  pt. a se putea face rocada si pe layouturi random sau custom.
pub(super) fn rocada(tabla: &Tabla, i: usize, j: usize) -> Vec<Mutare> {
    let mut rez = vec![];
    let rege = tabla.at((i, j)).piesa.clone().unwrap();

    // Daca regele a fost mutat sau e in sah, rocada nu se poate face.
    if rege.mutat || e_atacat(tabla, (i, j), rege.culoare) {
        return rez;
    }

    // Cauta rocada intai rocada mare (in dreapta),
    // apoi pe cea mica (in stanga).
    for dtura in [-4, 3] {
        // Daca pozitia turei nu este in tabla, nu are sens cautarea rocadei.
        if in_board(i as i32, j as i32 + dtura) {
            if let Some(tura) = tabla.at((i, (j as i32 + dtura) as usize)).piesa.clone() {
                if tura.culoare == rege.culoare && tura.tip == TipPiesa::Tura && !tura.mutat {
                    let dir = dtura.signum();
                    let mut ok = true;

                    // Se cauta ca toate patratele intre rege
                    // si tura sa fie libere si neatacate.
                    for k in 1..dtura.abs() {
                        let destj = (j as i32 + dir * k) as usize;
                        if tabla.at((i, destj)).piesa.is_some()
                            || e_atacat(tabla, (i, destj), rege.culoare)
                        {
                            ok = false;
                            break;
                        }
                    }

                    if ok {
                        rez.push(Mutare {
                            tip: TipMutare::Rocada((i, (j as i32 + dtura) as usize)),
                            dest: (i, (j as i32 + dir * 2) as usize),
                        });
                    }
                }
            }
        }
    }
    rez
}

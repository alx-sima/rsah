use crate::tabla::{
    input,
    miscari::{Mutare, TipMutare},
    sah, MatTabla, Tabla, TipPiesa,
};
use crate::Pozitie;

/// Genereaza o lista cu miscarile posibile (linie, coloana) pentru regele de la `poz`.
pub(super) fn get(tabla: &MatTabla, poz: Pozitie, check_all: bool) -> Vec<Mutare> {
    let mut rez = vec![];

    for di in -1..=1 {
        for dj in -1..=1 {
            let i = poz.0 as i32 + di;
            let j = poz.1 as i32 + dj;
            if input::in_board(i, j) {
                let ui = i as usize;
                let uj = j as usize;

                if let Some(mutare) = super::mutare_imediata(tabla, poz, (ui, uj), check_all) {
                    rez.push(mutare);
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
pub(super) fn rocada(tabla: &Tabla, poz: Pozitie) -> Vec<Mutare> {
    let mut rez = vec![];
    let rege = tabla.at(poz).piesa.clone().unwrap();

    // Daca regele a fost mutat sau e in sah, rocada nu se poate face.
    if rege.mutat || sah::e_atacat(tabla, poz, rege.culoare) {
        return rez;
    }

    // Cauta rocada intai rocada mare (in dreapta),
    // apoi pe cea mica (in stanga).
    for dtura in [-4, 3] {
        let jtura = poz.1 as i32 + dtura;

        // Daca pozitia turei nu este in tabla, nu are sens cautarea rocadei.
        if input::in_board(poz.0 as i32, jtura) {
            if let Some(tura) = tabla.at((poz.0, jtura as usize)).piesa.clone() {
                if tura.culoare == rege.culoare && tura.tip == TipPiesa::Tura && !tura.mutat {
                    let dir = dtura.signum();
                    let mut ok = true;

                    // Se cauta ca toate patratele intre rege
                    // si tura sa fie libere si neatacate.
                    for k in 1..dtura.abs() {
                        let destj = (poz.1 as i32 + dir * k) as usize;
                        if tabla.at((poz.0, destj)).piesa.is_some()
                            || sah::e_atacat(tabla, (poz.0, destj), rege.culoare)
                        {
                            ok = false;
                            break;
                        }
                    }

                    if ok {
                        rez.push(Mutare {
                            // In `tip` se va retine pozitia *turei*.
                            tip: TipMutare::Rocada((poz.0, jtura as usize)),
                            // In `dest` se va retine destinatia *regelui*
                            // (la 2 patratele distanta).
                            dest: (poz.0, (poz.1 as i32 + dir * 2) as usize),
                        });
                    }
                }
            }
        }
    }

    rez
}

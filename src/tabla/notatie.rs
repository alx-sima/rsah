use crate::TipMutare;
use crate::{tabla::input::in_board, Mutare};

use super::miscari::get_poz_rege;

use super::{Culoare, MatTabla, Pozitie, Tabla, TipPiesa};

use lazy_static::lazy_static;
use regex::Regex;

/// Expresia dupa care se (de)codifica o mutare.
const REGEXPR: &str = r"^([RBNQK])?([a-h1-8])?(x)?([a-h])([1-8])([+#])?( e.p.)?$";

/// Genereaza [algebraic notation-ul][1]
/// pentru mutarea de la `src` -> `dest`.
///
/// [1]: https://en.wikipedia.org/wiki/Algebraic_notation_(chess)
pub(crate) fn encode_move(tabla: &MatTabla, src: Pozitie, mutare: &Mutare) -> String {
    let p_old = tabla[src.0][src.1].clone().piesa.unwrap();
    let p_new = tabla[mutare.dest.0][mutare.dest.1].clone().piesa;

    let mut notatie = format!("{}", p_old.tip);
    let mut scrie_lin = false;
    let mut scrie_col = false;

    for k in &tabla[mutare.dest.0][mutare.dest.1].afecteaza {
        // Exista o alta piesa care poate ajunge pe celula destinatie...
        if src != *k {
            // ... si e de acelasi tip
            if let Some(piesa) = &tabla[k.0 as usize][k.1 as usize].piesa {
                if piesa.tip == p_old.tip && piesa.culoare == p_old.culoare {
                    // Daca sunt pe aceeasi coloana se va afisa linia
                    if k.1 == src.1 {
                        scrie_lin = true;
                    // Daca nu, se scrie coloana, indiferent daca sunt pe
                    // aceeasi linie sau nu (se poate intampla la cai de ex.)
                    } else {
                        scrie_col = true;
                    }
                }
            }
        }
    }

    // Pt. ca en passantul nu e tinut nici in `afecteaza`,
    // nici in `ataca`, trebuie verificat manual daca exista
    // o alta piesa care poate face acelasi en passant, caz
    // in care notatia contine si un discriminant.
    if let TipMutare::EnPassant((_, jpion)) = mutare.tip {
        // Pionul `victima` este la mijlocul dintre
        // pionul `src` si potentialul pion cautat.
        let j_celalalt_pion = 2 * jpion - src.1;
        if let Some(piesa) = &tabla[src.0][j_celalalt_pion].piesa {
            if piesa.tip == TipPiesa::Pion && piesa.culoare == p_old.culoare {
                scrie_col = true;
            }
        }
    }

    if scrie_col {
        notatie.push((src.1 as u8 + b'a') as char);
    }
    if scrie_lin {
        notatie.push((b'8' - src.0 as u8) as char);
    }

    if p_new.is_some() {
        notatie += "x"
    }

    notatie.push((mutare.dest.1 as u8 + b'a') as char);
    notatie.push((b'8' - mutare.dest.0 as u8) as char);

    if let TipMutare::EnPassant(_) = mutare.tip {
        notatie += " e.p.";
    }

    notatie
}

/// Inversul operatiei [encode_move]:
/// Din stringul `mov` si pozitiile pieselor de pe `tabla`,
/// se deduc pozitiile de unde si pana unde a fost mutata piesa.
///
/// Rezultatul va fi `Some(poz_piesa, mutare)`
/// sau `None` (stringul nu este valid).
///
/// FIXME: decodingul nu merge pt en passant.
pub(crate) fn decode_move(tabla: &Tabla, mov: &str, turn: Culoare) -> Option<(Pozitie, Mutare)> {
    lazy_static! {
        static ref REGX: Regex = Regex::new(REGEXPR).unwrap();
    }

    // Nu trebuie parcurse capturile, pt ca regexul sigur
    // nu va returna mai multe (e intre /^/ si /$/)
    if let Some(capture) = REGX.captures_iter(mov).next() {
        if let (Some(j), Some(i)) = (capture.get(4), capture.get(5)) {
            // FIXME: urat:(
            let pozi = 8 - str::parse::<usize>(i.as_str()).unwrap();
            let pozj = (j.as_str().chars().last().unwrap() as u8 - b'a') as usize;

            let tip_piesa = match capture.get(1) {
                Some(x) => match x.as_str() {
                    "R" => TipPiesa::Tura,
                    "N" => TipPiesa::Cal,
                    "B" => TipPiesa::Nebun,
                    "Q" => TipPiesa::Regina,
                    "K" => TipPiesa::Rege,
                    _ => unreachable!(),
                },
                None => TipPiesa::Pion,
            };

            // " e.p." e prezent -> miscarea este en passant.
            if capture.get(7).is_some() {
                if tip_piesa != TipPiesa::Pion {
                    return None;
                }

                let lin = if turn == Culoare::Alb {
                    pozi as i32 + 1
                } else {
                    pozi as i32 - 1
                };

                if let Some(discriminant) = capture.get(2) {
                    let d_str = discriminant.as_str();
                    let col = d_str.chars().next().unwrap() as u8 - b'a';

                    if in_board(lin, col as i32) {
                        let lin = lin as usize;
                        let col = col as usize;

                        if let Some(piesa) = &tabla.at((lin, col)).piesa {
                            if piesa.tip == TipPiesa::Pion {
                                return Some((
                                    (lin, col),
                                    Mutare {
                                        tip: TipMutare::EnPassant((lin, pozj)),
                                        dest: (pozi, pozj),
                                    },
                                ));
                            }
                        }
                    }
                    return None;
                }

                // En passantul nu este tinut in `afecteaza`,
                // pentru ca e doar pentru pioni, asa ca
                // trebuie cautat manual.
                for dir in [-1, 1] {
                    let jpion = pozj as i32 + dir;
                    if in_board(lin, jpion) {
                        let lin = lin as usize;
                        let col = jpion as usize;

                        if let Some(piesa) = &tabla.at((lin, col)).piesa {
                            if piesa.tip == TipPiesa::Pion {
                                return Some((
                                    (lin, col),
                                    Mutare {
                                        tip: TipMutare::EnPassant((lin, pozj)),
                                        dest: (pozi, pozj),
                                    },
                                ));
                            }
                        }
                    }
                }
            }

            let (dif_i, dif_j) = if let Some(discriminant) = capture.get(2) {
                let d_str = discriminant.as_str();
                if let Ok(lin) = str::parse::<usize>(d_str) {
                    (Some(8 - lin), None)
                } else {
                    let col = (*d_str.chars().collect::<Vec<char>>().last().unwrap() as u8 - b'a')
                        as usize;
                    (None, Some(col))
                }
            } else {
                (None, None)
            };

            // "x" e prezent -> mutarea este capturare,
            // asa ca piesa care face miscarea trebuie
            // cautata printre cele care ataca, nu care
            // se pot muta pe pozitia data. Asta deoarece
            // pionul se misca diferit fata de cum ataca,
            // iar en passantul este tratat separat mai sus.
            let (candidati, tip_mutare) = if capture.get(3).is_some() {
                (&tabla.at((pozi, pozj)).atacat, TipMutare::Captura)
            } else {
                (&tabla.at((pozi, pozj)).afecteaza, TipMutare::Normal)
            };

            // Parcurge celulele care ataca (pozi, pozj)
            // si cauta piesa:
            //  - de aceeasi culoare cu jucatorul curent;
            //  - care e de acelasi tip cu piesa mutata;
            //  - in caz ca exista >1 piesa care se
            // incadreaza, face diferenta (cu discriminantul).
            for (i, j) in candidati {
                if let Some(piesa) = &tabla.at((*i, *j)).piesa {
                    if piesa.culoare == turn && piesa.tip == tip_piesa {
                        if let Some(dif_i) = dif_i {
                            if *i == dif_i {
                                return Some((
                                    (*i, *j),
                                    Mutare {
                                        tip: tip_mutare,
                                        dest: (pozi, pozj),
                                    },
                                ));
                            }
                        } else if let Some(dif_j) = dif_j {
                            if *j == dif_j {
                                return Some((
                                    (*i, *j),
                                    Mutare {
                                        tip: tip_mutare,
                                        dest: (pozi, pozj),
                                    },
                                ));
                            }
                        } else {
                            return Some((
                                (*i, *j),
                                Mutare {
                                    tip: tip_mutare,
                                    dest: (pozi, pozj),
                                },
                            ));
                        }
                    }
                }
            }
            return None;
        }
    // Rocada mica
    } else if mov == "O-O" {
        let (i, j) = get_poz_rege(tabla, turn);

        return Some((
            (i, j),
            Mutare {
                tip: TipMutare::Rocada((i, j + 3)),
                dest: (i, j + 2),
            },
        ));
    // Rocada mare
    } else if mov == "O-O-O" {
        let (i, j) = get_poz_rege(tabla, turn);

        return Some((
            (i, j),
            Mutare {
                tip: TipMutare::Rocada((i, j - 4)),
                dest: (i, j - 2),
            },
        ));
    }
    None
}

#[cfg(test)]
mod test {
    use regex::Regex;

    use crate::{
        tabla::{Culoare, Tabla},
        TipMutare,
    };

    #[test]
    /// Verifica daca regexul recunoaste stringurile cu mutarile.
    fn regexurile_se_potrivesc() {
        let input = ["Nexc7", "Rfh7"];
        let re = Regex::new(super::REGEXPR).unwrap();
        for test in input {
            assert!(re.is_match(test));
        }
    }

    #[test]
    /// Testeaza daca se decodeaza corect miscarile.
    fn decodeaza_miscari() {
        let input = [
            ("Nexc7", ["N", "  p", "    N", "", "", "", "", "R"]),
            ("Rfh7", ["", "     R", "       R", "", "", "", "", ""]),
            ("R6a7", ["R", "", "R", "", "", "", "", ""]),
        ];

        for (str, template) in input {
            let tabla = Tabla::from(template);
            println!("{:?}", super::decode_move(&tabla, str, Culoare::Alb));
        }
    }

    #[test]
    /// Se testeaza daca se decodeaza corect rocadele.
    fn rocade() {
        let template = ["R   K  R", "", "", "", "", "", "", " k"];
        let tabla = Tabla::from(template);

        if let Some((_, mutare)) = super::decode_move(&tabla, "O-O", Culoare::Alb) {
            assert_eq!(mutare.tip, TipMutare::Rocada((0, 7)));
            assert_eq!(mutare.dest, (0, 6));
        } else {
            panic!("Nu s-a gasit rocada mica");
        }

        if let Some((_, mutare)) = super::decode_move(&tabla, "O-O-O", Culoare::Alb) {
            assert_eq!(mutare.tip, TipMutare::Rocada((0, 0)));
            assert_eq!(mutare.dest, (0, 2));
        } else {
            panic!("Nu s-a gasit rocada mare");
        }
    }
}

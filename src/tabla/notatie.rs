use super::miscari::get_poz_rege;

use super::{Culoare, MatTabla, Pozitie, TipPiesa};

use lazy_static::lazy_static;
use regex::Regex;

/// Expresia dupa care se decodifica o mutare.
const REGEXPR: &str = r"^([RBNQK])?([a-h1-8])?(x)?([a-h])([1-8])([+#])?( e. p.)?$";

/// Genereaza [algebraic notationul][1]
/// pentru mutarea de la `src` -> `dest`.
///
/// [1]: https://en.wikipedia.org/wiki/Algebraic_notation_(chess)
pub(crate) fn encode_move(tabla: &MatTabla, src: Pozitie, dest: Pozitie) -> String {
    let p_old = tabla[src.0][src.1].clone();
    let p_new = tabla[dest.0][dest.1].clone();
    // Adaugare miscare in istoric
    // FIXME: scapa de unwrap
    let mut mutare = format!("{}", p_old.clone().piesa.unwrap().tip);

    let mut scrie_lin = false;
    let mut scrie_col = false;
    for k in &tabla[dest.0][dest.1].atacat {
        // Exista o alta piesa care ataca celula destinatie...
        if k.0 != src.0 || k.1 != src.1 {
            // ... si e de acelasi tip
            if tabla[k.0 as usize][k.1 as usize].piesa == p_old.clone().piesa {
                // Daca sunt pe aceeasi coloana se va afisa linia
                if k.1 == src.1 {
                    scrie_lin = true;
                // Daca nu, se scrie coloana, indiferent daca sunt pe aceeasi linie sau nu (se poate intampla la cai de ex.)
                } else {
                    scrie_col = true;
                }
            }
        }
    }

    if scrie_col {
        mutare.push((src.1 as u8 + b'a') as char);
    }
    if scrie_lin {
        mutare.push((b'8' - src.0 as u8) as char);
    }

    if p_new.piesa.is_some() {
        mutare += "x"
    }

    mutare.push((dest.1 as u8 + b'a') as char);
    mutare.push((b'8' - dest.0 as u8) as char);

    mutare
}

/// Inversul operatiei [encode_move]:
/// Din stringul `mov` si pozitiile pieselor de pe `tabla`,
/// se deduc pozitiile de unde si pana unde a fost mutata piesa.
/// Rezultatul va fi `Some((src_i, src_j), (dest_i, dest_j))`
/// sau `None` (stringul nu este valid).
/// FIXME: decodingul nu merge pt en passant.
pub(crate) fn decode_move(
    tabla: &MatTabla,
    mov: &str,
    turn: Culoare,
) -> Option<(Pozitie, Pozitie)> {
    lazy_static! {
        static ref REGX: Regex = Regex::new(REGEXPR).unwrap();
    }

    // Nu trebuie parcurse capturile, pt ca regexul sigur
    // nu va returna mai multe (e intre /^/ si /$/)
    if let Some(capture) = REGX.captures_iter(mov).next() {
        if let (Some(j), Some(i)) = (capture.get(4), capture.get(5)) {
            // FIXME: jegos
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
            println!("{:?}", mov);

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

            // Parcurge celulele care ataca (pozi, pozj)
            // si cauta piesa:
            //  - de aceeasi culoare cu jucatorul curent;
            //  - care e de acelasi tip cu piesa mutata;
            //  - in caz ca exista >1 piesa care se incadreaza, face diferenta (cu discriminantul).
            for (i, j) in &tabla[pozi][pozj].afecteaza {
                println!("{:?}", (i, j));
                if let Some(piesa) = &tabla[*i][*j].piesa {
                    if piesa.culoare == turn && piesa.tip == tip_piesa {
                        if let Some(dif_i) = dif_i {
                            if *i == dif_i {
                                return Some(((*i, *j), (pozi, pozj)));
                            }
                        } else if let Some(dif_j) = dif_j {
                            if *j == dif_j {
                                return Some(((*i, *j), (pozi, pozj)));
                            }
                        } else {
                            return Some(((*i, *j), (pozi, pozj)));
                        }
                    }
                }
            }
            return None;
        }
    // rocada mica
    } else if mov == "O-O" {
        let (i, j) = get_poz_rege(tabla, turn);
        return Some(((i, j), (i, j + 2)));
    // rocada mare
    } else if mov == "O-O-O" {
        let (i, j) = get_poz_rege(tabla, turn);
        return Some(((i, j), (i, (j - 2) as usize)));
    }
    None
}

#[cfg(test)]
mod test {
    use regex::Regex;

    use crate::tabla::{Culoare, Tabla};

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
            println!("{:?}", super::decode_move(&tabla.mat, str, Culoare::Alb));
        }
    }

    #[test]
    /// Se testeaza daca se decodeaza corect rocadele.
    fn rocade() {
        let template = ["R   K  R", "", "", "", "", "", "", " k"];
        let tabla = Tabla::from(template);

        assert_eq!(
            super::decode_move(&tabla.mat, "O-O", Culoare::Alb),
            Some(((0, 4), (0, 6)))
        );
        assert_eq!(
            super::decode_move(&tabla.mat, "O-O-O", Culoare::Alb),
            Some(((0, 4), (0, 2)))
        );
    }
}

use crate::tabla::TipPiesa;

use super::{Culoare, Patratel};

use lazy_static::lazy_static;
use regex::Regex;

/// Pentru mutarea de la *src_poz* la *dest_poz* (tuple de forma *(i, j)*), genereaza
/// [**algebraic notation**](https://en.wikipedia.org/wiki/Algebraic_notation_(chess))-ul.
pub(crate) fn encode_move(
    tabla: &mut [[Patratel; 8]; 8],
    src_poz: (usize, usize),
    dest_poz: (usize, usize),
) -> String {
    let p_old = tabla[src_poz.0][src_poz.1].clone();
    let p_new = tabla[dest_poz.0][dest_poz.1].clone();
    // Adaugare miscare in istoric
    // FIXME: scapa de unwrap
    let mut mutare = format!("{}", p_old.clone().piesa.unwrap().tip);

    let mut scrie_lin = false;
    let mut scrie_col = false;
    for k in &tabla[dest_poz.0][dest_poz.1].atacat {
        // Exista o alta piesa care ataca celula destinatie...
        if k.0 != src_poz.0 || k.1 != src_poz.1 {
            // ... si e de acelasi tip
            if tabla[k.0 as usize][k.1 as usize].piesa == p_old.clone().piesa {
                // Daca sunt pe aceeasi coloana se va afisa linia
                if k.1 == src_poz.1 {
                    scrie_lin = true;
                // Daca nu, se scrie coloana, indiferent daca sunt pe aceeasi linie sau nu (se poate intampla la cai de ex.)
                } else {
                    scrie_col = true;
                }
            }
        }
    }

    if scrie_col {
        // Wall of shame: aduni un string cu un numar si vrei sa-ti dea un caracter
        // Ce voiai sa faci: 'a' + 1 = 'b'
        // Ce ai facut: 1 + "a" = "1a"

        // 65 = 'a'
        mutare += ((src_poz.1 as u8 + 97u8) as char).to_string().as_str();
    }
    if scrie_lin {
        mutare += (8 - src_poz.0).to_string().as_str();
    }

    if p_new.piesa.is_some() {
        mutare += "x"
    }

    mutare += ((dest_poz.1 as u8 + 97u8) as char).to_string().as_str();
    mutare += (8 - dest_poz.0).to_string().as_str();
    mutare
}

/// Inversul operatiei **encode_move**:
/// Din stringul *mov* si pozitiile pieselor de pe *tabla*,
/// se deduc pozitiile de unde si pana unde a fost mutata piesa.
/// Rezultatul va fi *Some((src_i, src_j), (dest_i, dest_j))*
/// sau *None* (stringul nu este valid).
pub(crate) fn decode_move(
    tabla:  &[[Patratel; 8]; 8],
    mov: &str,
    turn: Culoare,
) -> Option<((usize, usize), (usize, usize))> {
    lazy_static! {
        static ref REGX: Regex =
            Regex::new(r"^([RBNQK])?([a-h1-8])?(x)?([a-h])([1-8])([+#])?$").unwrap();
    }
    // Nu trebuie parcurse capturile, pt ca regexul sigur
    // nu va returna mai multe (e intre /^/ si /$/)
    let capture = REGX.captures_iter(mov).nth(0).unwrap();
    if let (Some(j), Some(i)) = (capture.get(4), capture.get(5)) {
        // FIXME: jegos
        let pozi = 8 - str::parse::<usize>(i.as_str()).unwrap();
        let pozj =
            (*j.as_str().chars().collect::<Vec<char>>().last().unwrap() as u8 - 97u8) as usize;
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

        let (dif_i, dif_j) = if let Some(discriminant) = capture.get(2) {
            let d_str = discriminant.as_str();
            if let Some(lin) = str::parse::<usize>(d_str).ok() {
                (Some(8 - lin), None)
            } else {
                let col =
                    (*d_str.chars().collect::<Vec<char>>().last().unwrap() as u8 - 97u8) as usize;
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
        for (i, j) in &tabla[pozi][pozj].atacat {
            println!("{} {}", i, j);
            if let Some(piesa) = &tabla[*i][*j].piesa {
                println!("({},{}):{:?}", i, j, piesa);
                if piesa.culoare == turn {
                    if piesa.tip == tip_piesa {
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
        }
        return None;
    }
    None
}

#[cfg(test)]
mod test {
    use regex::Regex;

    use crate::tabla::{generare, Culoare};

    #[test]
    /// Verifica daca regexul recunoaste stringurile cu mutarile
    fn regexurile_se_potrivesc() {
        let input = ["Nexc7", "Rfh7"];
        let re = Regex::new(r"^([RBNQK])?([a-h1-8])?(x)?([a-h])([1-8])([+#])?$").unwrap();
        for test in input {
            assert!(re.is_match(test));
        }
    }

    #[test]
    fn decodeaza_miscari() {
        let input = [
            (
                "Nexc7",
                [
                    "N       ", "  p     ", "    N   ", "        ", "        ", "        ",
                    "        ", "R       ",
                ],
            ),
            (
                "Rfh7",
                [
                    "        ", "     R  ", "       R", "        ", "        ", "        ",
                    "        ", "        ",
                ],
            ),
            (
                "R6a7",
                [
                    "R       ", "        ", "R       ", "        ", "        ", "        ",
                    "        ", "        ",
                ],
            ),
        ];

        for (str, template) in input {
            let tabla = generare::tabla_from(template);
            println!("{:?}", super::decode_move(&tabla, str, Culoare::Alb));
        }
    }
}

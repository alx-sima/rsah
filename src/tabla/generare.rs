use super::{miscari, Culoare, Piesa, Tabla, TipPiesa};

/// Genereaza layoutul tablei de sah dupa template.
/// (Pt. a fi mai usor de citit (mai ales cand e hardcodat),
/// 'template' este un vector de stringuri, fiecare string
/// marcand o linie, in loc sa fie un singur string separat de '\n')
pub(crate) fn tabla_from(template: [&str; 8]) -> Tabla {
    let mut tabla: Tabla = Default::default();
    for (i, line) in template.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            let culoare = if c.is_lowercase() {
                Culoare::Negru
            } else {
                Culoare::Alb
            };
            // FIXME: urat:(
            match c.to_lowercase().to_string().as_str() {
                "p" => tabla[i][j].piesa = Some(Piesa::new(TipPiesa::Pion, culoare)),
                "r" => tabla[i][j].piesa = Some(Piesa::new(TipPiesa::Tura, culoare)),
                "b" => tabla[i][j].piesa = Some(Piesa::new(TipPiesa::Nebun, culoare)),
                "n" => tabla[i][j].piesa = Some(Piesa::new(TipPiesa::Cal, culoare)),
                "k" => tabla[i][j].piesa = Some(Piesa::new(TipPiesa::Rege, culoare)),
                "q" => tabla[i][j].piesa = Some(Piesa::new(TipPiesa::Regina, culoare)),
                _ => tabla[i][j].piesa = None,
            }
        }
    }

    // Calculeaza pozitiile atacate
    for i in 0..8 {
        for j in 0..8 {
            miscari::set_influenta(&mut tabla, (i, j));
        }
    }

    tabla
}

/// Genereaza o tabla de sah clasica
pub(crate) fn tabla_clasica() -> Tabla {
    tabla_from([
        "rnbqkbnr", "pppppppp", "........", "........", "........", "........", "PPPPPPPP",
        "RNBQKBNR",
    ])
}

pub(crate) fn _tabla_cu_pozitii(piese: Vec<&str>) -> Tabla {
    let mut tabla = Tabla::default();
    for piesa in piese {
        let i = piesa.chars().nth(0).unwrap() as usize - 'a' as usize;
        let j = piesa.chars().nth(1).unwrap() as usize - '1' as usize;
        let tip = piesa.chars().nth(2).unwrap();
        let culoare = if tip.is_uppercase() {
            Culoare::Alb
        } else {
            Culoare::Negru
        };
        let tip = match tip.to_lowercase().to_string().as_str() {
            "p" => TipPiesa::Pion,
            "r" => TipPiesa::Tura,
            "b" => TipPiesa::Nebun,
            "n" => TipPiesa::Cal,
            "k" => TipPiesa::Rege,
            "q" => TipPiesa::Regina,
            _ => panic!("Tipul piesei {} nu e valid", piesa),
        };

        tabla[i][j].piesa = Some(Piesa::new(tip, culoare));
    }

    // Calculeaza pozitiile atacate
    for i in 0..8 {
        for j in 0..8 {
            miscari::set_influenta(&mut tabla, (i, j));
        }
    }

    tabla
}

/// Genereaza o tabla de joc aleatorie
pub(crate) fn _tabla_random() -> Tabla {
    todo!()
}

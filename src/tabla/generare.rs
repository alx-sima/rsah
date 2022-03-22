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
            miscari::set_influenta(&mut tabla, i, j);
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

/// Genereaza o tabla de joc aleatorie
pub(crate) fn _tabla_random() -> Tabla {
    todo!()
}

use super::{miscari, Culoare, MatTabla, Piesa, Tabla, TipPiesa};
use rand::{self, Rng};

/// Generarea layouturilor tablei de sah.
impl Tabla {
    /// Genereaza layoutul tablei de sah dupa template.
    /// (Pt. a fi mai usor de citit (mai ales cand e hardcodat),
    /// 'template' este un vector de stringuri, fiecare string
    /// marcand o linie, in loc sa fie un singur string separat de '\n')
    pub(crate) fn from(template: [&str; 8]) -> Tabla {
        let mut tabla: MatTabla = Default::default();

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
                    "q" => tabla[i][j].piesa = Some(Piesa::new(TipPiesa::Regina, culoare)),
                    "k" => tabla[i][j].piesa = Some(Piesa::new(TipPiesa::Rege, culoare)),
                    _ => tabla[i][j].piesa = None,
                }
            }
        }

        Tabla::from_layout(tabla)
    }

    /// Genereaza o tabla de sah clasica.
    pub(crate) fn new_clasica() -> Tabla {
        Tabla::from([
            "rnbqkbnr", "pppppppp", "........", "........", "........", "........", "PPPPPPPP",
            "RNBQKBNR",
        ])
    }

    /// Genereaza o tabla de joc aleatorie
    pub(crate) fn new_random() -> Tabla {
        let mut tabla = MatTabla::default();
        let mut rng = rand::thread_rng();

        for i in 0..2 {
            for j in 0..8 {
                let tip = rng
                    .choose(&[
                        TipPiesa::Pion,
                        TipPiesa::Tura,
                        TipPiesa::Nebun,
                        TipPiesa::Cal,
                        TipPiesa::Regina,
                    ])
                    .unwrap();

                tabla[i][j].piesa = Some(Piesa::new(*tip, Culoare::Negru));
                tabla[7 - i][j].piesa = Some(Piesa::new(*tip, Culoare::Alb));
            }
        }

        // Amplaseaza regele la final ca sa existe doar unul
        let i = rng.gen_range(0, 2);
        let j = rng.gen_range(0, 8);
        tabla[i][j].piesa = Some(Piesa::new(TipPiesa::Rege, Culoare::Negru));
        tabla[7 - i][j].piesa = Some(Piesa::new(TipPiesa::Rege, Culoare::Alb));

        Tabla::from_layout(tabla)
    }

    /// Genereaza o [Tabla] initializata dintr-o [MatTabla].
    pub(crate) fn from_layout(layout: MatTabla) -> Tabla {
        let mut tabla = Tabla {
            mat: layout,
            ..Default::default()
        };

        init_piese(&mut tabla);
        tabla
    }
}

/// Calculeaza pozitiile atacate/afectate de
/// fiecare piesa si retine pozitiile regilor.
pub(crate) fn init_piese(tabla: &mut Tabla) {
    for i in 0..8 {
        for j in 0..8 {
            miscari::set_influenta(tabla, (i, j));

            // Retine pozitiile regilor (se considera ca exista doar 2).
            if let Some(piesa) = &tabla.get((i, j)).piesa {
                if piesa.tip == TipPiesa::Rege {
                    if piesa.culoare == Culoare::Alb {
                        tabla.regi.0 = (i, j);
                    } else {
                        tabla.regi.1 = (i, j);
                    }
                }
            }
        }
    }
}

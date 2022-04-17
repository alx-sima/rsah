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
                    "k" => tabla[i][j].piesa = Some(Piesa::new(TipPiesa::Rege, culoare)),
                    "q" => tabla[i][j].piesa = Some(Piesa::new(TipPiesa::Regina, culoare)),
                    _ => tabla[i][j].piesa = None,
                }
            }
        }

        let mut tabla = Tabla {
            mat: tabla,
            ..Default::default()
        };

        // Calculeaza pozitiile atacate
        init_piese(&mut tabla);

        tabla
    }

    /// Genereaza o tabla de sah clasica.
    pub(crate) fn new_clasica() -> Tabla {
        Tabla::from([
            "rnbqkbnr", "pppppppp", "........", "........", "........", "........", "PPPPPPPP",
            "RNBQKBNR",
        ])
    }

    /*
    pub(crate) fn _tabla_cu_pozitii(piese: Vec<&str>) -> MatTabla {
        let mut tabla = MatTabla::default();
        for piesa in piese {
            let i = piesa.chars().next().unwrap() as usize - 'a' as usize;
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
    */

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

        let mut tabla = Tabla {
            mat: tabla,
            ..Default::default()
        };

        init_piese(&mut tabla);
        tabla
    }

    pub(crate) fn from_layout(layout: MatTabla) -> Tabla {
        let mut tabla = Tabla::default();
        for (i, line) in layout.iter().enumerate() {
            for (j, elem) in line.iter().enumerate() {
                if let Some(piesa) = &elem.piesa {
                    tabla.mat[i][j].piesa = Some(Piesa::new(piesa.tip, piesa.culoare));
                }
            }
        }

        init_piese(&mut tabla);
        tabla
    }
}

/// Calculeaza pozitiile atacate/afectate de fiecare piesa.
pub(crate) fn init_piese(tabla: &mut Tabla) {
    for i in 0..8 {
        for j in 0..8 {
            miscari::set_influenta(tabla, (i, j));
        }
    }
}

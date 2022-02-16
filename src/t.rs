use crate::{miscari, Culoare, Patratel, Piesa, TipPiesa};

pub(crate) fn generare_tabla(tabla: &mut [[Patratel; 8]; 8], template: &str) {
    for (i, line) in template.split('\n').enumerate() {
        for (j, c) in line.chars().enumerate() {
            let culoare = if c.is_lowercase() {
                Culoare::Negru
            } else {
                Culoare::Alb
            };
            match c.to_lowercase().to_string().as_str() {
                "r" => {
                    tabla[i][j].piesa = Some(Piesa {
                        tip: TipPiesa::Tura,
                        culoare,
                    })
                }
                "n" => {
                    tabla[i][j].piesa = Some(Piesa {
                        tip: TipPiesa::Nebun,
                        culoare,
                    })
                }
                "b" => {
                    tabla[i][j].piesa = Some(Piesa {
                        tip: TipPiesa::Cal,
                        culoare,
                    })
                }
                "q" => {
                    tabla[i][j].piesa = Some(Piesa {
                        tip: TipPiesa::Regina,
                        culoare,
                    })
                }
                "k" => {
                    tabla[i][j].piesa = Some(Piesa {
                        tip: TipPiesa::Rege,
                        culoare,
                    })
                }
                "p" => {
                    tabla[i][j].piesa = Some(Piesa {
                        tip: TipPiesa::Pion,
                        culoare,
                    })
                }
                _ => tabla[i][j].piesa = None,
            }
        }
    }
    for i in 0..8 {
        for j in 0..8 {
            set_atacat_field(tabla, i as i32, j as i32);
        }
    }
    for i in 0..8 {
        for j in 0..8 {
            print!("{:?}", tabla[i][j].atacat);
        }
        println!();
    }
}

/// Genereaza o tabla de sah clasica
pub(crate) fn generare_tabla_clasic(tabla: &mut [[Patratel; 8]; 8]) {
    generare_tabla(
        tabla,
        "rnbqkbnr\npppppppp\n........\n........\n........\n........\nPPPPPPPP\nRNBQKBNR",
    );
}

/// Marcheaza patratele atacate de piesa de la (i, j)
pub(crate) fn set_atacat_field(tabla: &mut [[Patratel; 8]; 8], i: i32, j: i32) {
    if let Some(piesa) = &tabla[i as usize][j as usize].piesa {
        if piesa.culoare == Culoare::Alb {
            for (x, y) in miscari::get_miscari(tabla, i, j, true) {
                tabla[x as usize][y as usize].atacat.0.push((i, j));
            }
        } else {
            for (x, y) in miscari::get_miscari(tabla, i, j, true) {
                tabla[x as usize][y as usize].atacat.1.push((i, j));
            }
        }
    }
}

/// Verifica daca celula (i, j) intra in tabla de joc
pub(crate) fn in_board(i: i32, j: i32) -> bool {
    i >= 0 && i < 8 && j >= 0 && j < 8
}

pub(crate) fn init_tabla() -> [[Patratel; 8]; 8] {
    // Wall of shame
    let rez = [
        [
            Patratel {
                atacat: (Vec::new(), Vec::new()),
                piesa: None,
            },
            Patratel {
                atacat: (Vec::new(), Vec::new()),
                piesa: None,
            },
            Patratel {
                atacat: (Vec::new(), Vec::new()),
                piesa: None,
            },
            Patratel {
                atacat: (Vec::new(), Vec::new()),
                piesa: None,
            },
            Patratel {
                atacat: (Vec::new(), Vec::new()),
                piesa: None,
            },
            Patratel {
                atacat: (Vec::new(), Vec::new()),
                piesa: None,
            },
            Patratel {
                atacat: (Vec::new(), Vec::new()),
                piesa: None,
            },
            Patratel {
                atacat: (Vec::new(), Vec::new()),
                piesa: None,
            },
        ],
        [
            Patratel {
                atacat: (Vec::new(), Vec::new()),
                piesa: None,
            },
            Patratel {
                atacat: (Vec::new(), Vec::new()),
                piesa: None,
            },
            Patratel {
                atacat: (Vec::new(), Vec::new()),
                piesa: None,
            },
            Patratel {
                atacat: (Vec::new(), Vec::new()),
                piesa: None,
            },
            Patratel {
                atacat: (Vec::new(), Vec::new()),
                piesa: None,
            },
            Patratel {
                atacat: (Vec::new(), Vec::new()),
                piesa: None,
            },
            Patratel {
                atacat: (Vec::new(), Vec::new()),
                piesa: None,
            },
            Patratel {
                atacat: (Vec::new(), Vec::new()),
                piesa: None,
            },
        ],
        [
            Patratel {
                atacat: (Vec::new(), Vec::new()),
                piesa: None,
            },
            Patratel {
                atacat: (Vec::new(), Vec::new()),
                piesa: None,
            },
            Patratel {
                atacat: (Vec::new(), Vec::new()),
                piesa: None,
            },
            Patratel {
                atacat: (Vec::new(), Vec::new()),
                piesa: None,
            },
            Patratel {
                atacat: (Vec::new(), Vec::new()),
                piesa: None,
            },
            Patratel {
                atacat: (Vec::new(), Vec::new()),
                piesa: None,
            },
            Patratel {
                atacat: (Vec::new(), Vec::new()),
                piesa: None,
            },
            Patratel {
                atacat: (Vec::new(), Vec::new()),
                piesa: None,
            },
        ],
        [
            Patratel {
                atacat: (Vec::new(), Vec::new()),
                piesa: None,
            },
            Patratel {
                atacat: (Vec::new(), Vec::new()),
                piesa: None,
            },
            Patratel {
                atacat: (Vec::new(), Vec::new()),
                piesa: None,
            },
            Patratel {
                atacat: (Vec::new(), Vec::new()),
                piesa: None,
            },
            Patratel {
                atacat: (Vec::new(), Vec::new()),
                piesa: None,
            },
            Patratel {
                atacat: (Vec::new(), Vec::new()),
                piesa: None,
            },
            Patratel {
                atacat: (Vec::new(), Vec::new()),
                piesa: None,
            },
            Patratel {
                atacat: (Vec::new(), Vec::new()),
                piesa: None,
            },
        ],
        [
            Patratel {
                atacat: (Vec::new(), Vec::new()),
                piesa: None,
            },
            Patratel {
                atacat: (Vec::new(), Vec::new()),
                piesa: None,
            },
            Patratel {
                atacat: (Vec::new(), Vec::new()),
                piesa: None,
            },
            Patratel {
                atacat: (Vec::new(), Vec::new()),
                piesa: None,
            },
            Patratel {
                atacat: (Vec::new(), Vec::new()),
                piesa: None,
            },
            Patratel {
                atacat: (Vec::new(), Vec::new()),
                piesa: None,
            },
            Patratel {
                atacat: (Vec::new(), Vec::new()),
                piesa: None,
            },
            Patratel {
                atacat: (Vec::new(), Vec::new()),
                piesa: None,
            },
        ],
        [
            Patratel {
                atacat: (Vec::new(), Vec::new()),
                piesa: None,
            },
            Patratel {
                atacat: (Vec::new(), Vec::new()),
                piesa: None,
            },
            Patratel {
                atacat: (Vec::new(), Vec::new()),
                piesa: None,
            },
            Patratel {
                atacat: (Vec::new(), Vec::new()),
                piesa: None,
            },
            Patratel {
                atacat: (Vec::new(), Vec::new()),
                piesa: None,
            },
            Patratel {
                atacat: (Vec::new(), Vec::new()),
                piesa: None,
            },
            Patratel {
                atacat: (Vec::new(), Vec::new()),
                piesa: None,
            },
            Patratel {
                atacat: (Vec::new(), Vec::new()),
                piesa: None,
            },
        ],
        [
            Patratel {
                atacat: (Vec::new(), Vec::new()),
                piesa: None,
            },
            Patratel {
                atacat: (Vec::new(), Vec::new()),
                piesa: None,
            },
            Patratel {
                atacat: (Vec::new(), Vec::new()),
                piesa: None,
            },
            Patratel {
                atacat: (Vec::new(), Vec::new()),
                piesa: None,
            },
            Patratel {
                atacat: (Vec::new(), Vec::new()),
                piesa: None,
            },
            Patratel {
                atacat: (Vec::new(), Vec::new()),
                piesa: None,
            },
            Patratel {
                atacat: (Vec::new(), Vec::new()),
                piesa: None,
            },
            Patratel {
                atacat: (Vec::new(), Vec::new()),
                piesa: None,
            },
        ],
        [
            Patratel {
                atacat: (Vec::new(), Vec::new()),
                piesa: None,
            },
            Patratel {
                atacat: (Vec::new(), Vec::new()),
                piesa: None,
            },
            Patratel {
                atacat: (Vec::new(), Vec::new()),
                piesa: None,
            },
            Patratel {
                atacat: (Vec::new(), Vec::new()),
                piesa: None,
            },
            Patratel {
                atacat: (Vec::new(), Vec::new()),
                piesa: None,
            },
            Patratel {
                atacat: (Vec::new(), Vec::new()),
                piesa: None,
            },
            Patratel {
                atacat: (Vec::new(), Vec::new()),
                piesa: None,
            },
            Patratel {
                atacat: (Vec::new(), Vec::new()),
                piesa: None,
            },
        ],
    ];
    rez
}

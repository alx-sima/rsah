use crate::{Culoare, Patratel, Piesa};

// genereaza o tabla clasica
pub(crate) fn generare_tabla(tabla: &mut [[Option<Patratel>; 8]; 8]) {
    // pioni
    for i in 0..8 {
        tabla[1][i] = Some(Patratel {
            piesa: Piesa::Pion,
            culoare: Culoare::Negru,
        });
        tabla[6][i] = Some(Patratel {
            piesa: Piesa::Pion,
            culoare: Culoare::Alb,
        });
    }
    // Piesele care sunt simetrice
    for (i, piesa) in [Piesa::Tura, Piesa::Cal, Piesa::Nebun].iter().enumerate() {
        tabla[0][i] = Some(Patratel {
            piesa: *piesa,
            culoare: Culoare::Negru,
        });
        tabla[0][7 - i] = Some(Patratel {
            piesa: *piesa,
            culoare: Culoare::Negru,
        });
        tabla[7][i] = Some(Patratel {
            piesa: *piesa,
            culoare: Culoare::Alb,
        });
        tabla[7][7 - i] = Some(Patratel {
            piesa: *piesa,
            culoare: Culoare::Alb,
        });
    }
    // restul de piese
    tabla[0][3] = Some(Patratel {
        piesa: Piesa::Regina,
        culoare: Culoare::Negru,
    });
    tabla[0][4] = Some(Patratel {
        piesa: Piesa::Rege,
        culoare: Culoare::Negru,
    });
    tabla[7][3] = Some(Patratel {
        piesa: Piesa::Regina,
        culoare: Culoare::Alb,
    });
    tabla[7][4] = Some(Patratel {
        piesa: Piesa::Rege,
        culoare: Culoare::Alb,
    });
}

// verifica daca celula (x, y) intra in tabla de joc
pub(crate) fn in_board(x: i32, y: i32) -> bool {
    x >= 0 && x < 8 && y >= 0 && y < 8
}

pub(crate) fn start_joc(tabla: &mut [[Option<Patratel>; 8]; 8]) {
    generare_tabla(tabla);
}

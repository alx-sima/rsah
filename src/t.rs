use crate::{Culoare, Patratel, Piesa};

/// Genereaza o tabla de sah clasica
pub(crate) fn generare_tabla(tabla: &mut [[Option<Patratel>; 8]; 8]) {
    // Pioni
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
    // Restul de piese
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

/// Verifica daca celula (i, j) intra in tabla de joc
pub(crate) fn in_board(i: i32, j: i32) -> bool {
    i >= 0 && i < 8 && j >= 0 && j < 8
}

pub(crate) fn valid_moves(tabla: &[[Option<Patratel>; 8]; 8], miscari: Vec<(i32, i32)>, i: usize, j: usize) -> Vec<(i32, i32)> {
    let mut rez = Vec::new();

    for (i, j) in miscari {
        
    }
    rez
}

use crate::{miscari, L};

pub(crate) mod editor;
pub(crate) mod game;
pub(crate) mod generare;
pub(crate) mod istoric;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum TipPiesa {
    Pion,
    Tura,
    Cal,
    Nebun,
    Regina,
    Rege,
}

impl std::fmt::Display for TipPiesa {
    /// Se afiseaza initiala piesei (in engleza) pt. istoricul miscarilor,
    /// de aceea, la pion nu se afiseaza nimic.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TipPiesa::Pion => "",
                TipPiesa::Tura => "R",
                TipPiesa::Cal => "N",
                TipPiesa::Nebun => "B",
                TipPiesa::Regina => "Q",
                TipPiesa::Rege => "K",
            }
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum Culoare {
    Alb,
    Negru,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Piesa {
    pub(crate) tip: TipPiesa,
    pub(crate) culoare: Culoare,
}

#[derive(Clone, Debug)]
pub(crate) struct Patratel {
    pub(crate) piesa: Option<Piesa>,
    pub(crate) atacat: Vec<(usize, usize)>,
}

pub(crate) type Tabla = [[Patratel; 8]; 8];

impl Default for Patratel {
    fn default() -> Self {
        Patratel {
            piesa: None,
            atacat: vec![],
        }
    }
}

/// Verifica daca celula (i, j) intra in tabla de joc
pub(crate) fn in_board(i: i32, j: i32) -> bool {
    i >= 0 && i < 8 && j >= 0 && j < 8
}

/// Returneaza coordonatele patratului unde se afla mouse-ul, sau
/// None => mouse-ul nu se afla in tabla de sah
pub(crate) fn get_square_under_mouse(
    ctx: &mut ggez::Context,
    reversed: bool,
) -> Option<(usize, usize)> {
    let cursor = ggez::input::mouse::position(ctx);
    let x = (cursor.x / L) as i32;
    let y = (cursor.y / L) as i32;
    if in_board(x, y) {
        if reversed {
            println!("{}, {}", x, 7 - y);
            Some((x as usize, 7 - y as usize))
        } else {
            Some((x as usize, y as usize))
        }
    } else {
        None
    }
}

/// Marcheaza patratele atacate de piesa de la (i, j)
pub(crate) fn set_atacat_field(tabla: &mut [[Patratel; 8]; 8], i: usize, j: usize) {
    for (x, y) in miscari::get_miscari(tabla, i as i32, j as i32, true) {
        tabla[x][y].atacat.push((i, j));
    }
}

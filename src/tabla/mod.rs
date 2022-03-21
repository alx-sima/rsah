/// gasirea patratelelor atacate de o anumita piesa,
/// sau a caror modificare o pot afecta
pub(crate) mod miscari;
/// desenarea tablei si a pieselor de pe aceasta
pub(crate) mod draw;
/// amplasarea pieselor (in modul editor)
/// pt. a crea scenarii de joc
pub(crate) mod editor;
/// mutarea pieselor pe parcursul unui joc
pub(crate) mod game;
/// aranjarea pieselor pe tabla
/// inaintea inceperii jocului
pub(crate) mod generare;
/// procesarea clickurilor si centrarea tablei
/// in functie de dimensiunile aplicatiei
pub(crate) mod input;
/// deducerea notatiei algebrice
/// dintr-o miscare (sau invers)
pub(crate) mod notatie;

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
    /// Se afiseaza initiala piesei (in engleza) pt. istoricul
    /// miscarilor, de aceea la pion nu se afiseaza nimic.
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

/// culoarea unei piese (+ a jucatorului care o detine);
/// folosita si pentru a retine al cui este randul sa mute
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum Culoare {
    Alb,
    Negru,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Piesa {
    /// ce piesa e
    pub(crate) tip: TipPiesa,
    /// Oare?
    pub(crate) culoare: Culoare,
    /// daca piesa a mai fost mutata
    /// (pt rocada, en passant etc.)
    pub(crate) mutat: bool,
}

impl Piesa {
    pub(crate) fn new(tip: TipPiesa, culoare: Culoare) -> Piesa {
        Piesa {
            tip,
            culoare,
            mutat: false,
        }
    }
}

/// Un patrat de pe tabla, care poate avea o **piesa**;
/// retine si pozitiile *(i, j)* pieselor care il ataca.
#[derive(Clone, Debug)]
pub(crate) struct Patratel {
    /// piesa de pe acel patrat (daca exista)
    pub(crate) piesa: Option<Piesa>,
    /// pozitiile (i, j) pieselor care ataca acest patrat
    pub(crate) atacat: Vec<(usize, usize)>,
}

/// O **tabla** de sah este o matrice 8x8 de *patratele*.
pub(crate) type Tabla = [[Patratel; 8]; 8];

impl Default for Patratel {
    fn default() -> Self {
        Patratel {
            piesa: None,
            atacat: vec![],
        }
    }
}

/// Marcheaza patratele atacate de piesa de la *(i, j)*.
pub(crate) fn set_atacat_field(tabla: &mut Tabla, i: usize, j: usize) {
    for (x, y) in miscari::get_miscari(tabla, i as i32, j as i32, true) {
        tabla[x][y].atacat.push((i, j));
    }
}

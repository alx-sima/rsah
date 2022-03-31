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
/// gasirea patratelelor atacate de o anumita piesa,
/// sau a caror modificare o pot afecta
pub(crate) mod miscari;
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

/// o piesa de pe tabla
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Piesa {
    /// ce piesa e
    pub(crate) tip: TipPiesa,
    /// Oare?
    pub(crate) culoare: Culoare,
    /// daca piesa a mai fost mutata
    /// (pt rocada, en passant etc.)
    /// DEPRECATED (probabil) pt ca ne vom putea uita in pozitii anterioare
    pub(crate) mutat: bool,
    /// Pozitiile pe care piesa a fost inainte
    pub(crate) pozitii_anterioare: Vec<PozitieSafe>,
}

impl Piesa {
    pub(crate) fn new(tip: TipPiesa, culoare: Culoare) -> Piesa {
        Piesa {
            pozitii_anterioare: vec![],
            mutat: false,
            culoare,
            tip,
        }
    }
}

/// Un patrat de pe tabla, care poate avea o **piesa**;
/// retine si pozitiile *(i, j)* pieselor care il ataca.
#[derive(Clone, Debug)]
pub(crate) struct Patratel {
    /// piesa de pe acel patrat (daca exista)
    pub(crate) piesa: Option<Piesa>,
    /// pozitiile (i, j) pieselor care ataca acest patrat (indiferent de culoare)
    pub(crate) atacat: Vec<PozitieSafe>,
    /// piesele care, prin modificarea acestui patrat, pot fi afectate
    pub(crate) afecteaza: Vec<PozitieSafe>,
}

impl Default for Patratel {
    fn default() -> Self {
        Patratel {
            afecteaza: vec![],
            atacat: vec![],
            piesa: None,
        }
    }
}

/// O **tabla** de sah este o matrice 8x8 de *patratele*.
pub(crate) type Tabla = [[Patratel; 8]; 8];

/// Coordonatele patratelor care sunt garantate sa existe si sa fie valide.
pub(crate) type PozitieSafe = (usize, usize);

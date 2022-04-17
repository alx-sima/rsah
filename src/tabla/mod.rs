use serde::{Deserialize, Serialize};

/// Tabla de sah + informatii despre joc.
#[derive(Clone, Default)]
pub(crate) struct Tabla {
    /// Istoric miscari
    pub(crate) istoric: Vec<String>,
    /// Tabla de sah
    pub(crate) mat: MatTabla,
    pub(crate) match_state: MatchState,
    /// Ultima miscare efectuata (daca exista)
    pub(crate) ultima_miscare: Option<(Pozitie, Pozitie)>,
}

impl Tabla {
    /// Returneaza patratul de pe `poz`.
    pub(crate) fn at(&self, poz: Pozitie) -> &Patratel {
        &self.mat[poz.0][poz.1]
    }

    /// Returneaza o referinta mutable a patratului de pe `poz`.
    pub(crate) fn get(&mut self, poz: Pozitie) -> &mut Patratel {
        &mut self.mat[poz.0][poz.1]
    }
}

/// O matrice 8x8 de [Patratel].
pub(crate) type MatTabla = [[Patratel; 8]; 8];

/// Un patrat de pe tabla, care poate avea o [Piesa];
/// retine si pozitia pieselor care il ataca.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub(crate) struct Patratel {
    /// piesa de pe acel patrat (daca exista)
    pub(crate) piesa: Option<Piesa>,
    /// pozitiile pieselor care ataca acest patrat (indiferent de culoare)
    #[serde(skip)]
    pub(crate) atacat: Vec<Pozitie>,
    /// piesele care, prin modificarea acestui patrat, pot fi afectate
    #[serde(skip)]
    pub(crate) afecteaza: Vec<Pozitie>,
}

/// O piesa de sah.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct Piesa {
    /// ce piesa e
    pub(crate) tip: TipPiesa,
    /// Oare?
    pub(crate) culoare: Culoare,
    /// daca piesa a mai fost mutata
    /// (pt rocada, en passant etc.)
    /// DEPRECATED (probabil) pt ca ne vom putea uita in pozitii anterioare
    #[serde(skip)]
    pub(crate) mutat: bool,
    /// Pozitiile pe care piesa a fost inainte
    #[serde(skip)]
    pub(crate) pozitii_anterioare: Vec<Pozitie>,
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

/// culoarea unei piese (+ a jucatorului care o detine);
/// folosita si pentru a retine al cui este randul sa mute
/// FIXME: :)))
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) enum Culoare {
    /// Culoarea neagra.
    Alb,
    /// Culoarea alba.
    Negru,
}

/// Tipul unei piese.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
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

/// In ce stadiu se poate afla jocul de sah.
#[derive(Clone, PartialEq)]
pub(crate) enum MatchState {
    /// Normal
    Playing,
    /// Negru castiga
    AlbEMat,
    /// Alb castiga
    NegruEMat,
    /// Egalitate
    Pat,
}

impl Default for MatchState {
    /// Incepe jocul cu stadiul normal.
    fn default() -> Self {
        MatchState::Playing
    }
}

/// Coordonatele [Patratel]or de pe [MatTabla]
/// care sunt garantate sa existe si sa fie valide.
pub(crate) type Pozitie = (usize, usize);

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

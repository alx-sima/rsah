use ggez::{event::MouseButton, filesystem};
use regex::Regex;

use super::{input, Culoare, MatTabla, Patratel, Piesa, TipPiesa};

use lazy_static::lazy_static;

/// Amplaseaza piesa 'tip' a jucatorului 'culoare' la (i, j)
pub(crate) fn place(
    tabla: &mut [[Patratel; 8]; 8],
    i: usize,
    j: usize,
    tip: TipPiesa,
    culoare: Culoare,
) {
    if tabla[i][j].piesa.is_none() {
        tabla[i][j].piesa = Some(Piesa::new(tip, culoare));
    }
}

/// Sterge piesa de pe pozitia `(i, j)`
pub(crate) fn delete(tabla: &mut [[Patratel; 8]; 8], i: usize, j: usize) {
    tabla[i][j] = Default::default();
}

/// Handle pt. clickurile facute in editor:
/// *click* => pune piesa *alba*;
/// *click dr.* => pune piesa *neagra*;
/// *clic mij.* => *sterge* piesa.
pub(crate) fn editor_handler(
    ctx: &mut ggez::Context,
    tabla: &mut [[Patratel; 8]; 8],
    piesa_selectata_editor: TipPiesa,
) {
    // la un click, amplaseaza piesa alba
    if ggez::input::mouse::button_pressed(ctx, MouseButton::Left) {
        // reversed va fi mereu false pt ca nu esti masochist sa editezi tabla invers
        if let Some((j, i)) = input::get_mouse_square(ctx, false) {
            place(tabla, i, j, piesa_selectata_editor, Culoare::Alb);
        }
    // la click-dreapta, amplaseaza piesa neagra
    } else if ggez::input::mouse::button_pressed(ctx, MouseButton::Right) {
        if let Some((j, i)) = input::get_mouse_square(ctx, false) {
            place(tabla, i, j, piesa_selectata_editor, Culoare::Negru);
        }
    // la click pe rotita, sterge pionul
    } else if ggez::input::mouse::button_pressed(ctx, MouseButton::Middle) {
        if let Some((j, i)) = input::get_mouse_square(ctx, false) {
            delete(tabla, i, j);
        }
    }
}

/// Incarca layoutul din fisierul `path`.
/// Returneaza `None` daca fisierul nu poate fi incarcat.
pub(crate) fn load_file(ctx: &ggez::Context, path: &String) -> Option<MatTabla> {
    let f = filesystem::open(ctx, path).unwrap();
    serde_json::from_reader(f).ok()
}

/// Cauta toate fisierele cu extensia `.json`
/// din filesistem care contin un layout valid.
pub(crate) fn list_files(ctx: &ggez::Context) -> Vec<String> {
    lazy_static! {
        static ref PATTERN: Regex = Regex::new(r"(.*)\.json$").unwrap();
    };
    let mut res = vec![];

    if let Ok(files) = filesystem::read_dir(ctx, "/") {
        for file in files {
            let file = file.to_str().unwrap();
            if PATTERN.is_match(file) && layout_valid(ctx, &file.to_owned()) {
                res.push(file.to_owned());
            }
        }
    }
    res
}

/// Verifica daca layoutul din fisierul `path` este valid.
fn layout_valid(ctx: &ggez::Context, path: &String) -> bool {
    load_file(ctx, path).is_some()
}

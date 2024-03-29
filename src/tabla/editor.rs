use ggez::{event::MouseButton, filesystem};
use regex::Regex;

use super::{input, Culoare, MatTabla, Piesa, TipPiesa};

use lazy_static::lazy_static;

/// Handle pt. clickurile facute in editor:
///  - `click`: pune piesa `alba`;
///  - `click dr.`: pune piesa `neagra`;
///  - `clic mij.`: sterge piesa.
pub(crate) fn editor_handler(ctx: &mut ggez::Context, tabla: &mut MatTabla, piesa_sel: TipPiesa) {
    // la un click, amplaseaza piesa alba
    if ggez::input::mouse::button_pressed(ctx, MouseButton::Left) {
        // reversed va fi mereu false
        if let Some((i, j)) = input::get_mouse_square(ctx, false) {
            place(tabla, i, j, piesa_sel, Culoare::Alb);
        }
        // la click-dreapta, amplaseaza piesa neagra
    } else if ggez::input::mouse::button_pressed(ctx, MouseButton::Right) {
        if let Some((i, j)) = input::get_mouse_square(ctx, false) {
            place(tabla, i, j, piesa_sel, Culoare::Negru);
        }
        // la click pe rotita, sterge pionul
    } else if ggez::input::mouse::button_pressed(ctx, MouseButton::Middle) {
        if let Some((i, j)) = input::get_mouse_square(ctx, false) {
            delete(tabla, i, j);
        }
    }
}

/// Cauta toate fisierele cu extensia `.json`
/// din filesistem care contin un layout valid.
pub(crate) fn list_files(ctx: &ggez::Context) -> Vec<String> {
    lazy_static! {
        static ref PATTERN: Regex = Regex::new(r"/(.*)\.json$").unwrap();
    };
    let mut res = vec![];

    if let Ok(files) = filesystem::read_dir(ctx, "/") {
        for file in files {
            let file = file.to_str().unwrap();

            if let Some(capture) = PATTERN.captures_iter(file).next() {
                if let Some(name) = capture.get(1) {
                    let name = name.as_str().to_string();
                    if layout_valid(ctx, &name) {
                        res.push(name);
                    }
                }
            }
        }
    }
    res
}

/// Plaseaza piesa `tip` a jucatorului `culoare` la `(i, j)`.
pub(crate) fn place(tabla: &mut MatTabla, i: usize, j: usize, tip: TipPiesa, culoare: Culoare) {
    if tabla[i][j].piesa.is_none() {
        tabla[i][j].piesa = Some(Piesa::new(tip, culoare));
    }
}

/// Sterge piesa de pe pozitia `(i, j)`.
pub(crate) fn delete(tabla: &mut MatTabla, i: usize, j: usize) {
    tabla[i][j] = Default::default();
}

/// Verifica daca layoutul din fisierul `path` este valid.
fn layout_valid(ctx: &ggez::Context, path: &String) -> bool {
    load_file(ctx, path).is_some()
}

/// Incarca layoutul din fisierul `path`.
///
/// Returneaza `None` daca fisierul nu poate fi incarcat.
pub(crate) fn load_file(ctx: &ggez::Context, path: &String) -> Option<MatTabla> {
    let f = filesystem::open(ctx, &format!("/{}.json", path)).unwrap();
    serde_json::from_reader(f).ok()
}

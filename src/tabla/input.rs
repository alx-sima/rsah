use ggez::graphics;

use super::Pozitie;

/// Verifica daca celula (i, j) intra in tabla de joc
pub(crate) fn in_board(i: i32, j: i32) -> bool {
    (0..8).contains(&i) && (0..8).contains(&j)
}

/// Calculeaza latura unui patratel de pe tabla de sah
/// si paddingul la stanga/sus, pentru ca tabla sa fie
/// la centru-dreapta (in stanga se rezerva loc pt gui).
///
/// Returneaza `(latura, offset x, offset y)`.
pub(crate) fn get_dimensiuni_tabla(ctx: &ggez::Context) -> (f32, f32, f32) {
    // Spatiul rezervat pt gui
    const RESERVED: f32 = 100.0;

    let (width, height) = graphics::drawable_size(ctx);
    if width - height >= RESERVED {
        // Orientare landscape
        let w_ofs = width - height;
        (height / 8.0, w_ofs, 0.0)
    } else {
        // Orientare portret
        let width = width - RESERVED;
        let h_ofs = (height - width) / 2.0;
        (width / 8.0, RESERVED, h_ofs)
    }
}

/// Returneaza coordonatele patratului in care se afla mouse-ul
/// sau `None`, daca mouse-ul nu se afla in tabla de sah.
pub(crate) fn get_mouse_square(ctx: &mut ggez::Context, reversed: bool) -> Option<Pozitie> {
    let (l, x_ofs, y_ofs) = get_dimensiuni_tabla(ctx);
    let cursor = ggez::input::mouse::position(ctx);

    // "Translateaza" tabla a.i. sa aiba (0, 0) in stanga sus
    let nx = cursor.x - x_ofs;
    let ny = cursor.y - y_ofs;
    if nx < 0.0 || ny < 0.0 {
        return None;
    }

    let x = (nx / l) as i32;
    let y = (ny / l) as i32;
    if in_board(x, y) {
        if reversed {
            Some((7 - x as usize, 7 - y as usize))
        } else {
            Some((x as usize, y as usize))
        }
    } else {
        None
    }
}

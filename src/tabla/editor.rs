use ggez::event::MouseButton;

use super::{Culoare, Patratel, Piesa, TipPiesa};

/// Amplaseaza piesa 'tip' a jucatorului 'culoare' la (i, j)
pub(crate) fn place(
    tabla: &mut [[Patratel; 8]; 8],
    i: usize,
    j: usize,
    tip: TipPiesa,
    culoare: Culoare,
) {
    if tabla[i][j].piesa.is_none() {
        tabla[i][j].piesa = Some(Piesa { tip, culoare });
    }
}

/// Sterge piesa de pe pozitia (i, j)
pub(crate) fn delete(tabla: &mut [[Patratel; 8]; 8], i: usize, j: usize) {
    tabla[i][j] = Default::default();
}

/// Handle pt. clickurile facute in editor.
/// Click => pune piesa alba;
/// Click dr. => pune piesa neagra;
/// Clic mij. => sterge piesa.
pub(crate) fn player_turn(
    ctx: &mut ggez::Context,
    tabla: &mut [[Patratel; 8]; 8],
    piesa_selectata_editor: TipPiesa,
) {
    // la un click, amplaseaza piesa alba
    if ggez::input::mouse::button_pressed(ctx, MouseButton::Left) {
        if let Some((j, i)) = super::get_square_under_mouse(ctx) {
            place(tabla, i, j, piesa_selectata_editor, Culoare::Alb);
        }
    // la click-dreapta, amplaseaza piesa neagra
    } else if ggez::input::mouse::button_pressed(ctx, MouseButton::Right) {
        if let Some((j, i)) = super::get_square_under_mouse(ctx) {
            place(tabla, i, j, piesa_selectata_editor, Culoare::Negru);
        }
    // la click pe rotita, sterge pionul
    } else if ggez::input::mouse::button_pressed(ctx, MouseButton::Middle) {
        if let Some((j, i)) = super::get_square_under_mouse(ctx) {
            delete(tabla, i, j);
        }
    }
}

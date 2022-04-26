use ggez::graphics::{self, MeshBuilder};

use crate::{State, TipMutare};

use super::{input::get_dimensiuni_tabla, miscari::get_poz_rege, sah::e_in_sah};

/// Deseneaza tabla de joc.
pub(crate) fn board(ctx: &mut ggez::Context) -> ggez::GameResult {
    let white = graphics::Color::from_rgb(189, 114, 49);
    let black = graphics::Color::from_rgb(102, 52, 9);

    let (l, x_ofs, y_ofs) = get_dimensiuni_tabla(ctx);
    // TODO: mesh mai scurt?
    let mesh = MeshBuilder::new()
        .rectangle(
            graphics::DrawMode::fill(),
            graphics::Rect::new(0.0, 0.0, l, l),
            white,
        )?
        .rectangle(
            graphics::DrawMode::fill(),
            graphics::Rect::new(l, 0.0, l, l),
            black,
        )?
        .rectangle(
            graphics::DrawMode::fill(),
            graphics::Rect::new(0.0, l, l, l),
            black,
        )?
        .rectangle(
            graphics::DrawMode::fill(),
            graphics::Rect::new(l, l, l, l),
            white,
        )?
        .build(ctx)?;

    // Aplica meshurile
    for i in 0..4 {
        for j in 0..4 {
            graphics::draw(
                ctx,
                &mesh,
                ([x_ofs + j as f32 * 2.0 * l, y_ofs + i as f32 * 2.0 * l],),
            )?;
        }
    }
    Ok(())
}

/// Deseneaza piesele de pe tabla.
pub(crate) fn pieces(state: &State, ctx: &mut ggez::Context) -> ggez::GameResult {
    let (l, x_ofs, y_ofs) = get_dimensiuni_tabla(ctx);

    for i in 0..8 {
        for j in 0..8 {
            let (i_pies, j_pies) = if state.guest { (7 - i, 7 - j) } else { (i, j) };
            if let Some(patratel) = &state.tabla.at((i_pies, j_pies)).piesa {
                let img = graphics::Image::new(
                    ctx,
                    &format!("/images/{:?}/{:?}.png", patratel.culoare, patratel.tip),
                )?;

                // Cat trebuie scalate piesele
                // pentru a incapea intr-un patratel.
                let w = l * 0.85 / img.width() as f32;
                let h = l * 0.85 / img.height() as f32;

                graphics::draw(
                    ctx,
                    &img,
                    graphics::DrawParam::default()
                        .dest([
                            x_ofs + j as f32 * l + 0.07 * l,
                            y_ofs + i as f32 * l + 0.07 * l,
                        ])
                        .scale([w, h]),
                )?;
            }
        }
    }
    Ok(())
}

/// Afiseaza ultima miscare, daca regele e in sah, piesa
/// selectata si miscarile disponibile ale acesteia.
// TODO: sparge in mai multe functii
pub(crate) fn attack(state: &State, ctx: &mut ggez::Context) -> ggez::GameResult {
    let (l, x_ofs, y_ofs) = get_dimensiuni_tabla(ctx);
    let guest = state.guest;

    // Se coloreaza cu albastru ultima miscare
    if let Some((src, dest)) = state.tabla.ultima_miscare {
        let patrat_albastru = build_square(ctx, l, graphics::Color::from_rgba(255, 255, 0, 90))?;

        let (x_src, y_src) = adjust_for_multiplayer(src.1, src.0, l, guest);
        let (x_dest, y_dest) = adjust_for_multiplayer(dest.1, dest.0, l, guest);
        graphics::draw(ctx, &patrat_albastru, ([x_ofs + x_src, y_ofs + y_src],))?;
        graphics::draw(ctx, &patrat_albastru, ([x_ofs + x_dest, y_ofs + y_dest],))?;
    }

    // Se coloreaza cu rosu regele, daca e in sah.
    if e_in_sah(&state.tabla, state.turn) {
        let patrat_rosu = build_square(ctx, l, graphics::Color::from_rgba(255, 0, 0, 170))?;

        let (x, y) = get_poz_rege(&state.tabla, state.turn);
        let (x, y) = adjust_for_multiplayer(y, x, l, guest);
        graphics::draw(ctx, &patrat_rosu, ([x_ofs + x, y_ofs + y],))?;
    }

    // Se coloreaza cu verde piesa selectata si
    // cu galben mutarile posibile ale acesteia.
    if let Some((x, y)) = state.piesa_sel {
        let patrat_galben = MeshBuilder::new()
            .circle(
                graphics::DrawMode::fill(),
                [l / 2.0, l / 2.0],
                l / 8.0,
                0.01,
                graphics::Color::from_rgba(0, 0, 0, 127),
            )?
            .build(ctx)?;
        let patrat_galben_gol = MeshBuilder::new()
            .circle(
                graphics::DrawMode::stroke(5.0),
                [l / 2.0, l / 2.0],
                l / 2.0,
                0.01,
                graphics::Color::from_rgba(0, 0, 0, 127),
            )?
            .build(ctx)?;
        let patrat_verde = build_square(ctx, l, graphics::Color::from_rgba(255, 255, 0, 90))?;

        for mutare in &state.miscari_disponibile {
            let (x, y) = adjust_for_multiplayer(mutare.dest.1, mutare.dest.0, l, guest);
            match mutare.tip {
                TipMutare::Normal | TipMutare::Rocada(_) => {
                    graphics::draw(ctx, &patrat_galben, ([x_ofs + x, y_ofs + y],))?
                }
                TipMutare::Captura | TipMutare::EnPassant(_) => {
                    graphics::draw(ctx, &patrat_galben_gol, ([x_ofs + x, y_ofs + y],))?
                }
            }
        }

        let (x, y) = adjust_for_multiplayer(y, x, l, guest);
        graphics::draw(ctx, &patrat_verde, ([x_ofs + x, y_ofs + y],))?;
    }
    Ok(())
}

/// Ajusteaza coordonatele pt multiplayer (cand tabla este invers).
fn adjust_for_multiplayer(x: usize, y: usize, l: f32, guest: bool) -> (f32, f32) {
    let x = x as f32;
    let y = y as f32;

    if guest {
        ((7.0 - x) * l, (7.0 - y) * l)
    } else {
        (x * l, y * l)
    }
}

/// Construieste un patrat cu `culoarea` si dimensiunea `l` specificate.
fn build_square(
    ctx: &mut ggez::Context,
    l: f32,
    colour: graphics::Color,
) -> Result<graphics::Mesh, ggez::GameError> {
    MeshBuilder::new()
        .rectangle(
            graphics::DrawMode::fill(),
            graphics::Rect::new(0.0, 0.0, l, l),
            colour,
        )?
        .build(ctx)
}

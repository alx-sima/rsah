use ggez::graphics::{self, MeshBuilder};

use crate::State;

use super::input::get_dimensiuni_tabla;

/// Deseneaza tabla de joc
pub(crate) fn board(ctx: &mut ggez::Context) -> ggez::GameResult {
    let (l, x_ofs, y_ofs) = get_dimensiuni_tabla(ctx);
    // TODO: pune culorile din gosah?
    let negru = graphics::Color::from_rgb(0, 0, 0);
    let alb = graphics::Color::from_rgb(255, 255, 255);

    // TODO: mesh mai scurt?
    let mesh = MeshBuilder::new()
        .rectangle(
            graphics::DrawMode::fill(),
            graphics::Rect::new(0.0, 0.0, l, l),
            alb,
        )?
        .rectangle(
            graphics::DrawMode::fill(),
            graphics::Rect::new(l, 0.0, l, l),
            negru,
        )?
        .rectangle(
            graphics::DrawMode::fill(),
            graphics::Rect::new(0.0, l, l, l),
            negru,
        )?
        .rectangle(
            graphics::DrawMode::fill(),
            graphics::Rect::new(l, l, l, l),
            alb,
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

/// Deseneaza piesele
pub(crate) fn pieces(state: &State, ctx: &mut ggez::Context) -> ggez::GameResult {
    let (l, x_ofs, y_ofs) = get_dimensiuni_tabla(ctx);

    for i in 0..8 {
        for j in 0..8 {
            let (i_pies, j_pies) = if state.guest { (7 - i, 7 - j) } else { (i, j) };
            if let Some(patratel) = &state.tabla.mat[i_pies][j_pies].piesa {
                let img = graphics::Image::new(
                    ctx,
                    &format!("/images/{:?}/{:?}.png", patratel.culoare, patratel.tip),
                )?;
                // FIXME: marimi mai ok
                graphics::draw(
                    ctx,
                    &img,
                    graphics::DrawParam::default()
                        .dest([x_ofs + j as f32 * l + 6.0, y_ofs + i as f32 * l + 5.0])
                        .scale([0.5, 0.5]),
                )?;
            }
        }
    }
    Ok(())
}

pub(crate) fn attack(game_state: &State, ctx: &mut ggez::Context) -> ggez::GameResult {
    let (l, x_ofs, y_ofs) = get_dimensiuni_tabla(ctx);
    let guest = game_state.guest;

    // Se coloreaza cu albastru ultima miscare
    if let Some((src, dest)) = game_state.tabla.ultima_miscare {
        let patrat_albastru = MeshBuilder::new()
            .rectangle(
                graphics::DrawMode::fill(),
                graphics::Rect::new(0.0, 0.0, l, l),
                // FIXME: o culoare mai frumoasa
                graphics::Color::BLUE,
            )?
            .build(ctx)?;

        let (x_src, y_src) = draw_pos_multiplayer(src.1 as f32, src.0 as f32, l, guest);
        let (x_dest, y_dest) = draw_pos_multiplayer(dest.1 as f32, dest.0 as f32, l, guest);
        graphics::draw(ctx, &patrat_albastru, ([x_ofs + x_src, y_ofs + y_src],))?;
        graphics::draw(ctx, &patrat_albastru, ([x_ofs + x_dest, y_ofs + y_dest],))?;
    }

    // Se coloreaza cu verde piesa selectata si
    // cu galben mutarile posibile ale acesteia.
    if let Some((x, y)) = game_state.piesa_sel {
        // FIXME: CURSED??? sigur se pot face mai usor patrate
        let patrat_galben = MeshBuilder::new()
            .rectangle(
                graphics::DrawMode::fill(),
                graphics::Rect::new(0.0, 0.0, l, l),
                graphics::Color::YELLOW,
            )?
            .build(ctx)?;

        let patrat_verde = MeshBuilder::new()
            .rectangle(
                graphics::DrawMode::fill(),
                graphics::Rect::new(0.0, 0.0, l, l),
                graphics::Color::GREEN,
            )?
            .build(ctx)?;

        for (i, j) in &game_state.miscari_disponibile {
            let (x, y) = draw_pos_multiplayer(*j as f32, *i as f32, l, guest);
            graphics::draw(ctx, &patrat_galben, ([x_ofs + x, y_ofs + y],))?;
        }

        let (x, y) = draw_pos_multiplayer(y as f32, x as f32, l, guest);
        graphics::draw(ctx, &patrat_verde, ([x_ofs + x, y_ofs + y],))?;
    }
    Ok(())
}

/// Ajusteaza coordonatele pt multiplayer (cand tabla este invers).
fn draw_pos_multiplayer(x: f32, y: f32, l: f32, guest: bool) -> (f32, f32) {
    if guest {
        ((7.0 - x) * l, (7.0 - y) * l)
    } else {
        (x * l, y * l)
    }
}

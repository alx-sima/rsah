use ggez::graphics::{self, MeshBuilder};

use crate::{State, L};

/// Deseneaza tabla de joc
// FIXME: deseneaza tabla in functie de dimensiunile ecranului
pub(crate) fn board(ctx: &mut ggez::Context) -> ggez::GameResult {
    // TODO: pune culorile din gosah?
    let negru = graphics::Color::from_rgb(0, 0, 0);
    let alb = graphics::Color::from_rgb(255, 255, 255);

    // TODO: mesh mai scurt?
    let mesh = graphics::MeshBuilder::new()
        .rectangle(
            graphics::DrawMode::fill(),
            graphics::Rect::new(0.0, 0.0, L, L),
            alb,
        )?
        .rectangle(
            graphics::DrawMode::fill(),
            graphics::Rect::new(L, 0.0, L, L),
            negru,
        )?
        .rectangle(
            graphics::DrawMode::fill(),
            graphics::Rect::new(0.0, L, L, L),
            negru,
        )?
        .rectangle(
            graphics::DrawMode::fill(),
            graphics::Rect::new(L, L, L, L),
            alb,
        )?
        .build(ctx)?;

    // Aplica meshurile
    for i in 0..4 {
        for j in 0..4 {
            graphics::draw(ctx, &mesh, ([j as f32 * 2.0 * L, i as f32 * 2.0 * L],))?;
        }
    }
    Ok(())
}

/// Deseneaza piesele
pub(crate) fn pieces(state: &State, ctx: &mut ggez::Context) -> ggez::GameResult {
    for i in 0..8 {
        for j in 0..8 {
            let (i_pies, j_pies) = if state.guest { (7 - i, 7 - j) } else { (i, j) };
            if let Some(patratel) = &state.tabla[i_pies][j_pies].piesa {
                let img = graphics::Image::new(
                    ctx,
                    &format!("/images/{:?}/{:?}.png", patratel.culoare, patratel.tip),
                )?;
                // FIXME: marimi mai ok
                graphics::draw(
                    ctx,
                    &img,
                    graphics::DrawParam::default()
                        .dest([j as f32 * L + 5.0, i as f32 * L + 5.0])
                        .scale([0.35, 0.35]),
                )?;
            }
        }
    }
    Ok(())
}

pub(crate) fn attack(game_state: &State, ctx: &mut ggez::Context) -> ggez::GameResult {
    if let Some((x, y)) = game_state.piesa_sel {
        // TODO: CURSED??? sigur se pot face mai usor patrate
        let patrat_galben = MeshBuilder::new()
            .rectangle(
                graphics::DrawMode::fill(),
                graphics::Rect::new(0.0, 0.0, L, L),
                graphics::Color::YELLOW,
            )?
            .build(ctx)?;
        let patrat_verde = MeshBuilder::new()
            .rectangle(
                graphics::DrawMode::fill(),
                graphics::Rect::new(0.0, 0.0, L, L),
                graphics::Color::GREEN,
            )?
            .build(ctx)?;
        for (i, j) in &game_state.miscari_disponibile {
            let (x, y) = if game_state.guest {
                ((7 - *j) as f32 * L, (7 - *i) as f32 * L)
            } else {
                (*j as f32 * L, *i as f32 * L)
            };
            graphics::draw(ctx, &patrat_galben, ([x, y],))?;
        }
        if game_state.guest {
            graphics::draw(
                ctx,
                &patrat_verde,
                ([(7 - y) as f32 * L, (7 - x) as f32 * L],),
            )?;
        } else {
            graphics::draw(ctx, &patrat_verde, ([y as f32 * L, x as f32 * L],))?;
        }
    }
    Ok(())
}

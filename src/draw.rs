use ggez::graphics::{self, MeshBuilder};

use crate::State;

/// Calculeaza latura unui patratel de pe tabla de sah si paddingul la stanga/sus,
/// pentru ca tabla sa fie centrata.
/// Returneaza *(latura, offset x, offset y)*.
pub(crate) fn get_dimensiuni_tabla(ctx: &ggez::Context) -> (f32, f32, f32) {
    let (width, height) = graphics::drawable_size(ctx);
    // Portrait
    if height > width {
        let h_ofs = (height - width) / 2.0;
        (width / 8.0, 0.0, h_ofs)
    // Landscape
    } else {
        let w_ofs = (width - height) / 2.0;
        (height / 8.0, w_ofs, 0.0)
    }
}

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
                        .dest([x_ofs + j as f32 * l + 5.0, y_ofs + i as f32 * l + 5.0])
                        .scale([0.35, 0.35]),
                )?;
            }
        }
    }
    Ok(())
}

pub(crate) fn attack(game_state: &State, ctx: &mut ggez::Context) -> ggez::GameResult {
    let (l, x_ofs, y_ofs) = get_dimensiuni_tabla(ctx);
    if let Some((x, y)) = game_state.piesa_sel {
        // TODO: CURSED??? sigur se pot face mai usor patrate
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
            let (x, y) = if game_state.guest {
                ((7 - *j) as f32 * l, (7 - *i) as f32 * l)
            } else {
                (*j as f32 * l, *i as f32 * l)
            };
            graphics::draw(ctx, &patrat_galben, ([x_ofs + x, y_ofs + y],))?;
        }
        if game_state.guest {
            graphics::draw(
                ctx,
                &patrat_verde,
                ([x_ofs + (7 - y) as f32 * l, y_ofs + (7 - x) as f32 * l],),
            )?;
        } else {
            graphics::draw(
                ctx,
                &patrat_verde,
                ([x_ofs + y as f32 * l, y_ofs + x as f32 * l],),
            )?;
        }
    }
    Ok(())
}

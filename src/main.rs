use ggez::{event::MouseButton, graphics, Context};
use ggez_egui::EguiBackend;

struct State {
    game_state: u8,
    egui_backend: EguiBackend,
}

impl ggez::event::EventHandler<ggez::GameError> for State {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        let egui_ctx = self.egui_backend.ctx();
        // FIXME: fixeaza meniul pe ecran
        match self.game_state {
            // Main menu
            0 => {
                egui::Window::new("egui")
                    .title_bar(false)
                    .show(&egui_ctx, |ui| {
                        if ui.button("START").clicked() {
                            self.game_state = 1;
                        }
                        if ui.button("Editor").clicked() {
                            self.game_state = 2;
                        }
                        if ui.button("Quit").clicked() {
                            ggez::event::quit(ctx);
                        }
                    });
            }
            // Game
            1 => {
                egui::Window::new("egui-game")
                    .title_bar(false)
                    .show(&egui_ctx, |ui| {
                        if ui.button("help").clicked() {
                            // TODO: help text pt joc
                        }
                        if ui.button("back").clicked() {
                            self.game_state = 0;
                        }
                    });
            }
            // Editor
            2 => {
                egui::Window::new("egui-editor")
                    .title_bar(false)
                    .show(&egui_ctx, |ui| {
                        if ui.button("help").clicked() {
                            // TODO: help text pt editor
                        }
                        if ui.button("back").clicked() {
                            self.game_state = 0;
                        }
                    });
            }
            _ => {}
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::clear(ctx, graphics::Color::BLACK);

        if self.game_state != 0 {
            draw_board(ctx)?;
        }

        graphics::draw(ctx, &self.egui_backend, ([0.0, 0.0],))?;
        graphics::present(ctx)
    }

    // pt ca egui sa captureze mouseul
    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
        self.egui_backend.input.mouse_button_down_event(button);
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: MouseButton, _x: f32, _y: f32) {
        self.egui_backend.input.mouse_button_up_event(button);
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32) {
        self.egui_backend.input.mouse_motion_event(x, y);
    }
}

fn main() {
    let state = State {
        game_state: 0,
        egui_backend: EguiBackend::default(),
    };
    let c = ggez::conf::Conf::new();
    let (ctx, event_loop) = ggez::ContextBuilder::new("rsah", "bamse")
        .default_conf(c)
        .build()
        .unwrap();

    ggez::event::run(ctx, event_loop, state);
}

// Deseneaza tabla de joc
// FIXME: deseneaza tabla in functie de dimensiunile ecranului
fn draw_board(ctx: &mut ggez::Context) -> ggez::GameResult {
    const L: f32 = 100.0;
    let negru = graphics::Color::from_rgb(0, 0, 0);
    let alb = graphics::Color::from_rgb(255, 255, 255);

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

    for i in 0..4 {
        for j in 0..4 {
            graphics::draw(ctx, &mesh, ([j as f32 * 2.0 * L, i as f32 * 2.0 * L],))?;
        }
    }
    Ok(())
}

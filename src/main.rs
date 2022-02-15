use ggez::{event::MouseButton, graphics, Context};
use ggez_egui::EguiBackend;

#[derive(PartialEq)]
enum GameState {
    MainMenu,
    Game,
    Editor,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Piesa {
    Pion,
    Tura,
    Cal,
    Nebun,
    Regina,
    Rege,
}

#[derive(Clone, Copy, Debug)]
enum Culoare {
    Negru,
    Alb,
}

#[derive(Clone, Copy, Debug)]
struct Patratel {
    piesa: Piesa,
    culoare: Culoare,
}

struct State {
    game_state: GameState,
    tabla: [[Option<Patratel>; 8]; 8], // Tabla de joc
    piesa_selectata: Piesa,
    egui_backend: EguiBackend,
}

impl ggez::event::EventHandler<ggez::GameError> for State {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        let egui_ctx = self.egui_backend.ctx();
        // FIXME: fixeaza meniul pe ecran
        match self.game_state {
            GameState::MainMenu => {
                egui::Window::new("egui")
                    .title_bar(false)
                    .show(&egui_ctx, |ui| {
                        if ui.button("START").clicked() {
                            self.game_state = GameState::Game;
                        }
                        if ui.button("Editor").clicked() {
                            self.piesa_selectata = Piesa::Pion;
                            self.game_state = GameState::Editor;
                        }
                        if ui.button("Quit").clicked() {
                            ggez::event::quit(ctx);
                        }
                    });
            }
            GameState::Game => {
                egui::Window::new("egui-game")
                    .title_bar(false)
                    .show(&egui_ctx, |ui| {
                        if ui.button("help").clicked() {
                            // TODO: help text pt joc
                        }
                        if ui.button("back").clicked() {
                            self.game_state = GameState::MainMenu;
                        }
                    });
            }
            GameState::Editor => {
                egui::Window::new("egui-editor")
                    .title_bar(false)
                    .show(&egui_ctx, |ui| {
                        egui::ComboBox::from_label("Piesa")
                            .selected_text(format!("{:?}", self.piesa_selectata))
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut self.piesa_selectata, Piesa::Pion, "Pion");
                                ui.selectable_value(&mut self.piesa_selectata, Piesa::Tura, "Tura");
                                ui.selectable_value(&mut self.piesa_selectata, Piesa::Cal, "Cal");
                                ui.selectable_value(
                                    &mut self.piesa_selectata,
                                    Piesa::Nebun,
                                    "Nebun",
                                );
                                ui.selectable_value(
                                    &mut self.piesa_selectata,
                                    Piesa::Regina,
                                    "Regina",
                                );
                                ui.selectable_value(&mut self.piesa_selectata, Piesa::Rege, "Rege");
                            });
                        if ui.button("help").clicked() {
                            // TODO: help text pt editor
                        }
                        if ui.button("back").clicked() {
                            self.game_state = GameState::MainMenu;
                        }
                    });
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::clear(ctx, graphics::Color::BLACK);

        if self.game_state != GameState::MainMenu {
            draw_board(ctx)?;

            // TODO: draw curata codul de mai jos
            if self.game_state == GameState::Editor {
                // la un click, amplaseaza pionul
                if ggez::input::mouse::button_pressed(ctx, MouseButton::Left) {
                    let cursor = ggez::input::mouse::position(ctx);
                    let x = cursor.x as usize / 100;
                    let y = cursor.y as usize / 100;
                    if self.tabla[y as usize][x as usize].is_none() {
                        self.tabla[y as usize][x as usize] = Some(Patratel {
                            piesa: self.piesa_selectata,
                            culoare: Culoare::Alb,
                        });
                        for i in self.tabla.iter() {
                            println!("{:?}", i);
                        }
                    }
                // la click-dreapta, amplaseaza piesa neagra
                } else if ggez::input::mouse::button_pressed(ctx, MouseButton::Right) {
                    let cursor = ggez::input::mouse::position(ctx);
                    let x = cursor.x as usize / 100;
                    let y = cursor.y as usize / 100;
                    if self.tabla[y as usize][x as usize].is_none() {
                        self.tabla[y as usize][x as usize] = Some(Patratel {
                            piesa: self.piesa_selectata,
                            culoare: Culoare::Negru,
                        });
                    }
                    for i in self.tabla.iter() {
                        println!("{:?}", i);
                    }
                // la click pe rotita, sterge pionul
                } else if ggez::input::mouse::button_pressed(ctx, MouseButton::Middle) {
                    let cursor = ggez::input::mouse::position(ctx);
                    let x = cursor.x as usize / 100;
                    let y = cursor.y as usize / 100;
                    if self.tabla[y as usize][x as usize].is_some() {
                        self.tabla[y as usize][x as usize] = None;
                        for i in self.tabla.iter() {
                            println!("{:?}", i);
                        }
                    }
                }
            }
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
        game_state: GameState::MainMenu,
        piesa_selectata: Piesa::Pion,
        tabla: [[None; 8]; 8],
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

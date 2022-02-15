use ggez::{event::MouseButton, graphics, Context};
use ggez_egui::EguiBackend;

mod gui;
mod miscari;
mod t;

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

#[derive(Clone, Copy, Debug, PartialEq)]
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
    turn: Culoare,                     // Al cui e randul
    piesa_selectata: Piesa,
    egui_backend: EguiBackend,
}

impl ggez::event::EventHandler<ggez::GameError> for State {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        let egui_ctx = self.egui_backend.ctx();
        // FIXME: fixeaza meniul pe ecran
        match self.game_state {
            GameState::MainMenu => {
                gui::main_menu(self, &egui_ctx, ctx);
            }
            GameState::Game => {
                gui::game(self, &egui_ctx);
            }
            GameState::Editor => {
                gui::editor(self, &egui_ctx);
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::clear(ctx, graphics::Color::BLACK);

        if self.game_state != GameState::MainMenu {
            draw_board(ctx)?;
            draw_pieces(self, ctx)?;
            if self.game_state == GameState::Game {
                t::generare_tabla(&mut self.tabla);
                if ggez::input::mouse::button_pressed(ctx, MouseButton::Left) {
                    if let Some((x, y)) = get_square_under_mouse(ctx) {
                        if let Some(piesa) = self.tabla[y][x] {
                            if self.turn == piesa.culoare {
                                println!("{}, {}", x, y);
                                // Randul urmatorului jucator
                                self.turn = if self.turn == Culoare::Alb {
                                    Culoare::Negru
                                } else {
                                    Culoare::Alb
                                };
                            } else {
                                println!("nu e randul tau");
                            }
                        }
                    }
                }
            }

            // TODO: draw curata codul de mai jos
            if self.game_state == GameState::Editor {
                // la un click, amplaseaza pionul
                if ggez::input::mouse::button_pressed(ctx, MouseButton::Left) {
                    if let Some((x, y)) = get_square_under_mouse(ctx) {
                        if self.tabla[y][x].is_none() {
                            self.tabla[y][x] = Some(Patratel {
                                piesa: self.piesa_selectata,
                                culoare: Culoare::Alb,
                            });
                            for i in self.tabla.iter() {
                                //println!("{:?}", i);
                            }
                        }
                    }
                // la click-dreapta, amplaseaza piesa neagra
                } else if ggez::input::mouse::button_pressed(ctx, MouseButton::Right) {
                    if let Some((x, y)) = get_square_under_mouse(ctx) {
                        if self.tabla[y][x].is_none() {
                            self.tabla[y][x] = Some(Patratel {
                                piesa: self.piesa_selectata,
                                culoare: Culoare::Negru,
                            });
                        }
                        for i in self.tabla.iter() {
                            println!("{:?}", i);
                        }
                    }
                // la click pe rotita, sterge pionul
                } else if ggez::input::mouse::button_pressed(ctx, MouseButton::Middle) {
                    if let Some((x, y)) = get_square_under_mouse(ctx) {
                        if self.tabla[y][x].is_some() {
                            self.tabla[y][x] = None;
                            for i in self.tabla.iter() {
                                println!("{:?}", i);
                            }
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
        turn: Culoare::Alb,
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
    const L: f32 = 50.0;
    // TODO: pune culorile din gosah?
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
    //ggez::filesystem::print_all(ctx);
    //let img = graphics::Image::new(ctx, "/images/alb/pion.png")?;
    //graphics::draw(ctx, &img, ([0.0, 0.0],))?;
    Ok(())
}
fn draw_pieces(state: &State,ctx: &mut ggez::Context) -> ggez::GameResult {
    for i in 0..8 {
        for j in 0..8 {
            if let Some(patratel) = state.tabla[i][j] {
                let img = graphics::Image::new(
                    ctx,
                    &format!("/images/{:?}/{:?}.png", patratel.culoare, patratel.piesa),
                )?;
                // FIXME: marimi mai ok
                graphics::draw(
                    ctx,
                    &img,
                    graphics::DrawParam::default()
                        .dest([j as f32 * 50.0 + 5.0, i as f32 * 50.0 + 5.0])
                        .scale([0.35, 0.35]),
                )?;
            }
        }
    }
    Ok(())
}
// Returneaza coordonatele patratului unde se afla mouse-ul
fn get_square_under_mouse(ctx: &mut ggez::Context) -> Option<(usize, usize)> {
    let cursor = ggez::input::mouse::position(ctx);
    let x = cursor.x as i32 / 50;
    let y = cursor.y as i32 / 50;
    if t::in_board(x, y) {
        Some((x as usize, y as usize))
    } else {
        None
    }
}

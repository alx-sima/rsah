use ggez::{event::MouseButton, graphics, Context};
use ggez_egui::EguiBackend;

mod draw;
mod gui;
mod miscari;
mod t;

/// Latura unui patratel de pe tabla de sah
const L: f32 = 50.0;

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
    Alb,
    Negru,
}

#[derive(PartialEq)]
enum MatchState {
    Joc,
    Sah,
    Mat,
    Pat,
}

#[derive(Clone, Copy, Debug)]
struct Patratel {
    culoare: Culoare,
    piesa: Piesa,
}

struct State {
    game_state: GameState,
    /// Tabla de joc
    tabla: [[Option<Patratel>; 8]; 8],
    /// Patratele disponibile
    miscari_disponibile: Vec<(usize, usize)>,
    /// Al cui e randul
    turn: Culoare,
    match_state: MatchState,
    /// Pozitia piesei pe care a fost dat click pt a se muta
    /// (marcata cu un patrat verde)
    piesa_sel: Option<(usize, usize)>,
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
            draw::board(ctx)?;
            draw::attack(self, ctx)?;
            if self.game_state == GameState::Game {
                if ggez::input::mouse::button_pressed(ctx, MouseButton::Left) {
                    // Daca clickul este in interiorul tablei
                    if let Some((j, i)) = get_square_under_mouse(ctx) {
                        // Daca exista o piesa selectata
                        if let Some((i_sel, j_sel)) = self.piesa_sel {
                            // Patratul selectat
                            let p_sel = self.tabla[i_sel][j_sel];
                            if self.miscari_disponibile.contains(&(i, j)) {
                                self.tabla[i][j] = p_sel;
                                self.tabla[i_sel][j_sel] = None;
                                miscari::verif_sah(&self.tabla, i as i32, j as i32);

                                // Randul urmatorului jucator
                                self.turn = match self.turn {
                                    Culoare::Alb => Culoare::Negru,
                                    Culoare::Negru => Culoare::Alb,
                                };
                            }
                            self.piesa_sel = None;
                            self.miscari_disponibile = Vec::new();

                        // ...daca nu, o selecteaza (daca e de aceeasi culoare)
                        } else {
                            if let Some(piesa) = self.tabla[i][j] {
                                if self.turn == piesa.culoare {
                                    self.piesa_sel = Some((i, j));
                                    self.miscari_disponibile =
                                        miscari::get_miscari(&self.tabla, i as i32, j as i32);
                                }
                            }
                        }
                    }
                }
            }
            draw::pieces(self, ctx)?;

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
                    }
                // la click pe rotita, sterge pionul
                } else if ggez::input::mouse::button_pressed(ctx, MouseButton::Middle) {
                    if let Some((x, y)) = get_square_under_mouse(ctx) {
                        if self.tabla[y][x].is_some() {
                            self.tabla[y][x] = None;
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
        egui_backend: EguiBackend::default(),
        game_state: GameState::MainMenu,
        miscari_disponibile: Vec::new(),
        match_state: MatchState::Joc,
        piesa_sel: None,
        piesa_selectata: Piesa::Pion,
        tabla: [[None; 8]; 8],
        turn: Culoare::Alb,
    };
    let c = ggez::conf::Conf::new();
    let (ctx, event_loop) = ggez::ContextBuilder::new("rsah", "bamse")
        .default_conf(c)
        .build()
        .unwrap();

    ggez::event::run(ctx, event_loop, state);
}

/// Returneaza coordonatele patratului unde se afla mouse-ul, sau
/// None => mouse-ul nu se afla in tabla de sah
fn get_square_under_mouse(ctx: &mut ggez::Context) -> Option<(usize, usize)> {
    let cursor = ggez::input::mouse::position(ctx);
    let x = (cursor.x / L) as i32;
    let y = (cursor.y / L) as i32;
    if t::in_board(x, y) {
        Some((x as usize, y as usize))
    } else {
        None
    }
}

use std::{io::Read, net::TcpStream};

use ggez::{
    conf,
    event::{self, EventLoop, MouseButton},
    graphics, Context,
};
use ggez_egui::EguiBackend;

use tabla::{draw, Culoare, TipPiesa};

/// meniurile grafice pentru a selecta 
/// jocul, editorul, conectare multiplayer
mod gui;
/// tabla de sah si orice legat de aceasta
/// (afisare, piese, mutari, generare etc.)
mod tabla;

#[derive(PartialEq)]
enum GameState {
    MainMenu,
    Game,
    Editor,
    Multiplayer,
}

struct Address {
    ip: [u8; 4],
    port: u16,
}

/// Variabilele globale ale jocului
struct State {
    /// In ce meniu/mod de joc e
    game_state: GameState,
    /// Tabla de joc
    tabla: tabla::Tabla,
    /// Istoric miscari
    istoric: Vec<String>,
    /// Patratele disponibile
    miscari_disponibile: Vec<(usize, usize)>,
    /// Al cui e randul
    turn: Culoare,
    /// Pozitia piesei pe care a fost dat click pt a se muta
    /// (marcata cu un patrat verde)
    piesa_sel: Option<(usize, usize)>,
    piesa_selectata_editor: TipPiesa,
    egui_backend: EguiBackend,
    stream: Option<TcpStream>,
    address: Address,
    /// Daca e *true*, meciul se joaca pe alt dispozitiv,
    /// piesele negre vor aparea in josul tablei
    guest: bool,
}

impl Default for State {
    fn default() -> Self {
        State {
            egui_backend: EguiBackend::default(),
            game_state: GameState::MainMenu,
            miscari_disponibile: vec![],
            istoric: vec![],
            piesa_sel: None,
            piesa_selectata_editor: TipPiesa::Pion,
            tabla: Default::default(),
            turn: Culoare::Alb,
            stream: None,
            address: Address {
                ip: [127, 0, 0, 1],
                port: 8080,
            },
            guest: false,
        }
    }
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
            GameState::Multiplayer => {
                if (self.turn == Culoare::Negru) != self.guest {
                    let mut buf = [0; 16];
                    match self.stream.as_mut().unwrap().read(&mut buf) {
                        Ok(len) => {
                            let msg = std::str::from_utf8(&buf[..len]).unwrap();
                            if let Some((src_poz, dest_poz)) =
                                tabla::notatie::decode_move(&self.tabla, msg, self.turn)
                            {
                                // FIXME: CRED ca nu se actualizeaza celulele atacate de pioni
                                tabla::game::muta(&mut self.tabla, src_poz, dest_poz);

                                // Randul urmatorului jucator
                                // Schimba turn din alb in negru si din negru in alb
                                self.turn = match self.turn {
                                    Culoare::Alb => Culoare::Negru,
                                    Culoare::Negru => Culoare::Alb,
                                };
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::clear(ctx, graphics::Color::BLACK);

        if self.game_state != GameState::MainMenu {
            draw::board(ctx)?;
            draw::attack(self, ctx)?;
            if self.game_state == GameState::Game
                || self.game_state == GameState::Multiplayer
                    && (self.turn == Culoare::Negru) == self.guest
            {
                if ggez::input::mouse::button_pressed(ctx, MouseButton::Left) {
                    tabla::game::player_turn(
                        ctx,
                        &mut self.tabla,
                        &mut self.piesa_sel,
                        &mut self.miscari_disponibile,
                        &mut self.turn,
                        &mut self.istoric,
                        &mut self.stream,
                        self.guest,
                    );
                }
            } else if self.game_state == GameState::Editor {
                tabla::editor::player_turn(ctx, &mut self.tabla, self.piesa_selectata_editor);
            }
            draw::pieces(self, ctx)?;
        }

        graphics::draw(ctx, &self.egui_backend, ([0.0, 0.0],))?;
        graphics::present(ctx)
    }

    /// Updateaza rezolutia logica a ecranului cand se schimba cea fizica,
    /// altfel imaginile ar fi desenate scalate.
    fn resize_event(&mut self, ctx: &mut ggez::Context, width: f32, height: f32) {
        let screen_rect = graphics::Rect::new(0.0, 0.0, width, height);
        graphics::set_screen_coordinates(ctx, screen_rect).unwrap();
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

/// Configureaza aplicatia inainte de a o rula
fn build_context() -> ggez::GameResult<(Context, EventLoop<()>)> {
    let setup = conf::WindowSetup::default()
        .title("Chess")
        .icon("/images/appicon.png");
    let mode = conf::WindowMode::default()
        .min_dimensions(800.0, 600.0)
        .resizable(true);

    let c = conf::Conf::new();
    ggez::ContextBuilder::new("rsah", "bamse")
        .default_conf(c)
        .window_setup(setup)
        .window_mode(mode)
        .build()
}

fn main() -> ggez::GameResult {
    let state = State::default();
    let (ctx, event_loop) = build_context()?;
    event::run(ctx, event_loop, state);
}

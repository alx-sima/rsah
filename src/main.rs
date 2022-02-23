use ggez::{event::MouseButton, graphics, Context};
use ggez_egui::EguiBackend;
use tabla::{Culoare, Patratel, TipPiesa};

mod draw;
mod gui;
mod miscari;
mod tabla;

/// Latura unui patratel de pe tabla de sah
const L: f32 = 50.0;

#[derive(PartialEq)]
enum GameState {
    MainMenu,
    Game,
    Editor,
}

#[derive(PartialEq)]
enum MatchState {
    Joc,
    Sah,
    Mat,
    // Pat,
}

/// Variabilele globale ale jocului
struct State {
    /// In ce meniu/mod de joc e
    game_state: GameState,
    /// Tabla de joc
    tabla: [[Patratel; 8]; 8],
    /// Istoric miscari
    istoric: Vec<String>,
    /// Patratele disponibile
    miscari_disponibile: Vec<(usize, usize)>,
    /// Al cui e randul
    turn: Culoare,
    // FIXME: valea!!
    match_state: MatchState,
    /// Pozitia piesei pe care a fost dat click pt a se muta
    /// (marcata cu un patrat verde)
    piesa_sel: Option<(usize, usize)>,
    piesa_selectata_editor: TipPiesa,
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
                    tabla::game::player_turn(
                        ctx,
                        &mut self.tabla,
                        &mut self.piesa_sel,
                        &mut self.miscari_disponibile,
                        &mut self.turn,
                        &mut self.match_state,
                        &mut self.istoric,
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
        istoric: Vec::new(),
        piesa_sel: None,
        piesa_selectata_editor: TipPiesa::Pion,
        tabla: Default::default(),
        turn: Culoare::Alb,
    };
    let c = ggez::conf::Conf::new();
    let (ctx, event_loop) = ggez::ContextBuilder::new("rsah", "bamse")
        .default_conf(c)
        .build()
        .unwrap();

    ggez::event::run(ctx, event_loop, state);
}

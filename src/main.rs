use std::{
    fmt,
    net::{TcpListener, TcpStream},
};

use ggez::{
    conf,
    event::{self, EventLoop, MouseButton},
    graphics, Context,
};
use ggez_egui::EguiBackend;

use tabla::{draw, Culoare, Pozitie, Tabla, TipPiesa};

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

/// Variabilele globale ale jocului
struct State {
    // ================================ Globale ================================
    /// Backend pt user interface.
    egui_backend: EguiBackend,
    /// In ce meniu/mod de joc e.
    game_state: GameState,
    /// Ce aranjament al tablei e selectat.
    game_mode: GameMode,
    /// Layouturile custom incarcate din fisiere.
    game_layouts: Vec<String>,
    /// Tabla de joc
    tabla: Tabla,
    /// Pozitia de inceput a clickului
    /// (daca a fost initiat).
    click: Option<Pozitie>,

    // ================================== Joc ==================================
    /// Patratele disponibile.
    miscari_disponibile: Vec<Mutare>,
    /// Al cui e randul.
    turn: Culoare,
    /// Pozitia piesei pe care a fost dat click pt a se muta
    /// (marcata cu un patrat verde)
    piesa_sel: Option<Pozitie>,

    // ================================ Editor =================================
    /// (doar pt editor) Piesa care se va pune la click.
    piesa_sel_editor: TipPiesa,
    ed_save_name: String,

    // ============================== Multiplayer ==============================
    /// Adresa IP a jocului hostat
    address: String,
    /// Listenerul pt. conexiune, cand clientul
    /// hosteaza si asteapta un guest.
    tcp_host: Option<TcpListener>,
    /// Conexiunea la celalalt jucator.
    stream: Option<TcpStream>,
    /// Daca e *true*, meciul se joaca pe alt dispozitiv,
    /// piesele negre vor aparea in josul tablei
    guest: bool,
}

#[derive(Clone, Debug)]
struct Mutare {
    /// Pozitia pe care va ajunge piesa mutata.
    dest: Pozitie,
    /// In ce consta mutarea.
    tip: TipMutare,
}

#[derive(Clone, Debug, PartialEq)]
enum TipMutare {
    /// Mutare normala.
    Normal,
    /// O piesa este capturata.
    Captura,
    /// Miscarea presupune un en passant.
    /// Se retine pozitia **efectiva** a
    /// pionului care va fi capturat,
    /// nu destinatia, aceea va fi in `Mutare.dest`.
    EnPassant(Pozitie),
    /// Miscarea presupune o rocada.
    /// Se retine pozitia turei care participa.
    Rocada(Pozitie),
}

#[derive(PartialEq)]
enum GameMode {
    /// Aranjamentul standard.
    Clasic,
    /// Piese generate aleatoriu
    /// cu generare::new_random().
    Aleatoriu,
    Custom(String),
}

impl fmt::Display for GameMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GameMode::Clasic => write!(f, "Clasic"),
            GameMode::Aleatoriu => write!(f, "Aleatoriu"),
            GameMode::Custom(s) => write!(f, "Custom: {}", s),
        }
    }
}

impl Default for State {
    /// Valorile initiale ale variabilelor globale.
    fn default() -> Self {
        State {
            ed_save_name: String::from("custom_layout"),
            address: String::from("127.0.0.1:8080"),
            egui_backend: EguiBackend::default(),
            piesa_sel_editor: TipPiesa::Pion,
            game_state: GameState::MainMenu,
            game_mode: GameMode::Clasic,
            miscari_disponibile: vec![],
            tabla: Default::default(),
            game_layouts: vec![],
            turn: Culoare::Alb,
            piesa_sel: None,
            tcp_host: None,
            stream: None,
            guest: false,
            click: None,
        }
    }
}

impl ggez::event::EventHandler<ggez::GameError> for State {
    /// Logica jcocului.
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        let egui_ctx = self.egui_backend.ctx();

        match self.game_state {
            GameState::Game | GameState::Multiplayer => {
                gui::game(self, &egui_ctx, ctx);

                tabla::game::turn_handler(ctx, self);
            }
            GameState::Editor => {
                gui::editor(self, &egui_ctx, ctx);

                tabla::editor::editor_handler(ctx, &mut self.tabla.mat, self.piesa_sel_editor);
            }
            GameState::MainMenu => {
                gui::main_menu(self, &egui_ctx, ctx);
            }
        }
        Ok(())
    }

    /// Grafica jocului.
    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::clear(ctx, graphics::Color::BLACK);

        if self.game_state != GameState::MainMenu {
            draw::board(ctx)?;
            if self.game_state != GameState::Editor {
                draw::attack(self, ctx)?;
            }
            draw::pieces(self, ctx)?;
        }

        graphics::draw(ctx, &self.egui_backend, ([0.0, 0.0],))?;
        graphics::present(ctx)
    }

    // =========================================================================
    // ======================= Layere de compatibilitate =======================

    /// Updateaza rezolutia logica a ecranului cand se schimba cea fizica,
    /// altfel imaginile ar fi desenate scalate.
    fn resize_event(&mut self, ctx: &mut ggez::Context, width: f32, height: f32) {
        let screen_rect = graphics::Rect::new(0.0, 0.0, width, height);
        graphics::set_screen_coordinates(ctx, screen_rect).unwrap();
    }

    // =============================== Tastatura ===============================

    /// Pt. ca egui sa captureze tastatura.
    fn text_input_event(&mut self, _ctx: &mut Context, ch: char) {
        self.egui_backend.input.text_input_event(ch);
    }

    /// Pt. tastele care nu corespund cu caractere.
    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: event::KeyCode,
        keymods: event::KeyMods,
        _repeat: bool,
    ) {
        self.egui_backend.input.key_down_event(keycode, keymods);
    }

    // ================================= Mouse =================================

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

/// Se configureaza aplicatia inainte de a o rula
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

/// Entry-pointul aplicatiei.
fn main() -> ggez::GameResult {
    let state = State::default();
    let (ctx, event_loop) = build_context()?;
    event::run(ctx, event_loop, state);
}

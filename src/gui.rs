use std::{
    io::{ErrorKind, Read, Write},
    net::{TcpListener, TcpStream},
};

use ggez::filesystem;
use ggez_egui::EguiContext;

use crate::{
    tabla::{editor, game, generare, input, sah, Culoare, MatTabla, MatchState, Tabla, TipPiesa},
    GameMode, GameState, State,
};

/// Randeaza home-screenul.
pub(crate) fn main_menu(state: &mut State, egui_ctx: &EguiContext, ctx: &mut ggez::Context) {
    egui::Window::new("egui")
        .title_bar(false)
        .fixed_pos([0.0, 0.0])
        .show(egui_ctx, |ui| {
            // Umple tot ecranul cu GUI-ul
            let (x, y) = ggez::graphics::drawable_size(ctx);
            ui.set_width(x - 10.0);
            ui.set_height(y - 10.0);

            ui.vertical_centered_justified(|ui| {
                ui.group(|ui| {
                    let combox = egui::ComboBox::from_label("")
                        .selected_text(format!("{}", state.game_mode))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut state.game_mode, GameMode::Clasic, "Clasic");
                            ui.selectable_value(
                                &mut state.game_mode,
                                GameMode::Aleatoriu,
                                "Aleatoriu",
                            );

                            for l in &state.game_layouts {
                                if ui
                                    .selectable_value(
                                        &mut state.game_mode,
                                        GameMode::Custom(l.clone()),
                                        l,
                                    )
                                    .on_hover_text("click-dreapta pentru a sterge!")
                                    .secondary_clicked()
                                {
                                    if let Err(e) = filesystem::delete(ctx, l) {
                                        eprintln!("{}", e);
                                    }

                                    // Daca s-a sters layoutul selectat, se schimba la cel clasic.
                                    if let GameMode::Custom(s) = &state.game_mode {
                                        if s == l {
                                            state.game_mode = GameMode::Clasic;
                                        }
                                    }
                                }
                            }
                        });

                    if combox.response.clicked() {
                        state.game_layouts = editor::list_files(ctx);
                    }

                    if ui.button("Local").clicked() {
                        state.game_state = GameState::Game;
                        load_layout(&state.game_mode, ctx).init_game(state);
                    }

                    ui.horizontal(|ui| {
                        ui.add(egui::TextEdit::singleline(&mut state.address));
                        if let Some(listener) = &state.tcp_host {
                            // Guestul a acceptat conexiunea
                            match listener.accept() {
                                Ok((mut s, _)) => {
                                    state.game_state = GameState::Multiplayer;
                                    s.set_nonblocking(true).unwrap();
                                    load_layout(&state.game_mode, ctx).init_game(state);

                                    // Se trimite layout-ul de joc guestului.
                                    s.write_all(
                                        serde_json::to_string(&state.tabla.mat).unwrap().as_bytes(),
                                    )
                                    .unwrap();

                                    // Nu se mai asteapta alte conexiuni.
                                    state.tcp_host = None;
                                    state.stream = Some(s);
                                    return;
                                }

                                // Erorile WouldBlock inseamna ca conexiunea nu e
                                // inca stabilita, deci pot fi ignorate.
                                Err(e) if e.kind() != ErrorKind::WouldBlock => {
                                    state.tcp_host = None;
                                    eprintln!("{}", e);
                                }
                                _ => {}
                            }

                            if ui.button("Cancel").clicked() {
                                state.tcp_host = None;
                            }
                        } else {
                            if ui.button("Host").clicked() {
                                let s = TcpListener::bind(state.address.as_str()).unwrap();
                                s.set_nonblocking(true).unwrap();
                                state.tcp_host = Some(s);
                            }
                            if ui.button("Join").clicked() {
                                match TcpStream::connect(state.address.clone().as_str()) {
                                    Ok(mut s) => {
                                        let mut buf = [0u8; 2048];
                                        match s.read(&mut buf) {
                                            Ok(len) => {
                                                state.game_state = GameState::Multiplayer;
                                                s.set_nonblocking(true).unwrap();
                                                state.stream = Some(s);
                                                state.guest = true;

                                                Tabla::from_layout(
                                                    serde_json::from_slice(&buf[..len]).unwrap(),
                                                )
                                                .init_game(state);
                                            }
                                            Err(e) => {
                                                eprintln!("{}", e);
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        eprintln!("{}", e);
                                    }
                                };
                            }
                        }
                    });
                });
            });
            ui.vertical_centered_justified(|ui| {
                if ui.button("Editor").clicked() {
                    state.piesa_sel_editor = TipPiesa::Pion;
                    state.game_state = GameState::Editor;
                    state.miscari_disponibile = vec![];
                    state.tabla = Tabla::default();
                }

                if ui.button("Quit").clicked() {
                    ggez::event::quit(ctx);
                }
            });
        });
}

/// Randeaza meniul din timpul si de la sfarsitul jocului.
pub(crate) fn game(state: &mut State, gui_ctx: &EguiContext, ctx: &ggez::Context) {
    let match_state = &state.tabla.match_state;

    match *match_state {
        MatchState::Playing => {
            create_side_panel(ctx, "egui-game").show(gui_ctx, |ui| {
                if ui.button("back").clicked() {
                    state.game_state = GameState::MainMenu;
                    state.stream = None;
                }
                // afiseaza ultimele 10 miscari
                ui.group(|ui| {
                    ui.label("istoric:");
                    for i in state.tabla.istoric.iter().rev().take(10) {
                        ui.label(i);
                    }
                });
            });
        }
        MatchState::Promote(poz) => {
            create_side_panel(ctx, "egui-promote").show(gui_ctx, |ui| {
                ui.vertical(|ui| {
                    for p in [
                        TipPiesa::Tura,
                        TipPiesa::Cal,
                        TipPiesa::Nebun,
                        TipPiesa::Regina,
                    ]
                    .iter()
                    {
                        if ui.button(&format!("{:?}", p)).clicked() {
                            let mut culoare = state.turn;
                            culoare.invert();

                            game::set_piesa(&mut state.tabla, poz, Some(*p), culoare);
                            let mutare = format!("{}={}", state.mutare_buf, p);

                            // Se scrie mutarea
                            state.tabla.istoric.push(mutare.clone());
                            if let Some(s) = &mut state.stream {
                                s.write_all(mutare.as_bytes()).unwrap();
                            }

                            // Deselecteaza piesa.
                            state.piesa_sel = None;
                            state.miscari_disponibile = vec![];
                            // Se revine la joc
                            state.tabla.match_state = MatchState::Playing;
                        }
                    }
                });
            });
        }
        _ => {
            egui::Window::new("egui-end-game")
                .title_bar(false)
                .show(gui_ctx, |ui| {
                    match *match_state {
                        MatchState::Pat => {
                            ui.label("Egalitate!");
                        }
                        MatchState::Mat(loser) => {
                            ui.label(&format!("{:?} este in mat!", loser));
                        }
                        _ => unreachable!(),
                    }

                    if ui.button("Main Menu").clicked() {
                        state.game_state = GameState::MainMenu;
                    }
                });
        }
    }
}

/// Randeaza meniul din editor.
pub(crate) fn editor(state: &mut State, egui_ctx: &EguiContext, ctx: &mut ggez::Context) {
    create_side_panel(ctx, "egui-editor").show(egui_ctx, |ui| {
        ui.group(|ui| {
            for t in [
                TipPiesa::Pion,
                TipPiesa::Tura,
                TipPiesa::Cal,
                TipPiesa::Nebun,
                TipPiesa::Regina,
                TipPiesa::Rege,
            ] {
                ui.radio_value(&mut state.piesa_sel_editor, t, &format!("{:?}", t));
            }
        });

        ui.vertical_centered_justified(|ui| {
            ui.add(egui::TextEdit::singleline(&mut state.ed_save_name));

            let save_button = ui.button("save");
            if save_button.clicked() {
                if state.ed_save_name.is_empty() {
                    println!("numele e invalid");
                    return;
                }

                generare::init_piese(&mut state.tabla);

                if state.tabla.valideaza_layout() {
                    let rez = serde_json::to_string_pretty(&state.tabla.mat).unwrap();

                    let mut f = filesystem::create(ctx, &format!("/{}.json", state.ed_save_name)).unwrap();
                    f.write_all(rez.as_bytes()).unwrap();

                    state.game_state = GameState::MainMenu;
                    state.game_mode = GameMode::Custom(state.ed_save_name.clone());
                } else {
                    for i in 0..8 {
                        for j in 0..8 {
                            state.tabla.mat[i][j].afecteaza = vec![];
                            state.tabla.mat[i][j].atacat = vec![];
                        }
                    }
                }
            }

            if ui.button("back").clicked() {
                state.game_state = GameState::MainMenu;
            }
        });
    });
}

/// (pentru editor): verifica daca exista cate un rege pe tabla.
fn exista_rege(tabla: &MatTabla, culoare: Culoare) -> bool {
    for line in tabla.iter().take(8) {
        for item in line.iter().take(8) {
            if let Some(piesa) = &item.piesa {
                if piesa.tip == TipPiesa::Rege && piesa.culoare == culoare {
                    return true;
                }
            }
        }
    }
    false
}

/// Construieste un SidePanel pentru afisarea
/// informatiilor in editor sau in joc.
fn create_side_panel(ctx: &ggez::Context, id: &str) -> egui::SidePanel {
    let (_, x_pad, _) = input::get_dimensiuni_tabla(ctx);
    let x_pad = x_pad - 20.0;
    egui::SidePanel::left(id)
        .min_width(x_pad)
        .max_width(x_pad)
        .resizable(false)
}

/// Construieste Tabla din `game_mode`-ul selectat.
/// Daca `game_mode` e `GameMode::Custom`, se va incarca
/// layoutul din fisierul selectat.
fn load_layout(game_mode: &GameMode, ctx: &ggez::Context) -> Tabla {
    match game_mode {
        GameMode::Clasic => Tabla::new_clasica(),
        GameMode::Aleatoriu => Tabla::new_random(),
        GameMode::Custom(s) => {
            let layout = editor::load_file(ctx, s).unwrap();
            Tabla::from_layout(layout)
        }
    }
}

impl Tabla {
    /// Se initializeaza `state`ul pentru un
    /// nou joc cu templateul specificat.
    fn init_game(self, state: &mut State) {
        state.tabla = self;
        state.miscari_disponibile = vec![];
        state.turn = Culoare::Alb;
        state.piesa_sel = None;
    }

    /// Verifica daca layoutul tablei
    /// poate fi salvat, adica:
    /// - niciun rege nu e in sah
    /// - exista miscari valabile
    /// - exista 2 regi
    fn valideaza_layout(&self) -> bool {
        for culoare in [Culoare::Alb, Culoare::Negru] {
            if !exista_rege(&self.mat, culoare)
                || sah::verif_continua_jocul(self, culoare).is_some()
                || sah::in_sah(self, culoare)
            {
                return false;
            }
        }
        true
    }
}

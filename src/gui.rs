use std::{
    io::{ErrorKind, Read, Write},
    net::{TcpListener, TcpStream},
};

use ggez::filesystem;
use ggez_egui::EguiContext;

use crate::{
    tabla::{
        editor,
        generare::init_piese,
        input::get_dimensiuni_tabla,
        sah::{verif_continua_jocul, verif_sah},
        Culoare, MatTabla, MatchState, Tabla, TipPiesa,
    },
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
                    egui::ComboBox::from_label("Aranjament piese")
                        .selected_text(format!("{}", state.game_mode))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut state.game_mode, GameMode::Clasic, "Clasic");
                            ui.selectable_value(
                                &mut state.game_mode,
                                GameMode::Aleatoriu,
                                "Aleatoriu",
                            );

                            for l in &state.game_layouts {
                                ui.selectable_value(
                                    &mut state.game_mode,
                                    GameMode::Custom(l.clone()),
                                    l,
                                );
                            }
                        });
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

/// Randeaza meniul din timpul jocului si cel de la sfarsitul acestuia.
pub(crate) fn game(state: &mut State, gui_ctx: &EguiContext) {
    let match_state = &state.tabla.match_state;

    if *match_state == MatchState::Playing {
        egui::Window::new("egui-game")
            .title_bar(false)
            .fixed_pos([0.0, 0.0])
            .show(gui_ctx, |ui| {
                if ui.button("back").clicked() {
                    state.game_state = GameState::MainMenu;
                    state.stream = None;
                }
            });
    } else if let MatchState::Promote(poz) = match_state {
        egui::Window::new("egui-promote")
            .title_bar(false)
            .show(gui_ctx, |ui| {
                ui.horizontal(|ui| {
                    for p in [
                        TipPiesa::Tura,
                        TipPiesa::Cal,
                        TipPiesa::Nebun,
                        TipPiesa::Regina,
                    ]
                    .iter()
                    {
                        // FIXME: nume full
                        if ui.button(p.to_string()).clicked() {
                            // TODO:
                            //state.tabla.promote(poz, *p);
                        }
                    }
                });
            });
    } else {
        egui::Window::new("egui-end-game")
            .title_bar(false)
            .show(gui_ctx, |ui| {
                if *match_state == MatchState::Pat {
                    ui.label("Egalitate!");
                } else if let MatchState::Mat(loser) = match_state {
                    ui.label(format!("{:?} este in mat!", loser).as_str());
                }

                if ui.button("Main Menu").clicked() {
                    state.game_state = GameState::MainMenu;
                }
            });
    }
}

/// Randeaza meniul din editor.
pub(crate) fn editor(state: &mut State, egui_ctx: &EguiContext, ctx: &mut ggez::Context) {
    egui::Window::new("egui-editor")
        .title_bar(false)
        .fixed_pos([0.0, 0.0])
        .show(egui_ctx, |ui| {
            // FIXME: sa functioneze pe toate rezolutiile
            // Umple toata marginea din stanga
            let (_, x, _) = get_dimensiuni_tabla(ctx);
            let (_, y) = ggez::graphics::drawable_size(ctx);
            ui.set_width(x - 20.0);
            ui.set_height(y - 20.0);

            ui.radio_value(&mut state.piesa_sel_editor, TipPiesa::Pion, "Pion");
            ui.radio_value(&mut state.piesa_sel_editor, TipPiesa::Tura, "Tura");
            ui.radio_value(&mut state.piesa_sel_editor, TipPiesa::Cal, "Cal");
            ui.radio_value(&mut state.piesa_sel_editor, TipPiesa::Nebun, "Nebun");
            ui.radio_value(&mut state.piesa_sel_editor, TipPiesa::Regina, "Regina");
            ui.radio_value(&mut state.piesa_sel_editor, TipPiesa::Rege, "Rege");

            ui.vertical_centered_justified(|ui| {
                ui.add(egui::TextEdit::singleline(&mut state.save_name_editor));

                if ui.button("save").clicked() {
                    init_piese(&mut state.tabla);

                    if state.tabla.valideaza_layout() {
                        let rez = serde_json::to_string_pretty(&state.tabla.mat).unwrap();
                        let mut f =
                            filesystem::create(ctx, format!("/{}.json", state.save_name_editor))
                                .unwrap();
                        f.write_all(rez.as_bytes()).unwrap();
                        state.game_state = GameState::MainMenu;
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
                || verif_continua_jocul(self, culoare).is_some()
                || verif_sah(self, culoare)
            {
                return false;
            }
        }
        true
    }
}

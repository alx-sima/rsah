use std::{
    io::{ErrorKind, Write},
    net::{TcpListener, TcpStream},
};

use ggez::filesystem;
use ggez_egui::EguiContext;

use crate::{
    tabla::{
        game::verif_continua_jocul, generare::init_piese, input::get_dimensiuni_tabla,
        miscari::verif_sah, Culoare, MatTabla, MatchState, Tabla, TipPiesa,
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
                        init_game(state, ctx);
                    }

                    ui.horizontal(|ui| {
                        ui.add(egui::TextEdit::singleline(&mut state.address));
                        if let Some(listener) = &state.tcp_host {
                            // Guestul a acceptat conexiunea
                            match listener.accept() {
                                Ok((s, _)) => {
                                    state.game_state = GameState::Multiplayer;
                                    s.set_nonblocking(true).unwrap();
                                    state.stream = Some(s);
                                    // Nu mai asteapta alte conexiuni.
                                    state.tcp_host = None;
                                    init_game(state, ctx);
                                    return;
                                }

                                // Erorile WouldBlock inseamna ca conexiunea nu e
                                // inca stabilita, deci pot fi ignorate.
                                Err(e) if e.kind() != ErrorKind::WouldBlock => {
                                    state.tcp_host = None;
                                    println!("{}", e);
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
                                    Ok(s) => {
                                        state.game_state = GameState::Multiplayer;
                                        s.set_nonblocking(true).unwrap();
                                        state.stream = Some(s);
                                        state.guest = true;
                                        // TODO: sa primeasca prin tcp layoutul tablei
                                        init_game(state, ctx);
                                    }
                                    Err(e) => {
                                        println!("{}", e);
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
    } else {
        egui::Window::new("egui-end-game")
            .title_bar(false)
            .show(gui_ctx, |ui| {
                if *match_state == MatchState::Pat {
                    ui.label("Egalitate!");
                } else if *match_state == MatchState::AlbEMat {
                    ui.label("Negru castiga!");
                } else if *match_state == MatchState::NegruEMat {
                    ui.label("Alb castiga!");
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

                    if valideaza_layout(&state.tabla) {
                        let rez = serde_json::to_string_pretty(&state.tabla.mat).unwrap();
                        let mut f =
                            filesystem::create(ctx, format!("/{}.json", state.save_name_editor))
                                .unwrap();
                        f.write_all(rez.as_bytes()).unwrap();
                        state.game_state = GameState::MainMenu;
                    } else {
                        // FIXME: sa mearga pe aceeasi idee cu muta()?
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

fn init_game(state: &mut State, ctx: &ggez::Context) {
    state.tabla = match &state.game_mode {
        GameMode::Clasic => Tabla::new_clasica(),
        GameMode::Aleatoriu => Tabla::new_random(),
        GameMode::Custom(s) => {
            let layout = crate::tabla::editor::load_file(ctx, s).unwrap();
            Tabla::from_layout(layout)
        }
    };

    state.miscari_disponibile = vec![];
    state.turn = Culoare::Alb;
    state.piesa_sel = None;
}

fn valideaza_layout(tabla: &Tabla) -> bool {
    for culoare in [Culoare::Alb, Culoare::Negru] {
        if !exista_rege(&tabla.mat, culoare)
            || verif_continua_jocul(tabla, culoare) != MatchState::Playing
            || verif_sah(tabla, culoare)
        {
            return false;
        }
    }
    true
}

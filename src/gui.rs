use std::{
    io::{ErrorKind, Write},
    net::{TcpListener, TcpStream},
};

use ggez_egui::EguiContext;

use crate::{
    tabla::{
        generare, input::get_dimensiuni_tabla, miscari::verif_sah, Culoare, MatTabla, MatchState,
        TipPiesa,
    },
    GameState, State,
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
                ui.collapsing("Local", |ui| {
                    if ui.button("Clasic").clicked() {
                        state.turn = Culoare::Alb;
                        state.game_state = GameState::Game;
                        state.tabla = generare::tabla_clasica();
                    }

                    // TODO:
                    if ui.button("Aleator").clicked() {
                        state.game_state = GameState::Game;
                        state.tabla = generare::tabla_random();
                    }
                });

                // FIXME: centreaza collapsingurile
                ui.horizontal(|ui| {
                    ui.add(egui::TextEdit::singleline(&mut state.address));
                    if let Some(listener) = &state.tcp_host {
                        // Guestul a acceptat conexiunea
                        match listener.accept() {
                            Ok((s, _)) => {
                                s.set_nonblocking(true).unwrap();
                                state.stream = Some(s);
                                // Nu mai asteapta alte conexiuni.
                                state.tcp_host = None;
                                state.game_state = GameState::Multiplayer;
                                state.tabla = generare::tabla_clasica();
                                init_game(state);
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
                            let s = TcpListener::bind(state.address.clone().as_str()).unwrap();
                            s.set_nonblocking(true).unwrap();
                            state.tcp_host = Some(s);
                        }
                        if ui.button("Join").clicked() {
                            match TcpStream::connect(state.address.clone().as_str()) {
                                Ok(s) => {
                                    s.set_nonblocking(true).unwrap();
                                    state.tabla = generare::tabla_clasica();
                                    state.game_state = GameState::Multiplayer;
                                    state.stream = Some(s);
                                    state.guest = true;
                                    init_game(state);
                                }
                                Err(e) => {
                                    println!("{}", e);
                                }
                            };
                        }
                    }
                });
            });

            if ui.button("Editor").clicked() {
                state.piesa_selectata_editor = TipPiesa::Pion;
                state.piesa_sel = None;
                init_game(state);
                state.tabla = Default::default();
                state.game_state = GameState::Editor;
            }

            if ui.button("Quit").clicked() {
                ggez::event::quit(ctx);
            }
        });
}

/// Randeaza meniul din timpul jocului si cel de la sfarsitul acestuia.
pub(crate) fn game(game_state: &mut State, egui_ctx: &EguiContext) {
    let match_state = &game_state.tabla.match_state;
    if *match_state == MatchState::Playing {
        egui::Window::new("egui-game")
            .title_bar(false)
            .fixed_pos([0.0, 0.0])
            .show(egui_ctx, |ui| {
                if ui.button("help").clicked() {
                    // TODO: help text pt joc
                }
                if ui.button("back").clicked() {
                    game_state.game_state = GameState::MainMenu;
                }
            });
    } else {
        egui::Window::new("egui-end-game")
            .title_bar(false)
            .show(egui_ctx, |ui| {
                if *match_state == MatchState::Pat {
                    ui.label("Egalitate!");
                } else if *match_state == MatchState::AlbEMat {
                    ui.label("Negru castiga!");
                } else if *match_state == MatchState::NegruEMat {
                    ui.label("Alb castiga!");
                }

                if ui.button("Main Menu").clicked() {
                    game_state.game_state = GameState::MainMenu;
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

            ui.radio_value(&mut state.piesa_selectata_editor, TipPiesa::Pion, "Pion");
            ui.radio_value(&mut state.piesa_selectata_editor, TipPiesa::Tura, "Tura");
            ui.radio_value(&mut state.piesa_selectata_editor, TipPiesa::Cal, "Cal");
            ui.radio_value(&mut state.piesa_selectata_editor, TipPiesa::Nebun, "Nebun");
            ui.radio_value(
                &mut state.piesa_selectata_editor,
                TipPiesa::Regina,
                "Regina",
            );
            ui.radio_value(&mut state.piesa_selectata_editor, TipPiesa::Rege, "Rege");

            ui.vertical_centered_justified(|ui| {
                if ui.button("save").clicked() {
                    // Ambii regi trebuie sa existe
                    if exista_rege(&state.tabla.mat, Culoare::Alb)
                        && exista_rege(&state.tabla.mat, Culoare::Negru)
                    {
                        // Niciun rege nu trebuie sa fie in sah
                        // facut cu github copilot, poate trebuie refacut
                        if !verif_sah(&state.tabla, Culoare::Alb)
                            && !verif_sah(&state.tabla, Culoare::Negru)
                        {
                            let mut f = std::fs::File::create("tabla.txt").unwrap();
                            let mut s = String::new();
                            for i in 0..8 {
                                for j in 0..8 {
                                    s.push_str(&format!("{:?} ", state.tabla.at((i, j))));
                                }
                                s.push('\n');
                            }
                            f.write_all(s.as_bytes()).unwrap();
                        }
                        state.game_state = GameState::MainMenu;
                    }
                }

                if ui.button("help").clicked() {
                    // TODO: help text pt editor
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

fn init_game(game_state: &mut State) {
    game_state.miscari_disponibile = vec![];
    game_state.turn = Culoare::Alb;
}

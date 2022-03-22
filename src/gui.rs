use std::{
    io::Write,
    net::{TcpListener, TcpStream},
};

use ggez_egui::EguiContext;

use crate::{
    tabla::{generare, input::get_dimensiuni_tabla, miscari::verif_sah, Culoare, Tabla, TipPiesa},
    GameState, State,
};

pub(crate) fn main_menu(game_state: &mut State, egui_ctx: &EguiContext, ctx: &mut ggez::Context) {
    egui::Window::new("egui")
        .title_bar(false)
        .fixed_pos([0.0, 0.0])
        .show(egui_ctx, |ui| {
            // Umple tot ecranul cu GUI-ul
            let (x, y) = ggez::graphics::drawable_size(ctx);
            ui.set_width(x - 10.0);
            ui.set_height(y - 10.0);

            ui.vertical_centered_justified(|ui| {
                if ui.button("Start").clicked() {
                    game_state.turn = Culoare::Alb;
                    game_state.game_state = GameState::Game;
                    game_state.tabla = generare::tabla_clasica();
                }

                ui.label("Multiplayer");

                ui.add(egui::TextEdit::singleline(&mut game_state.address));
                if ui.button("Join").clicked() {
                    match TcpStream::connect(game_state.address.as_str()) {
                        Ok(s) => {
                            s.set_nonblocking(true).unwrap();
                            game_state.guest = true;
                            game_state.turn = Culoare::Alb;
                            game_state.tabla = generare::tabla_clasica();
                            game_state.stream = Some(s);
                            game_state.game_state = GameState::Multiplayer;
                        }
                        Err(e) => {
                            println!("{}", e);
                            return;
                        }
                    };
                }
                if ui.button("Host").clicked() {
                    let listener = TcpListener::bind(game_state.address.as_str()).unwrap();
                    match listener.accept() {
                        Ok((s, _addr)) => {
                            s.set_nonblocking(true).unwrap();
                            game_state.turn = Culoare::Alb;
                            game_state.tabla = generare::tabla_clasica();
                            game_state.stream = Some(s);
                            game_state.game_state = GameState::Multiplayer;
                        }
                        Err(e) => {
                            println!("couldn't get client: {}", e);
                            return;
                        }
                    };
                }

                if ui.button("Editor").clicked() {
                    game_state.piesa_selectata_editor = TipPiesa::Pion;
                    game_state.miscari_disponibile = vec![];
                    game_state.piesa_sel = None;
                    game_state.tabla = Default::default();
                    game_state.game_state = GameState::Editor;
                }
                if ui.button("Quit").clicked() {
                    ggez::event::quit(ctx);
                }
            });
        });
}

pub(crate) fn game(game_state: &mut State, egui_ctx: &EguiContext) {
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
}

pub(crate) fn editor(game_state: &mut State, egui_ctx: &EguiContext, ctx: &mut ggez::Context) {
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

            ui.radio_value(
                &mut game_state.piesa_selectata_editor,
                TipPiesa::Pion,
                "Pion",
            );
            ui.radio_value(
                &mut game_state.piesa_selectata_editor,
                TipPiesa::Tura,
                "Tura",
            );
            ui.radio_value(&mut game_state.piesa_selectata_editor, TipPiesa::Cal, "Cal");
            ui.radio_value(
                &mut game_state.piesa_selectata_editor,
                TipPiesa::Nebun,
                "Nebun",
            );
            ui.radio_value(
                &mut game_state.piesa_selectata_editor,
                TipPiesa::Regina,
                "Regina",
            );
            ui.radio_value(
                &mut game_state.piesa_selectata_editor,
                TipPiesa::Rege,
                "Rege",
            );

            ui.vertical_centered_justified(|ui| {
                if ui.button("save").clicked() {
                    // Ambii regi trebuie sa existe
                    if exista_rege(&mut game_state.tabla, Culoare::Alb)
                        && exista_rege(&mut game_state.tabla, Culoare::Negru)
                    {
                        // Niciun rege nu trebuie sa fie in sah
                        // facut cu github copilot, poate trebuie refacut
                        if !verif_sah(&mut game_state.tabla, Culoare::Alb)
                            && !verif_sah(&mut game_state.tabla, Culoare::Negru)
                        {
                            let mut f = std::fs::File::create("tabla.txt").unwrap();
                            let mut s = String::new();
                            for i in 0..8 {
                                for j in 0..8 {
                                    s.push_str(&format!("{:?} ", game_state.tabla[i][j]));
                                }
                                s.push_str("\n");
                            }
                            f.write_all(s.as_bytes()).unwrap();
                        }
                        game_state.game_state = GameState::MainMenu;
                    }
                }

                if ui.button("help").clicked() {
                    // TODO: help text pt editor
                }
                if ui.button("back").clicked() {
                    game_state.game_state = GameState::MainMenu;
                }
            });
        });
}

fn exista_rege(tabla: &Tabla, culoare: Culoare) -> bool {
    for i in 0..8 {
        for j in 0..8 {
            if let Some(piesa) = &tabla[i][j].piesa {
                if piesa.tip == TipPiesa::Rege && piesa.culoare == culoare {
                    return true;
                }
            }
        }
    }
    false
}

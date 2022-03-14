use std::net::{TcpListener, TcpStream};

use ggez_egui::EguiContext;

use crate::{
    tabla::{generare, Culoare, TipPiesa},
    GameState, State,
};

pub(crate) fn main_menu(game_state: &mut State, egui_ctx: &EguiContext, ctx: &mut ggez::Context) {
    egui::Window::new("egui")
        .title_bar(false)
        .show(egui_ctx, |ui| {
            if ui.button("Start").clicked() {
                game_state.turn = Culoare::Alb;
                game_state.game_state = GameState::Game;
                //game_state.tabla = generare::tabla_clasica();
                game_state.tabla = generare::tabla_from(["", "", "", "", "", "", "", "R   K  R"]);
            }

            ui.vertical(|ui| {
                ui.label("Multiplayer");

                ui.add(egui::Slider::new(&mut game_state.address.ip[0], 0..=255));
                ui.add(egui::Slider::new(&mut game_state.address.ip[1], 0..=255));
                ui.add(egui::Slider::new(&mut game_state.address.ip[2], 0..=255));
                ui.add(egui::Slider::new(&mut game_state.address.ip[3], 0..=255));
                ui.add(egui::Slider::new(&mut game_state.address.port, 0..=65535));
                if ui.button("Join").clicked() {
                    match TcpStream::connect(
                        format!(
                            "{}.{}.{}.{}:{}",
                            game_state.address.ip[0],
                            game_state.address.ip[1],
                            game_state.address.ip[2],
                            game_state.address.ip[3],
                            game_state.address.port
                        )
                        .as_str(),
                    ) {
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
                    let listener = TcpListener::bind(
                        format!(
                            "{}.{}.{}.{}:{}",
                            game_state.address.ip[0],
                            game_state.address.ip[1],
                            game_state.address.ip[2],
                            game_state.address.ip[3],
                            game_state.address.port
                        )
                        .as_str(),
                    )
                    .unwrap();
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
            });

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
}

pub(crate) fn game(game_state: &mut State, egui_ctx: &EguiContext) {
    egui::Window::new("egui-game")
        .title_bar(false)
        .show(egui_ctx, |ui| {
            if ui.button("help").clicked() {
                // TODO: help text pt joc
            }
            if ui.button("back").clicked() {
                game_state.game_state = GameState::MainMenu;
            }
        });
}

pub(crate) fn editor(game_state: &mut State, egui_ctx: &EguiContext) {
    egui::Window::new("egui-editor")
        .title_bar(false)
        .show(egui_ctx, |ui| {
            egui::ComboBox::from_label("Piesa")
                .selected_text(format!("{:?}", game_state.piesa_selectata_editor))
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut game_state.piesa_selectata_editor,
                        TipPiesa::Pion,
                        "Pion",
                    );
                    ui.selectable_value(
                        &mut game_state.piesa_selectata_editor,
                        TipPiesa::Tura,
                        "Tura",
                    );
                    ui.selectable_value(
                        &mut game_state.piesa_selectata_editor,
                        TipPiesa::Cal,
                        "Cal",
                    );
                    ui.selectable_value(
                        &mut game_state.piesa_selectata_editor,
                        TipPiesa::Nebun,
                        "Nebun",
                    );
                    ui.selectable_value(
                        &mut game_state.piesa_selectata_editor,
                        TipPiesa::Regina,
                        "Regina",
                    );
                    ui.selectable_value(
                        &mut game_state.piesa_selectata_editor,
                        TipPiesa::Rege,
                        "Rege",
                    );
                });
            if ui.button("help").clicked() {
                // TODO: help text pt editor
            }
            if ui.button("back").clicked() {
                game_state.game_state = GameState::MainMenu;
            }
        });
}

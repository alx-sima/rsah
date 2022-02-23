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
                generare::tabla_from(&mut game_state.tabla,[
                    "N       ",
                    "  p",
                    "    N",
                    "",
                    "",
                    "",
                    "",
                    "R",
                ]);
                //generare::tabla_clasica(&mut game_state.tabla);
            }
            /*
            ui.vertical(|ui| {
                ui.label("Multiplayer");
                let mut ip = [0u8; 4];
                let mut port = 8080;

                ui.add(egui::Slider::new(&mut ip[0], 0..=255));
                ui.add(egui::Slider::new(&mut ip[1], 0..=255));
                ui.add(egui::Slider::new(&mut ip[2], 0..=255));
                ui.add(egui::Slider::new(&mut ip[3], 0..=255));
                ui.add(egui::Slider::new(&mut port, 0..=65535));
                if ui.button("Connect").clicked() {}
            });
            */

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

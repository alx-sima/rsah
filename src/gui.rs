use ggez_egui::EguiContext;

use crate::{Piesa, GameState, Culoare, State};

pub(crate) fn main_menu(game_state: &mut State, egui_ctx: &EguiContext, ctx: &mut ggez::Context) {
    egui::Window::new("egui")
        .title_bar(false)
        .show(egui_ctx, |ui| {
            if ui.button("START").clicked() {
                game_state.turn = Culoare::Alb;
                game_state.game_state = GameState::Game;
            }
            if ui.button("Editor").clicked() {
                game_state.piesa_selectata = Piesa::Pion;
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

    for i in game_state.tabla {
        println!("{:?}", i);
    }
}

pub(crate) fn editor(game_state: &mut State, egui_ctx: &EguiContext) {
    egui::Window::new("egui-editor")
        .title_bar(false)
        .show(egui_ctx, |ui| {
            egui::ComboBox::from_label("Piesa")
                .selected_text(format!("{:?}", game_state.piesa_selectata))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut game_state.piesa_selectata, Piesa::Pion, "Pion");
                    ui.selectable_value(&mut game_state.piesa_selectata, Piesa::Tura, "Tura");
                    ui.selectable_value(&mut game_state.piesa_selectata, Piesa::Cal, "Cal");
                    ui.selectable_value(&mut game_state.piesa_selectata, Piesa::Nebun, "Nebun");
                    ui.selectable_value(&mut game_state.piesa_selectata, Piesa::Regina, "Regina");
                    ui.selectable_value(&mut game_state.piesa_selectata, Piesa::Rege, "Rege");
                });
            if ui.button("help").clicked() {
                // TODO: help text pt editor
            }
            if ui.button("back").clicked() {
                game_state.game_state = GameState::MainMenu;
            }
        });
}

/* use bevy::prelude::*;
use bevy_egui::{egui::{self, Ui}, EguiContext, EguiPlugin};

pub fn ui_select_music(
    mut egui_ctx: ResMut<EguiContext>
) {
    egui::SidePanel::left("music_info_panel")
        .default_width(600.)
        .max_width(600.)
        .min_width(600.)
        .resizable(false)
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.label("ABC");
        });

    egui::SidePanel::right("music_select_panel")
        .default_width(600.)
        .max_width(600.)
        .min_width(600.)
        .resizable(false)
        .show(egui_ctx.ctx_mut(), |ui|{
            egui::ScrollArea::vertical()
            .max_width(f32::INFINITY)
            .show(ui, |ui|
            {
                ui.label("abcd");
            });
        });

        /* 
    egui::CentralPanel::default()
        .show(egui_ctx.ctx_mut(), |ui| {
            egui::Frame::none()
            .fill(egui::Color32::LIGHT_GREEN)
            .show(ui, |ui| {
                ui.label("Label with light green background");
                ui.heading("Central Panel");

                ui.horizontal(|ui| {
                    ui.label("Write something: ");
                });
            });
        });
        */
}

//window 기본설정 : https://github.com/bevyengine/bevy/blob/latest/examples/window/window_settings.rs
//
*/

use bevy::prelude::*;

//draw 
pub fn draw_game_playing_ui () {

}


use super::{AppComponent, UIPages};
use crate::app::{widgets::IconButton, App};
use eframe::egui::Layout;

pub struct Titlebar;

impl AppComponent for Titlebar {
    type Context = App;

    fn add(ctx: &mut Self::Context, ui: &mut eframe::egui::Ui) {
        ui.horizontal(|ui| {
            ui.with_layout(Layout::right_to_left(), |ui| {
                ui.add_space(15.0);

                let settings = IconButton::new(&ctx.resources.icons().settings).frame(false);
                if ui.add(settings).clicked() {
                    ctx.page = if ctx.page == UIPages::Main {
                        UIPages::Settings
                    } else {
                        ctx.pomodoro.update_config(ctx.config.pomodoro);
                        UIPages::Main
                    }
                }
            });
        });
    }
}

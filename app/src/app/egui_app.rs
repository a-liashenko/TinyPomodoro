use eframe::egui::{CentralPanel, Context};
use eframe::{Frame, Storage};

use super::components::{AppComponent, MainPage, SettingsPage, Titlebar, Topbar, UIPages};
use super::App;

impl eframe::App for App {
    fn on_exit_event(&mut self) -> bool {
        // TODO: Show error
        let _err = self.config.save();
        true
    }

    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        if !self.inited {
            ctx.set_visuals(self.resources.visuals().clone());
            ctx.set_fonts(self.resources.fonts());
            // ctx.set_debug_on_hover(true);

            self.resources
                .load_runtime(&self.config, ctx)
                .expect("Failed to load Resources::Runtime");

            self.inited = true;
        }

        self.process_timer();
        self.process_hotkeys();

        CentralPanel::default().show(ctx, |ui| {
            Titlebar::with_frame(self, ui, frame);
            ui.add_space(15.0);
            Topbar::add(self, ui);
            ui.add_space(15.0);

            match self.page {
                UIPages::Main => MainPage::add(self, ui),
                UIPages::Settings => SettingsPage::add(self, ui),
            }
        });
    }

    fn warm_up_enabled(&self) -> bool {
        true
    }

    fn clear_color(&self, _visuals: &eframe::egui::Visuals) -> eframe::egui::Rgba {
        self.config.style.background.into()
    }
}

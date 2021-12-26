use eframe::{
    egui::{CentralPanel, CtxRef},
    epi::{Frame, Storage},
};

use super::components::{AppComponent, MainPage, SettingsPage, Titlebar, UIPages};
use super::App;

impl eframe::epi::App for App {
    fn on_exit(&mut self) {
        // TODO: Show error
        let _err = self.config.save();
    }

    fn setup(&mut self, ctx: &CtxRef, frame: &mut Frame<'_>, _storage: Option<&dyn Storage>) {
        ctx.set_visuals(self.resources.visuals().clone());
        ctx.set_fonts(self.resources.fonts());
        // ctx.set_debug_on_hover(true);

        self.resources
            .load_runtime(&self.config, frame)
            .expect("Failed to load Resources::Runtime");
    }

    fn update(&mut self, ctx: &CtxRef, _frame: &mut Frame<'_>) {
        self.process_timer();
        self.process_hotkeys(ctx);

        CentralPanel::default().show(ctx, |ui| {
            ui.add_space(15.0);
            Titlebar::add(self, ui);
            ui.add_space(15.0);

            match self.page {
                UIPages::Main => MainPage::add(self, ui),
                UIPages::Settings => SettingsPage::add(self, ui),
            }
        });
    }

    fn name(&self) -> &str {
        crate::defines::APP_NAME
    }

    fn warm_up_enabled(&self) -> bool {
        true
    }

    fn clear_color(&self) -> eframe::egui::Rgba {
        self.config.style.background.into()
    }
}

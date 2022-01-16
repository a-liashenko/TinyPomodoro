use eframe::{
    egui::{CentralPanel, Context},
    epi::{Frame, Storage},
    GlWindow, WithGL,
};

use super::components::{AppComponent, MainPage, SettingsPage, Titlebar, Topbar, UIPages};
use super::App;

impl eframe::epi::App for App {
    fn on_exit(&mut self) {
        // TODO: Show error
        let _err = self.config.save();
    }

    fn setup(&mut self, ctx: &Context, frame: &Frame, _storage: Option<&dyn Storage>) {
        ctx.set_visuals(self.resources.visuals().clone());
        ctx.set_fonts(self.resources.fonts());
        // ctx.set_debug_on_hover(true);

        self.resources
            .load_runtime(&self.config, frame)
            .expect("Failed to load Resources::Runtime");
    }

    fn update(&mut self, ctx: &Context, frame: &Frame) {
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

impl WithGL for App {
    fn set_window(&mut self, window: std::sync::Arc<GlWindow>) {
        self.window = Some(window);
    }
}

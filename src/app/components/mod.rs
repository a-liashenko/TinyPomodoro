use eframe::egui::Ui;

mod controls;
mod timer;
mod titlebar;

mod main_page;
mod settings_page;

pub use main_page::MainPage;
pub use settings_page::SettingsPage;
pub use titlebar::Titlebar;

#[derive(Debug, PartialEq, Eq)]
pub enum UIPages {
    Main,
    Settings,
}

pub trait AppComponent {
    type Context;

    fn add(ctx: &mut Self::Context, ui: &mut Ui);
}

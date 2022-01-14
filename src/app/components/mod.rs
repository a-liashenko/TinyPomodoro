use eframe::{egui::Ui, epi::Frame};

mod controls;
mod timer;
mod titlebar;
mod topbar;

mod main_page;
mod settings_page;

pub use main_page::MainPage;
pub use settings_page::SettingsPage;
pub use titlebar::Titlebar;
pub use topbar::Topbar;

#[derive(Debug, PartialEq, Eq)]
pub enum UIPages {
    Main,
    Settings,
}

pub trait AppComponent {
    type Context;

    #[allow(unused)]
    fn add(ctx: &mut Self::Context, ui: &mut Ui) {}

    #[allow(unused)]
    fn with_frame(ctx: &mut Self::Context, ui: &mut Ui, frame: &Frame) {}
}

// #![windows_subsystem = "windows"]

mod app;
mod color;
mod config;
mod defines;
mod pomodoro;
mod resources;
mod serde_helpers;

use config::AppConfig;
use eframe::egui::vec2;

fn main() {
    let native_options = eframe::NativeOptions {
        resizable: false,
        initial_window_size: Some(vec2(360.0, 520.0)),
        decorated: false,
        ..Default::default()
    };

    // TODO: Show error
    let config = AppConfig::load().unwrap_or_default();
    let app = app::App::from_config(config);

    eframe::run_native(Box::new(app), native_options);
}

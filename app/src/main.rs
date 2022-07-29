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
    let config = AppConfig::load().unwrap_or_default();
    let native_options = eframe::NativeOptions {
        resizable: false,
        initial_window_size: Some(vec2(360.0, 520.0)),
        decorated: false,
        always_on_top: config.always_on_top,
        ..Default::default()
    };

    eframe::run_native(
        "TinyPomodoro",
        native_options,
        Box::new(move |_cc| {
            let app = app::App::from_config(config, _cc.win.clone());
            let app = Box::new(app);
            app
        }),
    );
}

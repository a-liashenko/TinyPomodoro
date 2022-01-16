use std::sync::Arc;

use self::components::UIPages;
use self::widgets::CircleConfig;
use crate::config::{Actions, AppConfig};
use crate::pomodoro::Status;
use crate::{pomodoro::Pomodoro, resources::ResourceLoader};

use eframe::egui::{Context, Stroke};
use eframe::{winit::window::Window, GlWindow};

mod components;
mod egui_app;
mod widgets;

pub struct App {
    window: Option<Arc<GlWindow>>,
    resources: ResourceLoader,
    config: AppConfig,

    pomodoro: Pomodoro,
    circle: CircleConfig,

    page: UIPages,
}

impl Default for App {
    fn default() -> Self {
        let cfg = AppConfig::default();
        Self::from_config(cfg)
    }
}

impl App {
    pub fn from_config(config: AppConfig) -> Self {
        let resources = ResourceLoader::new(&config);

        let style = resources.visuals().widgets.noninteractive;

        let pomodoro = Pomodoro::new(config.pomodoro);
        let circle = CircleConfig {
            background: Some((3.0, style.fg_stroke.color).into()),
            foreground: Some(Self::status_stroke(&config, Status::Focus)),
            radius: 120.0,
            ..Default::default()
        };

        Self {
            window: None,

            config,
            resources,

            pomodoro,
            circle,

            page: UIPages::Main,
        }
    }

    fn status_stroke(cfg: &AppConfig, status: Status) -> Stroke {
        let color = match status {
            Status::Short => cfg.style.circle_short_break,
            Status::Long => cfg.style.circle_long_break,
            Status::Focus => cfg.style.circle_focus,
        };

        Stroke::new(10.0, color)
    }

    fn process_hotkeys(&mut self, ctx: &Context) {
        let input = ctx.input();
        let action = match self.resources.hotkeys().next_action(&input) {
            Some(v) => v,
            None => return,
        };

        match action {
            Actions::ToggleTimer => self.pomodoro.toggle(),
            Actions::ResetTimer => self.pomodoro.reset(),
        }
    }

    fn process_timer(&mut self) {
        use eframe::winit::window::UserAttentionType;
        let status = match self.pomodoro.try_next() {
            Some(v) => v,
            None => return,
        };

        if !self.config.muted {
            self.resources.notification().play();
        }

        self.circle.foreground = Some(Self::status_stroke(&self.config, status));
        self.window()
            .request_user_attention(Some(UserAttentionType::Critical));
    }

    fn window(&self) -> &Window {
        self.window.as_ref().unwrap().window()
    }
}

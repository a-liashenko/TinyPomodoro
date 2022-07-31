use std::rc::Rc;

use self::components::UIPages;
use self::widgets::CircleConfig;
use crate::config::{Actions, AppConfig};
use crate::pomodoro::Status;
use crate::{pomodoro::Pomodoro, resources::ResourceLoader};

use eframe::egui::Stroke;
use eframe::CreationContext;

mod components;
mod egui_app;
mod widgets;

pub type GlWindow = glutin::ContextWrapper<glutin::PossiblyCurrent, glutin::window::Window>;

pub struct App {
    window: Rc<GlWindow>,
    resources: ResourceLoader,
    config: AppConfig,

    pomodoro: Pomodoro,
    circle: CircleConfig,

    page: UIPages,
}

impl App {
    pub fn from_config(config: AppConfig, cc: &CreationContext) -> Self {
        let mut resources = ResourceLoader::new(&config);
        let style = resources.visuals().widgets.noninteractive;

        let pomodoro = Pomodoro::new(config.pomodoro);
        let circle = CircleConfig {
            background: Some((3.0, style.fg_stroke.color).into()),
            foreground: Some(Self::status_stroke(&config, Status::Focus)),
            radius: 120.0,
            ..Default::default()
        };

        cc.egui_ctx.set_visuals(resources.visuals().clone());
        cc.egui_ctx.set_fonts(resources.fonts());
        resources
            .load_runtime(&config, &cc.egui_ctx)
            .expect("Failed to load Resources::Runtime");

        Self {
            window: cc.win.clone(),

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

    fn process_hotkeys(&mut self) {
        let action = match self.resources.hotkeys().next_action() {
            Some(v) => v,
            None => return,
        };

        match action {
            Actions::ToggleTimer => self.pomodoro.toggle(),
            Actions::ResetTimer => self.pomodoro.reset(),
            Actions::ToggleMinimized => {
                // TODO: Add window helper for multiple OS
            }
        }
    }

    fn process_timer(&mut self) {
        use winit::window::UserAttentionType;
        let status = match self.pomodoro.try_next() {
            Some(v) => v,
            None => return,
        };

        if !self.config.muted {
            self.resources.notification().play();
        }

        self.circle.foreground = Some(Self::status_stroke(&self.config, status));

        if self.config.timer_notification {
            self.window()
                .request_user_attention(Some(UserAttentionType::Critical));
        }
    }

    fn window(&self) -> &glutin::window::Window {
        self.window.window()
    }
}

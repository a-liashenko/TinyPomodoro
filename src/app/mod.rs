use self::components::{AppComponent, MainPage, SettingsPage, Titlebar, UIPages};
use self::widgets::CircleConfig;
use crate::config::AppConfig;
use crate::pomodoro::Status;
use crate::{pomodoro::Pomodoro, resources::ResourceLoader};
use eframe::egui::{CentralPanel, CtxRef, Stroke};
use eframe::epi::{Frame, Storage};

mod components;
mod widgets;

pub struct App {
    resources: ResourceLoader,
    config: AppConfig,

    pomodoro: Pomodoro,
    circle: CircleConfig,

    page: UIPages,
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
}

impl Default for App {
    fn default() -> Self {
        let cfg = AppConfig::default();
        Self::from_config(cfg)
    }
}

impl eframe::epi::App for App {
    fn name(&self) -> &str {
        crate::defines::APP_NAME
    }

    fn clear_color(&self) -> eframe::egui::Rgba {
        self.config.style.background.into()
    }

    fn on_exit(&mut self) {
        // TODO: Show error
        let _err = self.config.save();
    }

    fn setup(&mut self, ctx: &CtxRef, _frame: &mut Frame<'_>, _storage: Option<&dyn Storage>) {
        ctx.set_visuals(self.resources.visuals().clone());
        ctx.set_fonts(self.resources.fonts());
        // ctx.set_debug_on_hover(true);

        let alloc = _frame.tex_allocator();
        self.resources
            .load_icons(alloc)
            .expect("Failed to load icons");
    }

    fn update(&mut self, ctx: &CtxRef, _frame: &mut Frame<'_>) {
        if let Some(status) = self.pomodoro.try_next() {
            if !self.config.muted {
                self.resources.notification().play();
            }

            self.circle.foreground = Some(Self::status_stroke(&self.config, status));
        }

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
}

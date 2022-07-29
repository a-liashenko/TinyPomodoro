use self::{hotkeys::Hotkeys, notification::Notification, runtime::Runtime, theme::Theme};
use crate::config::AppConfig;
use anyhow::Result;
use eframe::egui::{Context, FontDefinitions, Style, Visuals};

pub mod icon;

mod fonts;
mod hotkeys;
mod icon_loader;
mod notification;
mod runtime;
mod theme;

pub use self::icon_loader::Icons;

pub struct ResourceLoader {
    theme: Theme,
    notification: Notification,
    hotkeys: Hotkeys,
    runtime: Option<Runtime>,
}

impl ResourceLoader {
    pub fn new(config: &AppConfig) -> Self {
        // TODO: Show error
        let notification = config
            .load_sound()
            .unwrap_or(None)
            .map(Notification::new)
            .unwrap_or_default();

        ResourceLoader {
            notification,
            theme: Theme::new(&config.style),
            runtime: None,
            hotkeys: Hotkeys::new(config),
        }
    }

    fn runtime(&self) -> &Runtime {
        // TODO: Unwrap unchecked?
        self.runtime
            .as_ref()
            .expect("Resources runtime not allocated")
    }

    pub fn load_runtime(&mut self, cfg: &AppConfig, ctx: &Context) -> Result<()> {
        let runtime = Runtime::new(cfg, ctx)?;
        self.runtime = Some(runtime);
        Ok(())
    }

    pub fn fonts(&self) -> FontDefinitions {
        fonts::load_fonts()
    }

    pub fn visuals(&self) -> &Visuals {
        self.theme.visuals()
    }

    pub fn slider(&self) -> &Style {
        self.theme.slider()
    }

    pub fn checkbox(&self) -> &Style {
        self.theme.checkbox()
    }

    pub fn notification(&mut self) -> &mut Notification {
        &mut self.notification
    }

    pub fn icons(&self) -> &Icons {
        &self.runtime().icons
    }

    pub fn hotkeys(&mut self) -> &mut Hotkeys {
        &mut self.hotkeys
    }
}

use self::{notification::Notification, theme::Theme};
use crate::config::AppConfig;
use anyhow::Result;
use eframe::{
    egui::{FontDefinitions, Style, Visuals},
    epi::TextureAllocator,
};

pub mod icon;

mod fonts;
mod icon_loader;
mod notification;
mod theme;

pub use self::icon_loader::Icons;

pub struct ResourceLoader {
    theme: Theme,
    icons: Option<Icons>,
    notification: Notification,
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
            icons: None,
        }
    }

    pub fn load_icons(&mut self, alloc: &mut dyn TextureAllocator) -> Result<()> {
        if self.icons.is_none() {
            let icons = Icons::preload(alloc)?;
            self.icons = Some(icons);
        }
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

    pub fn icons(&self) -> &Icons {
        self.icons.as_ref().unwrap()
    }

    pub fn notification(&mut self) -> &mut Notification {
        &mut self.notification
    }
}

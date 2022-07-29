use crate::config::AppStyle;
use eframe::egui::{Style, Visuals};

pub struct Theme {
    visuals: Visuals,
    slider: Style,
    checkbox: Style,
}

impl Theme {
    pub fn new(cfg: &AppStyle) -> Self {
        let mut visuals = Visuals::dark();
        visuals.widgets.noninteractive.bg_fill = cfg.background;
        visuals.widgets.noninteractive.fg_stroke.color = cfg.foreground;
        visuals.widgets.noninteractive.rounding = 0.0.into();

        let mut slider = Style {
            visuals: visuals.clone(),
            ..Default::default()
        };
        slider.visuals.widgets.inactive.bg_fill = cfg.rounds;
        slider.visuals.widgets.inactive.fg_stroke.width = 0.0;
        slider.visuals.widgets.inactive.expansion = 3.0;

        slider.visuals.widgets.hovered = slider.visuals.widgets.inactive;
        slider.visuals.widgets.active = slider.visuals.widgets.inactive;

        let mut checkbox = Style {
            visuals: visuals.clone(),
            ..Default::default()
        };
        checkbox.visuals.widgets.noninteractive.fg_stroke.color = cfg.foreground;
        checkbox.visuals.widgets.noninteractive.bg_stroke.width = 0.5;
        checkbox.visuals.widgets.noninteractive.bg_stroke.color = cfg.foreground;
        checkbox.visuals.widgets.noninteractive.expansion = 1.0;
        checkbox.spacing.icon_spacing = 10.0;

        checkbox.visuals.widgets.inactive = checkbox.visuals.widgets.noninteractive;
        checkbox.visuals.widgets.active = checkbox.visuals.widgets.noninteractive;
        checkbox.visuals.widgets.hovered = checkbox.visuals.widgets.noninteractive;

        Self {
            visuals,
            slider,
            checkbox,
        }
    }

    pub fn visuals(&self) -> &Visuals {
        &self.visuals
    }

    pub fn slider(&self) -> &Style {
        &self.slider
    }

    pub fn checkbox(&self) -> &Style {
        &self.checkbox
    }
}

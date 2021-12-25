use crate::config::AppStyle;
use eframe::egui::{Style, Visuals};

pub struct Theme {
    visuals: Visuals,
    slider: Style,
}

impl Theme {
    pub fn new(cfg: &AppStyle) -> Self {
        let mut visuals = Visuals::dark();
        visuals.widgets.noninteractive.bg_fill = cfg.background;
        visuals.widgets.noninteractive.fg_stroke.color = cfg.foreground;
        visuals.widgets.noninteractive.corner_radius = 0.0;

        let mut slider = Style {
            visuals: visuals.clone(),
            ..Default::default()
        };
        slider.visuals.widgets.inactive.bg_fill = cfg.circle_focus;
        slider.visuals.widgets.inactive.fg_stroke.width = 0.0;
        slider.visuals.widgets.inactive.expansion = 3.0;

        slider.visuals.widgets.hovered = slider.visuals.widgets.inactive;
        slider.visuals.widgets.active = slider.visuals.widgets.inactive;

        Self { visuals, slider }
    }

    pub fn visuals(&self) -> &Visuals {
        &self.visuals
    }

    pub fn slider(&self) -> &Style {
        &self.slider
    }
}

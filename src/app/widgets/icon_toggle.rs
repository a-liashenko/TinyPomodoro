use crate::resources::icon::Icon;
use eframe::egui::{ImageButton, Response, Ui, Widget};

pub struct IconToggle<'a> {
    enabled: &'a Icon,
    disabled: &'a Icon,
    with_frame: bool,
    toggled: bool,
}

impl<'a> IconToggle<'a> {
    pub fn new(enabled: &'a Icon, disabled: &'a Icon, toggled: bool) -> Self {
        Self {
            enabled,
            disabled,
            with_frame: false,
            toggled,
        }
    }

    pub fn frame(mut self, enabled: bool) -> Self {
        self.with_frame = enabled;
        self
    }
}

impl<'a> Widget for IconToggle<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let btn = if self.toggled {
            ImageButton::new(self.enabled.id(), self.enabled.size_f32())
        } else {
            ImageButton::new(self.disabled.id(), self.disabled.size_f32())
        }
        .frame(self.with_frame);

        ui.add(btn)
            .on_hover_cursor(eframe::egui::CursorIcon::PointingHand)
    }
}

use crate::resources::icon::Icon;
use eframe::egui::{CursorIcon, ImageButton, Response, Ui, Widget};

pub struct IconButton<'a> {
    icon: &'a Icon,
    with_frame: bool,
}

impl<'a> IconButton<'a> {
    pub fn new(icon: &'a Icon) -> Self {
        Self {
            icon,
            with_frame: false,
        }
    }
}

impl<'a> Widget for IconButton<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let btn = ImageButton::new(self.icon.id(), self.icon.size_f32()).frame(self.with_frame);
        let response = ui.add(btn);
        response.on_hover_cursor(CursorIcon::PointingHand)
    }
}

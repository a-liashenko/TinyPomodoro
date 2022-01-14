use eframe::egui::{Color32, Layout, Rect, Sense, Shape, Stroke, Ui};
use eframe::epi::Frame;

use super::AppComponent;
use crate::app::{widgets::IconButton, App};
use crate::color::ColorHex;

pub struct Titlebar;

// TODO: Export color to config file
// #78909c
// Change SVG color for icons and icons group

impl AppComponent for Titlebar {
    type Context = App;

    fn with_frame(ctx: &mut Self::Context, ui: &mut Ui, frame: &Frame) {
        let icons = ctx.resources.icons();
        let width = ui.available_width();

        ui.horizontal(|ui| {
            ui.with_layout(Layout::right_to_left(), |ui| {
                if ui.add(IconButton::new(&icons.close)).clicked() {
                    frame.quit();
                };
                ui.add(IconButton::new(&icons.minimize));
                ui.add(IconButton::new(&icons.pin_on));
                // ui.add(IconButton::new(&icons.pin_off).frame(false));
            });

            let rect = Rect::from_min_size((0., 0.).into(), (width - 21. * 4., 21.).into());
            let response = ui.allocate_rect(rect, Sense::hover());
            if response.hovered() {
                frame.drag_window();
            }
        });

        ui.add_space(3.);

        // TODO: Check remove spacing from default Style
        let (response, painter) = ui.allocate_painter((360., 2.).into(), Sense::hover());
        if response.hovered() {
            frame.drag_window()
        }

        let rect = response.rect;
        let stroke = Stroke::new(0.5, Color32::from_hex_panic("#78909c"));
        let shape = Shape::line_segment([rect.min, rect.max], stroke);
        painter.add(shape);
    }
}

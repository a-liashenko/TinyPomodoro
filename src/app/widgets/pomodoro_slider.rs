use chrono::Duration;
use eframe::egui::{Color32, Response, Style, Ui, Widget};

use super::styled_slider::StyledSlider;

// Second
const DURATION_MIN: f64 = 10.0;
const DURATION_MAX: f64 = (60 * 60) as f64;

fn format_time(val: f64) -> String {
    let minutes = (val / 60.0) as u32;
    let seconds = val as u32 - minutes * 60;
    format!("{:02}:{:02}", minutes, seconds)
}

pub struct PomodoroSlider<'a> {
    pub title: String,
    pub duration: &'a mut Duration,

    pub color: Color32,
    pub style: &'a Style,
}

impl<'a> Widget for PomodoroSlider<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let mut value = self.duration.num_seconds() as f64;
        let mut style = self.style.clone();
        style.visuals.widgets.inactive.bg_fill = self.color;
        style.visuals.widgets.active.bg_fill = self.color;
        style.visuals.widgets.hovered.bg_fill = self.color;

        ui.horizontal(|ui| {
            ui.add_space(25.0);

            let slider = StyledSlider::new(self.title, DURATION_MIN, DURATION_MAX, &mut value)
                .with_formatter(format_time)
                .with_style(&style);

            if ui.add(slider).changed() {
                let new = Duration::seconds(value as i64);
                if &new != self.duration {
                    *self.duration = new;
                }
            }
        })
        .response
    }
}

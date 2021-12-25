use super::progress_circle::{CircleConfig, ProgressCircle};
use crate::pomodoro::{Pomodoro, Status};
use chrono::Duration;
use eframe::egui::{Ui, Widget};
use std::borrow::Cow;

fn status2text(status: Status) -> &'static str {
    match status {
        Status::Focus => "FOCUS",
        Status::Long => "LONG BREAK",
        Status::Short => "SHORT BREAK",
    }
}

#[derive(Debug, Clone)]
pub struct PomodoroTimer<'a> {
    time_left: String,
    status: String,
    angle: f32,
    circle: &'a CircleConfig,
}

impl<'a> PomodoroTimer<'a> {
    pub fn new(pomodoro: &Pomodoro, circle: &'a CircleConfig) -> Self {
        let status = pomodoro.status();
        let angle = {
            let duration = pomodoro.config().get_duration(&status);
            let left = pomodoro.time_left();
            Self::map_timer(duration, left)
        };

        let time_text = Self::get_left_text(pomodoro.time_left());
        let status_text = status2text(status);

        Self {
            angle,
            circle,
            time_left: time_text,
            status: status_text.into(),
        }
    }

    fn map_timer(duration: Duration, left: Duration) -> f32 {
        let total = duration.num_milliseconds() as f32;
        let value = total - left.num_milliseconds() as f32;
        let once = total / 360.0;
        value / once
    }

    fn get_left_text(duration: Duration) -> String {
        let seconds = duration.num_seconds() % 60;
        let minutes = (duration.num_seconds() / 60) % 60;
        format!("{:02}:{:02}", minutes, seconds)
    }
}

impl<'a> Widget for PomodoroTimer<'a> {
    fn ui(self, ui: &mut Ui) -> eframe::egui::Response {
        let circle = ProgressCircle {
            angle: self.angle,
            text_main: Cow::Owned(self.time_left),
            text_additional: Cow::Owned(self.status),
            config: Cow::Borrowed(self.circle),
        };

        if self.angle < 360.0 {
            ui.ctx().request_repaint()
        }

        ui.add(circle)
    }
}

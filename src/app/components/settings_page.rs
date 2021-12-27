use eframe::egui::{Color32, InnerResponse, Response, Style, Ui};

use super::AppComponent;
use crate::app::{
    widgets::{styled_slider::StyledSlider, PomodoroSlider},
    App,
};

const VERTICAL_PADDING: f32 = 25.0;
const LEFT_PADDING: f32 = 25.0;

// TODO: Cache sliders in Vec
pub struct SettingsPage;

impl SettingsPage {
    fn draw_rounds_slider(ui: &mut Ui, style: &Style, color: Color32, val: &mut f64) -> Response {
        let mut response = None;
        ui.horizontal(|ui| {
            ui.add_space(25.0);
            let mut style = style.clone();
            style.visuals.widgets.inactive.bg_fill = color;
            style.visuals.widgets.active.bg_fill = color;
            style.visuals.widgets.hovered.bg_fill = color;

            let slider = StyledSlider::new("Rounds", 1.0, 16.0, val).with_style(&style);
            response = Some(ui.add(slider));
        });

        response.unwrap()
    }

    fn paddig<R>(ui: &mut Ui, add: impl FnOnce(&mut Ui) -> R) -> InnerResponse<R> {
        ui.horizontal(|ui| {
            ui.add_space(LEFT_PADDING);
            add(ui)
        })
    }
}

impl AppComponent for SettingsPage {
    type Context = App;

    fn add(ctx: &mut Self::Context, ui: &mut eframe::egui::Ui) {
        ui.add_space(30.0);

        let slider = PomodoroSlider {
            title: "Short break".into(),
            color: ctx.config.style.circle_short_break,
            style: ctx.resources.slider(),
            duration: &mut ctx.config.pomodoro.short,
        };
        ui.add(slider);
        ui.add_space(VERTICAL_PADDING);

        let slider = PomodoroSlider {
            title: "Long break".into(),
            color: ctx.config.style.circle_long_break,
            style: ctx.resources.slider(),
            duration: &mut ctx.config.pomodoro.long,
        };
        ui.add(slider);
        ui.add_space(VERTICAL_PADDING);

        let slider = PomodoroSlider {
            title: "Focus".into(),
            color: ctx.config.style.circle_focus,
            style: ctx.resources.slider(),
            duration: &mut ctx.config.pomodoro.focus,
        };
        ui.add(slider);
        ui.add_space(VERTICAL_PADDING);

        let mut val: f64 = ctx.config.pomodoro.rounds as f64;
        let slider = Self::draw_rounds_slider(
            ui,
            ctx.resources.slider(),
            ctx.config.style.rounds,
            &mut val,
        );
        if slider.changed() {
            ctx.config.pomodoro.rounds = val as u16;
        }
        ui.add_space(VERTICAL_PADDING);

        Self::paddig(ui, |ui| {
            ui.set_style(ctx.resources.checkbox().clone());
            let ch = eframe::egui::Checkbox::new(&mut ctx.config.portable, "Portable mode")
                .text_style(eframe::egui::TextStyle::Monospace);
            ui.add(ch);
        });
    }
}

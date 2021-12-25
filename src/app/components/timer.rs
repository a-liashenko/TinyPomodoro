use super::AppComponent;
use crate::app::widgets::{IconToggle, PomodoroTimer};
use crate::app::App;

pub struct Timer;

impl AppComponent for Timer {
    type Context = App;

    fn add(ctx: &mut Self::Context, ui: &mut eframe::egui::Ui) {
        let diameter = ctx.circle.radius * 2.0;

        ui.horizontal(|ui| {
            ui.horizontal(|ui| {
                let padding = (ui.available_width() - diameter) / 2.0;
                ui.add_space(padding);

                let timer = PomodoroTimer::new(&ctx.pomodoro, &ctx.circle);
                ui.add(timer);
            });
        });

        ui.add_space(25.0);

        ui.horizontal(|ui| {
            let play = &ctx.resources.icons().play;
            let pause = &ctx.resources.icons().pause;
            ui.add_space((ui.available_width() - play.width as f32) / 2.0);

            let btn = IconToggle::new(pause, play, ctx.pomodoro.is_running()).frame(false);
            if ui.add(btn).clicked() {
                ctx.pomodoro.toggle();
            };
        });

        // ui.add_space(25.0);
    }
}

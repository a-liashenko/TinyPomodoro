use super::controls::Controls;
use super::timer::Timer;
use super::AppComponent;
use crate::app::App;

pub struct MainPage;

impl AppComponent for MainPage {
    type Context = App;

    fn add(ctx: &mut Self::Context, ui: &mut eframe::egui::Ui) {
        let diameter = ctx.circle.radius * 2.0;
        ui.add_space((ui.available_height() - diameter) / 2.0 - 75.0);

        Timer::add(ctx, ui);
        Controls::add(ctx, ui);
    }
}

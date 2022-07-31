use std::borrow::Cow;

use eframe::{
    egui::{
        Align2, Color32, Painter, Pos2, Response, Sense, Shape, Stroke, TextStyle, Ui, Vec2, Widget,
    },
    epaint::FontId,
};

const CIRCLE_POINTS: f32 = 1500.0;

// TODO: Cache me
fn calc_background(radius: f32, center: Pos2) -> Vec<Pos2> {
    let points = CIRCLE_POINTS;
    let single = points / 360.0;

    (0..points as u32)
        .map(|p| {
            let current = p as f32 / single;
            let (sin, cos) = current.to_radians().sin_cos();
            let x = center.x + radius * -sin;
            let y = center.y + radius * -cos;
            Pos2 { x, y }
        })
        .collect()
}

fn calc_foreground(radius: f32, center: Pos2, angle: f32) -> Vec<Pos2> {
    let points = CIRCLE_POINTS;
    let single = points / 360.0;

    (0..points as u32)
        .skip_while(|p| *p as f32 / single <= angle)
        .map(|p| {
            let current = p as f32 / single;
            let (sin, cos) = current.to_radians().sin_cos();
            let x = center.x + radius * -sin;
            let y = center.y + radius * -cos;
            Pos2 { x, y }
        })
        .collect()
}

#[derive(Debug, Clone)]
pub struct CircleConfig {
    pub radius: f32,
    pub background: Option<Stroke>,
    pub foreground: Option<Stroke>,

    pub text_main: Option<(Color32, TextStyle)>,
    pub text_additional: Option<(Color32, TextStyle)>,
}

impl Default for CircleConfig {
    fn default() -> Self {
        Self {
            radius: 32.0,
            background: Default::default(),
            foreground: Default::default(),
            text_main: Default::default(),
            text_additional: Default::default(),
        }
    }
}

pub struct ProgressCircle<'a> {
    pub text_main: Cow<'a, str>,
    pub text_additional: Cow<'a, str>,
    pub angle: f32,
    pub config: Cow<'a, CircleConfig>,
}

impl<'a> ProgressCircle<'a> {
    fn draw_main_text(&self, mut center: Pos2, painter: &Painter) {
        let (color, style) = &self.config.text_main.clone().unwrap_or_else(|| {
            let visuals = &painter.ctx().style().visuals.widgets.inactive;
            (visuals.fg_stroke.color, TextStyle::Heading)
        });
        center.y -= 14.0;

        painter.text(
            center,
            Align2::CENTER_CENTER,
            &self.text_main,
            FontId::monospace(48.0),
            color.clone(),
        );
    }

    fn draw_additional_text(&self, mut center: Pos2, painter: &Painter) {
        let (color, style) = self.config.text_main.clone().unwrap_or_else(|| {
            let visuals = &painter.ctx().style().visuals.widgets.inactive;
            (visuals.fg_stroke.color, TextStyle::Body)
        });
        center.y += 8.0 + self.config.radius / 3.0;

        painter.text(
            center,
            Align2::CENTER_CENTER,
            &self.text_additional,
            FontId::proportional(16.0),
            color.clone(),
        );
    }

    fn draw_circle(&self, center: Pos2, painter: &Painter) {
        let visuals = &painter.ctx().style().visuals;
        let stroke = self
            .config
            .background
            .unwrap_or_else(|| (10.0, visuals.extreme_bg_color).into());
        let bg = calc_background(self.config.radius - stroke.width - 1.0, center);
        let bg = Shape::closed_line(bg, stroke);
        painter.add(bg);

        if self.angle < 360.0 {
            let stroke = self
                .config
                .foreground
                .unwrap_or_else(|| (10.0, visuals.faint_bg_color).into());

            let fg = calc_foreground(self.config.radius - stroke.width, center, self.angle);
            let fg = Shape::line(fg, stroke);
            painter.add(fg);
        }
    }
}

impl<'a> Widget for ProgressCircle<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let cfg = &self.config;

        let (response, painter) = {
            let size = Vec2::splat(cfg.radius * 2.0);
            ui.allocate_painter(size, Sense::hover())
        };
        let rect = response.rect;
        let center = rect.center();

        self.draw_main_text(center, &painter);
        self.draw_additional_text(center, &painter);
        self.draw_circle(center, &painter);

        response
    }
}

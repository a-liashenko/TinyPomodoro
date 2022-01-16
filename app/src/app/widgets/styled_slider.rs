use eframe::egui::{Label, Response, RichText, Slider, Style, Ui, Widget};

pub type Formatter = fn(f64) -> String;

pub struct StyledSlider<'a> {
    title: String,
    min: f64,
    max: f64,
    value: &'a mut f64,

    formatter: Option<Formatter>,
    style: Option<&'a Style>,
}

impl<'a> StyledSlider<'a> {
    pub fn new(title: impl Into<String>, min: f64, max: f64, value: &'a mut f64) -> Self {
        Self {
            min,
            max,
            value,
            title: title.into(),
            formatter: None,
            style: None,
        }
    }

    pub fn with_style(mut self, style: &'a Style) -> Self {
        self.style = Some(style);
        self
    }

    pub fn with_formatter(mut self, formatter: Formatter) -> Self {
        self.formatter = Some(formatter);
        self
    }
}

impl<'a> Widget for StyledSlider<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let mut result = None;

        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                let space = (ui.available_width() - 30.0, ui.available_height());
                let text = RichText::new(&self.title).monospace();
                ui.add_sized(space, Label::new(text));
            });

            ui.add_space(15.0);

            ui.horizontal(|ui| {
                if let Some(style) = self.style {
                    *ui.style_mut() = style.clone();
                }
                ui.style_mut().spacing.slider_width = ui.available_width() - 70.0;

                let range = self.min..=self.max;
                let slider = Slider::new(self.value, range).show_value(false).integer();
                result = Some(ui.add(slider));

                let text = match self.formatter {
                    Some(fmt) => fmt(*self.value),
                    None => (*self.value as u64).to_string(),
                };

                ui.add_space(5.0);

                ui.monospace(text);
            })
        });

        result.unwrap()
    }
}

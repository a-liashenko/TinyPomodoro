use crate::defines::fonts::{FONT_ROBOTO, FONT_ROBOTO_MONO};
use eframe::egui::{FontData, FontDefinitions, Style, TextStyle};
use eframe::epaint::FontId;

pub fn register_fonts(style: &mut Style) {
    use eframe::epaint::FontFamily::{Monospace, Proportional};

    style.text_styles = [
        (TextStyle::Body, FontId::new(21.0, Proportional)),
        (TextStyle::Button, FontId::new(18.0, Monospace)),
        (TextStyle::Monospace, FontId::new(18.0, Monospace)),
        (TextStyle::Small, FontId::new(16.0, Proportional)),
        (TextStyle::Heading, FontId::new(48.0, Proportional)),
    ]
    .into();
}

pub fn load_fonts() -> FontDefinitions {
    use eframe::epaint::FontFamily::{Monospace, Proportional};

    let mut fonts = FontDefinitions::default();
    let roboto = FontData::from_static(FONT_ROBOTO);
    let roboto_mono = FontData::from_static(FONT_ROBOTO_MONO);

    fonts.font_data.insert("roboto".into(), roboto);
    fonts.font_data.insert("roboto_mono".into(), roboto_mono);

    fonts.families.insert(Monospace, vec!["roboto_mono".into()]);
    fonts.families.insert(Proportional, vec!["roboto".into()]);

    fonts
}

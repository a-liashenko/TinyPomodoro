use crate::defines::fonts::{FONT_ROBOTO, FONT_ROBOTO_MONO};
use eframe::egui::{FontData, FontDefinitions};

// TODO: Try to do something with different font sizes
// Allow custom TextStyles or Sizes?

pub fn load_fonts() -> FontDefinitions {
    use eframe::egui::{FontFamily, TextStyle};

    let mut fonts = FontDefinitions::default();
    let roboto = FontData::from_static(FONT_ROBOTO);
    let roboto_mono = FontData::from_static(FONT_ROBOTO_MONO);

    fonts.font_data.insert("roboto".into(), roboto);
    fonts.font_data.insert("roboto_mono".into(), roboto_mono);

    fonts
        .fonts_for_family
        .get_mut(&FontFamily::Proportional)
        .unwrap()
        .insert(0, "roboto_mono".into());

    fonts
        .fonts_for_family
        .get_mut(&FontFamily::Monospace)
        .unwrap()
        .insert(0, "roboto".into());

    fonts
        .family_and_size
        .insert(TextStyle::Body, (FontFamily::Proportional, 21.0));

    fonts
        .family_and_size
        .insert(TextStyle::Button, (FontFamily::Proportional, 18.0));

    fonts
        .family_and_size
        .insert(TextStyle::Monospace, (FontFamily::Monospace, 18.0));

    fonts
        .family_and_size
        .insert(TextStyle::Small, (FontFamily::Proportional, 16.0));

    fonts
        .family_and_size
        .insert(TextStyle::Heading, (FontFamily::Proportional, 48.0));

    fonts
}

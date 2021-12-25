use colorsys::Rgb;
use eframe::egui::Color32;

pub trait ColorHex: Sized {
    fn to_hex(&self) -> String;
    fn from_hex(hex: &str) -> Option<Self>;

    fn from_hex_panic(hex: &str) -> Self {
        Self::from_hex(hex).unwrap_or_else(|| panic!("Failed to parse color: {}", hex))
    }
}

impl ColorHex for Color32 {
    fn to_hex(&self) -> String {
        let color = Rgb::new(
            self.r().into(),
            self.g().into(),
            self.b().into(),
            Some(self.a().into()),
        );
        color.to_hex_string()
    }

    fn from_hex(hex: &str) -> Option<Self> {
        if let Ok(color) = Rgb::from_hex_str(hex) {
            let color =
                Color32::from_rgb(color.red() as u8, color.green() as u8, color.blue() as u8);
            return Some(color);
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color() {
        let blue = Color32::BLUE;
        let hex = blue.to_hex();
        assert_eq!(hex, "#0000ff");
        assert_eq!(Color32::BLUE, Color32::from_hex_panic(&hex));
    }
}

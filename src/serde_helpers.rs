use chrono::Duration;
use eframe::egui::Color32;
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Test {
    #[serde(with = "color")]
    duration: Color32,
}

pub mod duration {
    use super::*;

    pub fn serialize<S>(val: &Duration, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_i64(val.num_milliseconds())
    }

    pub fn deserialize<'de, D>(deser: D) -> Result<Duration, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v = i64::deserialize(deser)?;
        Ok(Duration::milliseconds(v))
    }
}

pub mod color {
    use super::*;
    use crate::color::ColorHex;

    pub fn serialize<S>(val: &Color32, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let hex = val.to_hex();
        serializer.serialize_str(&hex)
    }

    pub fn deserialize<'de, D>(deser: D) -> Result<Color32, D::Error>
    where
        D: Deserializer<'de>,
    {
        let val = String::deserialize(deser)?;
        let color = Color32::from_hex_panic(&val);
        Ok(color)
    }
}

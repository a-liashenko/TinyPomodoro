use super::{fonts, Icons};
use crate::config::AppConfig;
use anyhow::Result;
use eframe::egui::Context;

pub struct Runtime {
    pub icons: Icons,
}

impl Runtime {
    pub fn new(_cfg: &AppConfig, ctx: &Context) -> Result<Self> {
        let mut style = (*ctx.style()).clone();

        let icons = Icons::preload(ctx)?;
        fonts::register_fonts(&mut style);

        ctx.set_style(style);
        Ok(Self { icons })
    }
}

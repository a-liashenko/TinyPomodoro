use super::Icons;
use crate::config::AppConfig;
use anyhow::Result;
use eframe::egui::Context;

pub struct Runtime {
    pub icons: Icons,
}

impl Runtime {
    pub fn new(_cfg: &AppConfig, ctx: &Context) -> Result<Self> {
        let icons = Icons::preload(ctx)?;
        Ok(Self { icons })
    }
}

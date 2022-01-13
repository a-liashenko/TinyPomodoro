use super::Icons;
use crate::config::AppConfig;
use anyhow::Result;
use eframe::epi::Frame;

pub struct Runtime {
    pub icons: Icons,
}

impl Runtime {
    pub fn new(_cfg: &AppConfig, frame: &Frame) -> Result<Self> {
        let icons = Icons::preload(frame)?;
        Ok(Self { icons })
    }
}

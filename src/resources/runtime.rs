use super::Icons;
use crate::config::AppConfig;
use anyhow::Result;
use eframe::epi::Frame;

pub struct Runtime {
    pub icons: Icons,
}

impl Runtime {
    pub fn new(_cfg: &AppConfig, frame: &mut Frame<'_>) -> Result<Self> {
        let allocator = frame.tex_allocator();
        let icons = Icons::preload(allocator)?;
        Ok(Self { icons })
    }
}

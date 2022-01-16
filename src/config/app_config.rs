use std::path::{Path, PathBuf};
use std::{fs::File, io::Read, io::Write};

use crate::defines::APP_CONFIG_NAME;
use crate::pomodoro::PomodoroCfg;
use anyhow::Result;
use chrono::Duration;
use serde::{Deserialize, Serialize};

use super::AppStyle;

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub pomodoro: PomodoroCfg,
    pub style: AppStyle,
    pub notification: Option<PathBuf>,
    pub muted: bool,
    pub portable: bool,
    pub always_on_top: bool,
    pub timer_notification: bool,
}

impl AppConfig {
    fn get_location(portable: bool) -> Result<PathBuf> {
        let location = if portable {
            std::env::current_dir()?
        } else {
            match dirs::config_dir() {
                Some(v) => {
                    let path = v.join(crate::defines::APP_NAME);
                    std::fs::create_dir_all(&path)?;
                    path
                }
                None => std::env::current_dir()?,
            }
        };

        Ok(location.join(APP_CONFIG_NAME))
    }

    fn load_config(location: &Path) -> Result<Self> {
        let mut config = String::new();
        let mut file = File::open(location)?;
        file.read_to_string(&mut config)?;

        let cfg = serde_yaml::from_str(&config)?;
        Ok(cfg)
    }

    pub fn save(&self) -> Result<()> {
        let config = serde_yaml::to_string(&self)?;
        let location = Self::get_location(self.portable)?;

        let mut file = File::create(location)?;
        file.write_all(config.as_bytes())?;

        Ok(())
    }

    pub fn load() -> Result<Self> {
        let config = {
            let portable = Self::get_location(true)?;
            if let Ok(mut cfg) = Self::load_config(&portable) {
                cfg.portable = true;
                cfg
            } else {
                let system = Self::get_location(false)?;
                Self::load_config(&system)?
            }
        };

        Ok(config)
    }

    pub fn load_sound(&self) -> Result<Option<Vec<u8>>> {
        let path = match &self.notification {
            Some(v) => v,
            None => return Ok(None),
        };

        let mut bytes = Vec::new();
        let mut file = File::open(path)?;
        file.read_to_end(&mut bytes)?;

        Ok(Some(bytes))
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        let cfg = PomodoroCfg {
            focus: Duration::seconds(10),
            long: Duration::seconds(5),
            short: Duration::seconds(3),
            rounds: 3,
        };

        Self {
            pomodoro: cfg,
            muted: false,
            portable: false,
            notification: Default::default(),
            style: Default::default(),
            always_on_top: false,
            timer_notification: false,
        }
    }
}

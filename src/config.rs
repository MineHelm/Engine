use std::{fs, path::PathBuf, sync::{RwLockReadGuard, RwLockWriteGuard}};

use crate::engine::ContainerEngineKind;

pub(crate) struct MHConfig(std::sync::RwLock<MineHelmConfig>);

impl MHConfig {
    pub fn new(config: MineHelmConfig) -> Self {
        Self(std::sync::RwLock::new(config))
    }

    #[track_caller]
    pub fn read(&self) -> RwLockReadGuard<MineHelmConfig> {
        self.0.read().expect("Failed to lock read guard on configuration.")
    }
    #[track_caller]
    pub fn write(&self) -> RwLockWriteGuard<MineHelmConfig> {
        self.0.write().expect("Failed to lock write guard on configuration.")
    }

    pub fn update<F>(&self, update_fn: F) -> bool
    where F: FnOnce(&mut MineHelmConfig) -> () {
        let mut cfg = self.write();
        update_fn(&mut *cfg);
        cfg.try_save()
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MineHelmConfig {
    pub(crate) is_onboarded: bool,
    pub(crate) engine: ContainerEngineKind,

    #[serde(skip, default="default_config_path_")]
    config_path: PathBuf
}

impl Default for MineHelmConfig {
    fn default() -> Self {
        Self {
            is_onboarded: false,
            engine: ContainerEngineKind::Docker,

            config_path: default_config_path_()
        }
    }
}

fn default_config_path_() -> PathBuf {
    std::env::var("CONFIG_PATH")
        .unwrap_or("~/.config/MineHelm/config.json".to_string())
        .into()
}

impl MineHelmConfig {
    pub fn load_or_init() -> Self {
        let default_config = MineHelmConfig::default();
        let path = default_config.config_path.as_path();
        if path.exists() {
            if let Ok(file) = fs::File::open(path) {
                if let Ok(config) = serde_json::from_reader(file) {
                    return config;
                }
            }
        }

        default_config.try_save();
        default_config
    }

    pub fn try_save(&self) -> bool {
        let file = match fs::File::create(self.config_path.as_path()) { 
            Ok(file) => file,
            Err(err) => {
                log::warn!("Failed to open file {:?}: {err}", self.config_path);
                return false;
            }
        };
        match serde_json::to_writer_pretty(file, self) { 
            Ok(_) => {},
            Err(err) => {
                log::warn!("Failed to serialize config: {err}");
                return false;
            }
        };
        true
    }
}

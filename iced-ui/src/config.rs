use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::io;
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub server_address: String,
    pub debug_mode: bool,
    pub colorblind_mode: bool,
    pub ui_level: u8,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            server_address: "http://localhost:8080".to_string(),
            debug_mode: false,
            colorblind_mode: false,
            ui_level: 1,
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_path = get_config_path()?;
        
        if !config_path.exists() {
            let default_config = Config::default();
            default_config.save()?;
            return Ok(default_config);
        }
        
        let config_str = fs::read_to_string(&config_path)?;
        let config: Config = serde_json::from_str(&config_str)?;
        
        Ok(config)
    }
    
    pub fn save(&self) -> Result<()> {
        let config_path = get_config_path()?;
        
        // Ensure directory exists
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        let config_str = serde_json::to_string_pretty(self)?;
        fs::write(&config_path, config_str)?;
        
        Ok(())
    }
}

fn get_config_path() -> Result<std::path::PathBuf> {
    let home_dir = dirs::home_dir().ok_or_else(|| {
        io::Error::new(io::ErrorKind::NotFound, "Could not find home directory")
    })?;
    
    let config_dir = home_dir.join(".config").join("rayhunter");
    fs::create_dir_all(&config_dir)?;
    
    Ok(config_dir.join("ui-config.json"))
}
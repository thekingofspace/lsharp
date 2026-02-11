use serde::Deserialize;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::fmt;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    Nocheck,
    Strict,
    Nonstrict,
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Mode::Nocheck => "nocheck",
            Mode::Strict => "strict",
            Mode::Nonstrict => "nonstrict",
        };
        write!(f, "{}", value)
    }
}

#[derive(Debug, Deserialize)]
pub struct LshConfig {
    pub input: String,
    pub output: String,
    pub include_types: bool,
    pub mode: Mode,
}

impl LshConfig {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let current_dir: PathBuf = env::current_dir()?;
        let config_path = current_dir.join(".lshrc");

        if !config_path.exists() {
            return Err(".lshrc not found in current directory".into());
        }

        let contents = fs::read_to_string(config_path)?;
        let config: LshConfig = toml::from_str(&contents)?;

        Ok(config)
    }

    pub fn load_or_default() -> Self {
        Self::load().unwrap_or_else(|_| Self {
            input: "src".to_string(),
            output: "dist".to_string(),
            include_types: false,
            mode: Mode::Strict,
        })
    }
}

use std::env;

use serde::{Deserialize, Serialize};

const CONFIG_PATH: &str = "calculator_config.toml";
const MINIMUM_WAGE: f64 = 10.42;
const BASIC_RATE: f64 = 20.0;
const VAT: f64 = 20.0;

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub(crate) struct Config {
    pub(crate) markup_percentage: f64,
    pub(crate) hourly_rate: f64,
    pub(crate) tax_rate: f64,
    pub(crate) vat: f64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            markup_percentage: 0.0,
            hourly_rate: MINIMUM_WAGE,
            tax_rate: BASIC_RATE,
            vat: VAT,
        }
    }
}

fn get_config_path() -> String {
    let config_dir = match env::consts::OS {
        "windows" => env::var("LOCALAPPDATA").unwrap(),
        "linux" => env::var("XDG_CONFIG_HOME").unwrap_or(env::var("HOME").unwrap() + "/.config"),
        "macos" => env::var("HOME").unwrap() + "/Library/Preferences",
        _ => String::new(),
    };
    format!("{}/{}", config_dir, CONFIG_PATH)
}

pub(crate) fn get_config() -> Config {
    let stored_config = confy::load_path(&get_config_path());
    if stored_config.is_err() {
        let config = Config::default();
        store_config(&config);
        return config;
    }
    stored_config.unwrap()
}

pub(crate) fn store_config(config: &Config) {
    confy::store_path(&get_config_path(), config).expect("Unable to store updated config");
}

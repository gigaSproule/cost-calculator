use serde::{Deserialize, Serialize};

use crate::store::get_store_path;

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
    format!("{}/{}", get_store_path(), CONFIG_PATH)
}

pub(crate) fn get_config() -> Config {
    let stored_config = confy::load_path(&get_config_path());
    if stored_config.is_ok() {
        return stored_config.unwrap();
    }
    let config = Config::default();
    store_config(&config);
    config
}

pub(crate) fn store_config(config: &Config) {
    confy::store_path(&get_config_path(), config).expect("Unable to store updated config");
}

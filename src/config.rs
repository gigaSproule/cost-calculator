use serde::{Deserialize, Serialize};

const CONFIG_PATH: &str = "calculator_config.toml";
const MINIMUM_WAGE: f32 = 10.42;
const BASIC_RATE: f32 = 20.0;
const VAT: f32 = 20.0;

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub(crate) struct Config {
    pub(crate) markup_percentage: f32,
    pub(crate) hourly_rate: f32,
    pub(crate) tax_rate: f32,
    pub(crate) vat: f32,
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

pub(crate) fn get_config() -> Config {
    let config: Config = confy::load_path(CONFIG_PATH).expect("Unable to read config");
    config
}

pub(crate) fn store_config(config: &Config) {
    confy::store_path(CONFIG_PATH, config).expect("Unable to store updated config");
}

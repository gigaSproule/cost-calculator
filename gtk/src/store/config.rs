use serde::{Deserialize, Serialize};

use calculators::Config;

use crate::store::get_store_path;

const CONFIG_PATH: &str = "calculator_config.toml";
const MINIMUM_WAGE: f64 = 10.42;
const BASIC_RATE: f64 = 20.0;
const VAT: f64 = 20.0;
const CURRENCY: &str = "£";

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub(crate) struct ConfigImpl {
    pub(crate) markup_percentage: f64,
    pub(crate) hourly_rate: f64,
    pub(crate) tax_rate: f64,
    pub(crate) vat: f64,
    pub(crate) currency: String,
}

impl Default for ConfigImpl {
    fn default() -> Self {
        Self {
            markup_percentage: 0.0,
            hourly_rate: MINIMUM_WAGE,
            tax_rate: BASIC_RATE,
            vat: VAT,
            currency: String::from(CURRENCY),
        }
    }
}

impl Config for ConfigImpl {
    fn get_markup_percentage(&self) -> f64 {
        self.markup_percentage
    }
    fn get_hourly_rate(&self) -> f64 {
        self.hourly_rate
    }
    fn get_tax_rate(&self) -> f64 {
        self.tax_rate
    }
    fn get_vat(&self) -> f64 {
        self.vat
    }
}

fn get_config_path() -> String {
    format!("{}/{}", get_store_path(), CONFIG_PATH)
}

pub(crate) fn get_config() -> ConfigImpl {
    let stored_config = confy::load_path(&get_config_path());
    if stored_config.is_ok() {
        return stored_config.unwrap();
    }
    let config = ConfigImpl::default();
    store_config(&config);
    config
}

pub(crate) fn store_config(config: &ConfigImpl) {
    confy::store_path(&get_config_path(), config).expect("Unable to store updated config");
}

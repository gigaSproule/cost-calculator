use serde::{Deserialize, Serialize};

const CONFIG_PATH: &str = "calculator.conf";

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Config {
    pub(crate) markup_percentage: f32,
}

impl std::default::Default for Config {
    fn default() -> Self {
        Self {
            markup_percentage: 0.0,
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

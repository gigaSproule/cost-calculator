use std::env;

pub(crate) mod config;
pub(crate) mod materials;

fn get_store_path() -> String {
    let config_dir = match env::consts::OS {
        "windows" => env::var("LOCALAPPDATA").unwrap(),
        "linux" => env::var("XDG_CONFIG_HOME").unwrap_or(env::var("HOME").unwrap() + "/.config"),
        "macos" => env::var("HOME").unwrap() + "/Library/Preferences",
        _ => String::new(),
    };
    format!("{}/{}", config_dir, "cost-calculator")
}

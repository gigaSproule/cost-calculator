#![windows_subsystem = "windows"]
use std::env;

use cli::load_cli;
use ui::load_ui;

mod calculator;
mod cli;
mod store;
mod ui;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.contains(&String::from("--cli")) {
        load_cli();
    } else {
        load_ui();
    }
}

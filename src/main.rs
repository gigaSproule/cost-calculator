#![windows_subsystem = "windows"]

use ui::load_ui;

mod calculator;
mod store;
mod ui;

fn main() {
    load_ui();
}

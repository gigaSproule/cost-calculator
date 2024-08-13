#![windows_subsystem = "windows"]

use ui::load_ui;

mod store;
mod ui;

fn main() {
    load_ui();
}

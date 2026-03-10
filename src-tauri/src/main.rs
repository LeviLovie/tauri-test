#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    if let Err(e) = test_lib::run() {
        panic!("Application error: {e}");
    }
}

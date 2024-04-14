#![cfg_attr(all(windows, not(test)), windows_subsystem = "windows")]

use game_2048_logic::game_main_entry;

fn main() {
    game_main_entry().expect("Platform Error");
}

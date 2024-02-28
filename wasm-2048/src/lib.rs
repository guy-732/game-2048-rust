use game_2048_logic::game_main_entry;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen(start))]
pub fn main() {
    game_main_entry().expect("Platform Error");
}

use game_2048_logic::game_main_entry;

fn main() {
let RESP =      game_main_entry();
    RESP.expect("Platform Error");
}

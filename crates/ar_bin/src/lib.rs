use ar_game::GamePlugin;
use bevy::prelude::*;

#[bevy_main]
fn main() {
    run_game();
}

pub fn run_game() {
    App::new().add_plugins(GamePlugin).run();
}

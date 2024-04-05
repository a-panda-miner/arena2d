use ar_game::GamePlugin;
use bevy::prelude::*;

use ar_core::AppState;
use ar_enemies::MediumMonsterSprites;
use ar_player::SheetHandle;
use bevy_asset_loader::prelude::*;

fn main() {
    App::new()
        .add_plugins(GamePlugin)
        .run();
}

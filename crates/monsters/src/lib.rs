use bevy::prelude::*;
use bevy::math::vec2;
use bevy_rand::prelude::{WyRand, EntropyPlugin};
use bevy_rand::resource::GlobalEntropy;
use rand_core::RngCore;

use ar_core::{PlayerMarker, };
use ar_template::{MonsterTemplates, MonsterFlatList};
use ar_enemies::{MonsterSprites, MonsterLayoutType};
use ar_camera::{ARENA_HEIGHT_ZOOMOUT, ARENA_WIDTH_ZOOMOUT};

struct MonsterPlugin;

impl Plugin for MonsterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_monsters);
    }
}

fn spawn_monsters(
    mut commands: Commands,
    monster_sprites: Res<MonsterSprites>,
    monster_template: Res<MonsterTemplates>,
    monster_flat: Res<MonsterFlatList>,
    player_position: Query<&GlobalTransform, With<PlayerMarker>>,
    mut rng: ResMut<GlobalEntropy<WyRand>>,
) {
    let mut random = [0u8; 4];
    rng.fill_bytes(&mut random);
    let random = random;
    let left_right: bool = random[0] % 2 == 0;
    let up_down: bool = random[1] % 2 == 0;
    let spawn_point: Vec3 = if left_right {
        if up_down {
            Vec3::new(ARENA_WIDTH_ZOOMOUT, ARENA_HEIGHT_ZOOMOUT, 0.0)
        } else {
            Vec3::new(ARENA_WIDTH_ZOOMOUT, -ARENA_HEIGHT_ZOOMOUT, 0.0)
        }
    } else {
        if up_down {
            Vec3::new(-ARENA_WIDTH_ZOOMOUT , ARENA_HEIGHT_ZOOMOUT, 0.0)
        } else {
            Vec3::new(-ARENA_WIDTH_ZOOMOUT , -ARENA_HEIGHT_ZOOMOUT, 0.0)
        }
    };
    let player_position = player_position.single();
    let spawn_point = spawn_point + player_position.translation();

    let flat_len = monster_flat.name_difficulty.len();
    let random_index = (rng.next_u64() as usize) % flat_len;

    let name = monster_flat.name_difficulty[random_index].0.clone();
    let monster = monster_template.templates.get(&name).unwrap();

    let layout = match monster.layout {
        MonsterLayoutType::Small => monster_sprites.monster_layout_small.clone(),
        MonsterLayoutType::Medium => monster_sprites.monster_layout_large_four.clone(),
        MonsterLayoutType::Large => monster_sprites.monster_layout_large_nine.clone(),
        MonsterLayoutType::Boss => monster_sprites.monster_layout_large_nine.clone(),
    };

    commands.spawn_empty()
        .insert(SpriteSheetBundle {
            texture: monster_sprites.monster_sheets.get(name.as_str()).unwrap().clone(),
            transform: Transform::from_translation(spawn_point),
            ..Default::default()
        })
        .insert(TextureAtlas::from(layout));
}
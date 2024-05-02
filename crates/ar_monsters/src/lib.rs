use bevy::prelude::*;
use bevy::math::vec2;
use bevy_rand::prelude::{WyRand, EntropyPlugin};
use bevy_rand::resource::GlobalEntropy;
use rand_core::RngCore;
use bevy_xpbd_2d::prelude::*;

use ar_core::{PlayerMarker, Layer, Cooldown, MonsterMarker, MonsterSet, BaseSpeed};
use ar_template::{MonsterTemplates, MonsterFlatList};
use ar_enemies::{MonsterSprites, MonsterLayoutType};
use ar_camera::{ARENA_HEIGHT_ZOOMOUT, ARENA_WIDTH_ZOOMOUT};

pub struct MonsterPlugin;

impl Plugin for MonsterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_monsters.in_set(MonsterSet));
    }
}

// TODO! Add a random offset to the spawn point,
// add a timer,
// add a battle score system that changes the types of monsters that can be spawned and 
// the frequency of spawning
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

    let direction = (player_position.translation() - spawn_point).normalize_or_zero();

    let flat_len = monster_flat.name_difficulty.len();
    let random_index = (rng.next_u64() as usize) % flat_len;

    let name = monster_flat.name_difficulty[random_index].0.clone();
    let monster = monster_template.templates.get(&name).unwrap();

    let base_speed = match monster.movespeed {
        Some(speed) => speed,
        None => 0.0,
    };
    let speed: Vec2 = vec2(base_speed * direction.x * 10.0, base_speed * direction.y * 10.0);

    let (layout, collider_size, mass) = match monster.layout {
        MonsterLayoutType::Small => (monster_sprites.monster_layout_small.clone(), 8.0, 20.0),
        MonsterLayoutType::Medium => (monster_sprites.monster_layout_large_four.clone(), 12.0, 35.0),
        MonsterLayoutType::Large => (monster_sprites.monster_layout_large_nine.clone(), 16.0, 50.0),
        MonsterLayoutType::Boss => (monster_sprites.monster_layout_large_nine.clone(), 32.0, 100.0),
    };

    let sprite_name = monster.sprite_name.clone();
    info!("Spawning {} @ {:?}", sprite_name, spawn_point);
    commands.spawn_empty()
        .insert(MonsterMarker)
        .insert(SpriteSheetBundle {
            texture: monster_sprites.monster_sheets.get(sprite_name.as_str()).unwrap().clone(),
            transform: Transform::from_translation(spawn_point),
            atlas: TextureAtlas::from(layout),
            ..Default::default()
        })
        .insert(BaseSpeed(base_speed))
        .insert(RigidBody::Dynamic)
        .insert(Mass(mass))
        .insert(LinearVelocity(speed))
        .insert(AngularVelocity(0.0))
        .insert(Collider::circle(collider_size))
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(CollisionLayers::new([Layer::Monster], [Layer::Player, Layer::PlayerProjectile]))
        .insert(Cooldown(Timer::from_seconds(0.55, TimerMode::Repeating))); // Animation timer 
}
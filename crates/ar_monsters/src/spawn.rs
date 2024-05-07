use crate::*;
use bevy::prelude::*;

pub struct SpawnPlugin;

impl Plugin for SpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, spawn_monsters.in_set(MonsterSet));
    }
}

/// Handles the logic of monster spawning
fn spawn_monsters(
    time: Res<Time>,
    mut timer: ResMut<SpawnerTimer>,
    mut commands: Commands,
    monster_sprites: Res<MonsterSprites>,
    monster_template: Res<MonsterTemplates>,
    monster_flat: Res<MonsterFlatList>,
    player_position: Query<&GlobalTransform, With<PlayerMarker>>,
    mut rng: ResMut<GlobalEntropy<WyRand>>,
    target: Res<PlayerHandler>,
    game_score: Res<GameScore>,
    minutes_survived: Res<MinutesSurvived>,
    monsters_alive: Res<MonstersAlive>,
) {
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }
    let mut spawn_count = 1 + (minutes_survived.0 / 4) + (game_score.0 / 50);
    if monsters_alive.0 >= spawn_count {
        spawn_count = 1;
    } else {
        spawn_count -= monsters_alive.0;
    }

    while spawn_count > 0 {
        let mut random = [0u8; 8];
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
                Vec3::new(-ARENA_WIDTH_ZOOMOUT, ARENA_HEIGHT_ZOOMOUT, 0.0)
            } else {
                Vec3::new(-ARENA_WIDTH_ZOOMOUT, -ARENA_HEIGHT_ZOOMOUT, 0.0)
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
            Some(speed) => speed * 10.0,
            None => 0.0,
        };
        let speed: Vec2 = vec2(base_speed * direction.x, base_speed * direction.y);

        let (layout, collider_size, mass) = match monster.layout {
            MonsterLayoutType::Small => (monster_sprites.monster_layout_small.clone(), 8.0, 20.0),
            MonsterLayoutType::Medium => (
                monster_sprites.monster_layout_large_four.clone(),
                12.0,
                35.0,
            ),
            MonsterLayoutType::Large => (
                monster_sprites.monster_layout_large_nine.clone(),
                16.0,
                50.0,
            ),
            MonsterLayoutType::Boss => (
                monster_sprites.monster_layout_large_nine.clone(),
                32.0,
                100.0,
            ),
        };

        let sprite_name = monster.sprite_name.clone();
        info!("Spawning {} @ {:?}", sprite_name, spawn_point);
        commands
            .spawn_empty()
            .insert(MonsterMarker)
            .insert(SpriteSheetBundle {
                texture: monster_sprites
                    .monster_sheets
                    .get(sprite_name.as_str())
                    .unwrap()
                    .clone(),
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
            .insert(CollisionLayers::new(
                [Layer::Monster],
                [Layer::Player, Layer::PlayerProjectile],
            ))
            .insert(Cooldown(Timer::from_seconds(0.55, TimerMode::Repeating))) // Animation timer
            .insert(Chase {
                target: target.player_id,
            });
        spawn_count -= 1;
    }
}

use crate::*;
use bevy::prelude::*;

pub struct SpawnPlugin;

impl Plugin for SpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, spawn_monsters.in_set(MonsterSet));
    }
}

/// Handles the logic of monster spawning
//TODO! Separate into multiple systems instead of a big one
fn spawn_monsters(
    time: Res<Time>,
    mut timer: ResMut<SpawnerTimer>,
    mut commands: Commands,
    monster_sprites: Res<MonsterSprites>,
    monster_template: Res<MonsterTemplates>,
    monster_difficulty_lists: Res<MonsterDifficultyLists>,
    player_position: Query<(&Transform, &GlobalTransform), With<PlayerMarker>>,
    mut rng: ResMut<GlobalEntropy<WyRand>>,
    target: Res<PlayerHandler>,
    game_score: Res<GameScore>,
    minutes_survived: Res<MinutesSurvived>,
    monsters_alive: Res<MonstersAlive>,
) {
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }
    // How many monsters should be on the map
    let mut spawn_count = 1 + (minutes_survived.0 / 4) + (game_score.0 / 50);
    if monsters_alive.0 >= spawn_count {
        spawn_count = 1; // It spawns beyond the limit
    } else {
        spawn_count -= monsters_alive.0;
    }

    while spawn_count > 0 {
        let mut random = [0u8; 8];
        rng.fill_bytes(&mut random);
        let random = random;
        let left_right: bool = random[0] % 2 == 0;
        let up_down: bool = random[1] % 2 == 0;
        // TODO! Base it on game_score and minutes_survived instead of purely RNG
        let difficulty = random[2] % 4;
        let spawn_point: Vec3 = if left_right {
            if up_down {
                Vec3::new(ARENA_WIDTH_ZOOMOUT, ARENA_HEIGHT_ZOOMOUT, 0.0)
            } else {
                Vec3::new(ARENA_WIDTH_ZOOMOUT, -ARENA_HEIGHT_ZOOMOUT, 0.0)
            }
        } else if up_down {
            Vec3::new(-ARENA_WIDTH_ZOOMOUT, ARENA_HEIGHT_ZOOMOUT, 0.0)
        } else {
            Vec3::new(-ARENA_WIDTH_ZOOMOUT, -ARENA_HEIGHT_ZOOMOUT, 0.0)
        };
        let (player_position, _) = player_position.single();
        let spawn_point = spawn_point + player_position.translation;

        let direction = (player_position.translation - spawn_point).normalize_or_zero();

        let list = match difficulty {
            0 => &monster_difficulty_lists.difficulty_1,
            1 => &monster_difficulty_lists.difficulty_2,
            2 => &monster_difficulty_lists.difficulty_3,
            3 => &monster_difficulty_lists.difficulty_4,
            _ => panic!("Invalid difficulty"),
        };
        let random_index = (rng.next_u64() as usize) % list.len();

        let name = list[random_index].clone();
        let monster = monster_template.templates.get(&name).unwrap();

        let base_speed = match monster.movespeed {
            Some(speed) => speed * 10.0,
            None => 0.0,
        };
        let speed: Vec2 = vec2(base_speed * direction.x, base_speed * direction.y);

        let loot_tables: LootTables = monster.loot_tables.clone().into();

        let drop_chance: DropsChance = monster.drops_chance.unwrap_or(1.0).into();

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
        let monster_id = commands
            .spawn_empty()
            .insert(MonsterMarker)
            .insert(Sprite {
                image: monster_sprites
                    .monster_sheets
                    .get(sprite_name.as_str())
                    .unwrap()
                    .clone(),
                texture_atlas: Some(layout.clone().into()),
                ..Default::default()
            })
            .insert(Transform::from_translation(spawn_point))
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
            .insert(Health(monster.hp))
            .insert(Damage(monster.damage))
            .insert(loot_tables)
            .insert(drop_chance)
            .insert(Cooldown(Timer::from_seconds(0.55, TimerMode::Repeating))) // Animation timer
            .insert(Chase {
                target: target.player_id,
            })
            .id();
        if let MonsterLayoutType::Small = monster.layout {
            commands.entity(monster_id).insert(MonsterMarkerSmall);
        }
        spawn_count -= 1;
    }
}

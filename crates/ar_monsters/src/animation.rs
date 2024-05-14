use crate::*;

pub struct MonsterAnimationPlugin;

impl Plugin for MonsterAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            animation_small_monster_loop
                .run_if(on_timer(Duration::from_millis(740)))
                .in_set(MonsterSet),
        )
        .add_systems(
            Update,
            animation_small_monster_direction
                .run_if(on_timer(Duration::from_millis(2180)))
                .in_set(MonsterSet),
        );
    }
}

/// Defines the column in the sprite sheet that the animation should be, depending on way
/// the monster is moving towards the player
fn animation_small_monster_direction(
    mut query: Query<(&mut TextureAtlas, &GlobalTransform), With<MonsterMarkerSmall>>,
    player_position: Query<&GlobalTransform, With<PlayerMarker>>,
) {
    let player_position = player_position.get_single().unwrap();
    for (mut texture_atlas, transform) in query.iter_mut() {
        let relative_position = transform.translation() - player_position.translation();
        let (x, y, _) = relative_position.into();
        if x > 0.0 && y > 0.0 {
            texture_atlas.index = 1;
        }
        if x < 0.0 && y > 0.0 {
            texture_atlas.index = 3;
        }
        if x < 0.0 && y < 0.0 {
            texture_atlas.index = 5;
        }
        if x > 0.0 && y < 0.0 {
            texture_atlas.index = 7;
        }
    }
}

/// Loops between the 2-key animation frame sequence, depending on which column
/// the animation is in
fn animation_small_monster_loop(mut query: Query<&mut TextureAtlas, With<MonsterMarkerSmall>>) {
    for mut texture_atlas in query.iter_mut() {
        match texture_atlas.index {
            0 => texture_atlas.index = 1,
            1 => texture_atlas.index = 0,
            2 => texture_atlas.index = 3,
            3 => texture_atlas.index = 2,
            4 => texture_atlas.index = 5,
            5 => texture_atlas.index = 4,
            6 => texture_atlas.index = 7,
            7 => texture_atlas.index = 6,
            _ => {}
        }
    }
}

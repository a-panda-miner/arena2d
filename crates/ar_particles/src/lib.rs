use ar_core::{AppState, ParticleSet, PlayerMarker};
use bevy::prelude::*;
use bevy_hanabi::prelude::*;

pub struct ParticlesPlugin;

impl Plugin for ParticlesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(HanabiPlugin).add_systems(
            OnEnter(AppState::InBattle),
            add_particles_for_player.in_set(ParticleSet),
        );
    }
}

/// Adds a particle spawner to the player
/// Must be run after the player is spawned
fn add_particles_for_player(
    mut commands: Commands,
    mut effects: ResMut<Assets<EffectAsset>>,
    mut player: Query<Entity, With<PlayerMarker>>,
) {
    let gradient = Gradient::constant(Vec4::new(0.5, 0.5, 1.0, 0.3));

    let writer = ExprWriter::new();
    let age = writer.lit(0.).expr();
    let init_age = SetAttributeModifier::new(Attribute::AGE, age);

    let lifetime = writer.lit(2.0).expr();
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

    let init_pos = SetPositionCircleModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        axis: writer.lit(Vec3::Y).expr(),
        radius: writer.lit(10.5).expr(),
        dimension: ShapeDimension::Volume,
    };

    let init_vel = SetVelocityCircleModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        axis: writer.lit(Vec3::Y).expr(),
        speed: (writer.lit(1.) + writer.lit(0.5) * writer.rand(ScalarType::Float)).expr(),
    };
    let accel = writer.lit(Vec3::Y * -10.0).expr();
    let update_accel = AccelModifier::new(accel);

    let drag = writer.lit(4.0).expr();
    let update_drag = LinearDragModifier::new(drag);

    let mut module = writer.finish();

    let round = RoundModifier::constant(&mut module, 2.0 / 3.0);

    let spawner = Spawner::rate(5.0.into());
    let effect = effects.add(
        EffectAsset::new(4096, spawner, module)
            .with_name("player_particles")
            .init(init_pos)
            .init(init_vel)
            .init(init_age)
            .init(init_lifetime)
            .update(update_accel)
            .update(update_drag)
            .render(SizeOverLifetimeModifier {
                gradient: Gradient::constant(Vec3::splat(1.0)),
                screen_space_size: false,
            })
            .render(ColorOverLifetimeModifier { gradient })
            .render(round),
    );

    let player = player.single_mut();
    commands
        .spawn(ParticleEffectBundle {
            effect: ParticleEffect::new(effect).with_z_layer_2d(Some(10.0)),
            ..default()
        })
        .set_parent(player);
}

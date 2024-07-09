use ar_battle::PlayerDamageEvent;
use ar_core::{AppState, ParticleSet, PlayerMarker};
use bevy::prelude::*;
use bevy_hanabi::prelude::*;

pub struct ParticlesPlugin;

impl Plugin for ParticlesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(HanabiPlugin)
            .add_systems(FixedUpdate, player_damaged_particles.in_set(ParticleSet))
            .add_systems(
                OnEnter(AppState::InBattle),
                setup_particle_effects.in_set(ParticleSet),
            );
    }
}

#[derive(Component)]
pub struct DamagedParticleMarker;

fn setup_particle_effects(mut commands: Commands, mut effects: ResMut<Assets<EffectAsset>>) {
    let writer = ExprWriter::new();
    let age = writer.lit(0.).expr();
    let init_age = SetAttributeModifier::new(Attribute::AGE, age);
    let lifetime = writer.lit(0.5).expr();
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

    let drag = writer.lit(2.).expr();
    let update_drag = LinearDragModifier::new(drag);

    let spawn_color = writer.add_property("spawn_color", 0xFFFFFFFFu32.into());
    let color = writer.prop(spawn_color).expr();
    let init_color = SetAttributeModifier::new(Attribute::COLOR, color);

    let normal = writer.add_property("normal", Vec3::ZERO.into());
    let normal = writer.prop(normal);

    let pos = normal.clone();
    let init_pos = SetAttributeModifier::new(Attribute::POSITION, pos.expr());

    let tangent = writer.lit(Vec3::Z).cross(normal.clone());
    let spread = writer.rand(ScalarType::Float) * writer.lit(2.) - writer.lit(1.);
    let speed = writer.rand(ScalarType::Float) * writer.lit(0.2);
    let velocity = (normal + tangent * spread * writer.lit(5.0)).normalized() * speed;
    let init_vel = SetAttributeModifier::new(Attribute::VELOCITY, velocity.expr());

    let spawner = Spawner::once(100.0.into(), false);

    let effect = effects.add(
        EffectAsset::new(vec![32768], spawner, writer.finish())
            .with_name("spawn_on_command")
            .init(init_pos)
            .init(init_vel)
            .init(init_age)
            .init(init_lifetime)
            .init(init_color)
            .update(update_drag)
            // Set a size of 3 (logical) pixels, constant in screen space, independent of projection
            .render(SetSizeModifier {
                size: Vec2::splat(3.).into(),
            })
            .render(ScreenSpaceSizeModifier),
    );

    commands
        .spawn(ParticleEffectBundle::new(effect))
        .insert(Name::new("effect"))
        .insert(DamagedParticleMarker);
}

fn player_damaged_particles(
    mut ev_player_damaged: EventReader<PlayerDamageEvent>,
    player_pos: Query<&Transform, (With<PlayerMarker>, Without<DamagedParticleMarker>)>,
    mut effect: Query<
        (&mut EffectProperties, &mut EffectSpawner, &mut Transform),
        With<DamagedParticleMarker>,
    >,
) {
    if ev_player_damaged.is_empty() {
        return;
    }
    for _ in ev_player_damaged.read() {
        let Ok((mut properties, mut spawner, mut effect_transform)) = effect.get_single_mut()
        else {
            continue;
        };

        let pos = player_pos.single().translation;

        effect_transform.translation = pos;

        let color = 0xFFFF0000u32;

        properties.set("spawn_color", color.into());

        // Spawns the particle
        spawner.reset();
    }
}

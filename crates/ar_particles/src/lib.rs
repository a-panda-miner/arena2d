use bevy::prelude::*;
// Remove comments when bevy_hanabi is updated to support bevy 0.14
// use bevy_hanabi::prelude::*;
// use ar_battle::PlayerDamageEvent;
pub struct ParticlesPlugin;

impl Plugin for ParticlesPlugin {
    fn build(&self, _app: &mut App) {
        //        app.add_plugins(HanabiPlugin)
        //            .add_systems(FixedUpdate, player_damaged_particles);
    }
}

//
//fn setup_particle_effects(mut effects: ResMut<Assets<EffectAsset>>) {
//
//}

//fn player_damaged_particles(
//    mut commands: Commands,
//    mut ev_player_damaged: EventReader<PlayerDamageEvent>,
//) {
//    if ev_player_damaged.is_empty() {
//        return;
//    }
//
//}

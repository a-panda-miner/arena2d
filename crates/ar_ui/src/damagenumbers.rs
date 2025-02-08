use crate::{DisplayDamageEvent, FontAssets, LifeTime, PlayerMarker, PlayerMinusHpEvent, UiSet};
use bevy::prelude::*;
pub struct DamageNumbersPlugin;

impl Plugin for DamageNumbersPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (display_player_damaged_numbers, display_damage_numbers).in_set(UiSet),
        );
    }
}

fn display_player_damaged_numbers(
    fonts: Res<FontAssets>,
    mut damage: EventReader<PlayerMinusHpEvent>,
    mut commands: Commands,
    player_pos: Query<&Transform, With<PlayerMarker>>,
) {
    if damage.is_empty() {
        return;
    }
    let font = fonts.damage_font.clone();
    let color: Color = Color::srgba_u8(15, 56, 15, 255);
    let transform = player_pos.single();
    for damage_number in damage.read() {
        let displayed_number = damage_number.damage.to_string();
        commands
            .spawn(Text2d::new(displayed_number))
            .insert(TextFont {
                font: font.clone(),
                font_size: 14.0,
                ..default()
            })
            .insert(TextColor(color))
            .insert(*transform)
            .insert(LifeTime {
                timer: Timer::from_seconds(1.2, TimerMode::Once),
            });
    }
    damage.clear();
}

fn display_damage_numbers(
    mut commands: Commands,
    fonts: Res<FontAssets>,
    mut damage: EventReader<DisplayDamageEvent>,
    monster_pos: Query<&Transform, Without<PlayerMarker>>,
) {
    if damage.is_empty() {
        return;
    }
    let font = fonts.damage_font.clone();
    let color: Color = Color::srgba_u8(15, 56, 15, 255);

    for ev in damage.read() {
        if let Ok(transform) = monster_pos.get(ev.target) {
            let displayed_number = ev.damage.to_string();
            commands
                .spawn(Text2d::new(displayed_number))
                .insert(TextFont {
                    font: font.clone(),
                    font_size: 18.0,
                    ..default()
                })
                .insert(TextColor(color))
                .insert(*transform)
                .insert(LifeTime {
                    timer: Timer::from_seconds(0.8, TimerMode::Once),
                });
        }
    }
}

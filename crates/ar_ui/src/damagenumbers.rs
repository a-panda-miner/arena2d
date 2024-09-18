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
    spatial_query: Query<(&Transform, &GlobalTransform), With<PlayerMarker>>,
) {
    if damage.is_empty() {
        return;
    }
    let font = fonts.damage_font.clone();
    let color: Color = Color::srgba_u8(15, 56, 15, 255);
    let textstyle: TextStyle = TextStyle {
        font,
        font_size: 14.0,
        color,
    };
    let (transform, global_transform) = spatial_query.single();
    for damage_number in damage.read() {
        let displayed_number = damage_number.damage.to_string();
        commands
            .spawn(Text2dBundle {
                text: Text::from_section(displayed_number, textstyle.clone()),
                transform: *transform,
                global_transform: *global_transform,
                ..default()
            })
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
    spatial_query: Query<(&Transform, &GlobalTransform), Without<PlayerMarker>>,
) {
    if damage.is_empty() {
        return;
    }
    let font = fonts.damage_font.clone();
    let color: Color = Color::srgba_u8(15, 56, 15, 255);
    let textstyle: TextStyle = TextStyle {
        font,
        font_size: 18.0,
        color,
    };
    for ev in damage.read() {
        if let Ok((transform, global_transform)) = spatial_query.get(ev.target) {
            let displayed_number = ev.damage.to_string();
            commands
                .spawn(Text2dBundle {
                    text: Text::from_section(displayed_number, textstyle.clone()),
                    transform: *transform,
                    global_transform: *global_transform,
                    ..default()
                })
                .insert(LifeTime {
                    timer: Timer::from_seconds(0.8, TimerMode::Once),
                });
        }
    }
}

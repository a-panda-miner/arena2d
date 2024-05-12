pub mod damagenumbers;

use crate::damagenumbers::DamageNumbersPlugin;
use ar_core::{AppState, Health, LifeTime, PlayerMarker, PlayerMinusHpEvent, UiMarker, UiSet};
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct FontAssets {
    #[asset(path = "fonts/PublicPixel-E447g.ttf")]
    pub ui_font: Handle<Font>,
    #[asset(path = "fonts/NotJamSignature17.ttf")]
    pub damage_font: Handle<Font>,
    #[asset(path = "fonts/ThickPixels.ttf")]
    pub menu_font: Handle<Font>,
}

#[derive(Component)]
struct PlayerHealthText;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DamageNumbersPlugin)
            .add_systems(
                OnEnter(AppState::InBattle),
                set_display_player_health.in_set(UiSet),
            )
            .add_systems(FixedUpdate, update_health_displayer.in_set(UiSet));
    }
}

fn set_display_player_health(
    fonts: Res<FontAssets>,
    health: Query<&Health, With<PlayerMarker>>,
    mut commands: Commands,
) {
    let health = health.single();
    let text = format!("HP: {}", health.0);
    let font = fonts.ui_font.clone();
    let textstyle: TextStyle = TextStyle {
        font,
        font_size: 16.0,
        color: Color::rgb(1.0, 1.0, 1.0),
    };

    commands
        .spawn(
            TextBundle::from_section(text, textstyle)
                .with_text_justify(JustifyText::Center)
                .with_style(Style {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(5.0),
                    right: Val::Px(5.0),
                    ..default()
                }),
        )
        .insert(UiMarker)
        .insert(PlayerHealthText);
}

fn update_health_displayer(
    mut text: Query<&mut Text, With<PlayerHealthText>>,
    health: Query<&Health, With<PlayerMarker>>,
) {
    let health = health.single();
    let mut text = text.single_mut();
    text.sections[0].value = format!("HP: {}", health.0);
}

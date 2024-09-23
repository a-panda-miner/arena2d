use ar_core::{ChooseACard, UiSet, AppState};
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy::utils::HashMap;

pub struct DisplayCardsPlugin;

#[derive(Debug, AssetCollection, Resource)]
pub struct CardsSprite {
    #[asset(
        paths(
            "templates/card_uncommon.png",
            "templates/card_common.png",
            "templates/card_rare.png",
            "cards/fireball.png",
            "cards/health.png",
            "cards/stamina.png",
        ),
        collection(mapped, typed)
    )]
    pub cards_sprites: HashMap<AssetFileStem, Handle<Image>>,
    #[asset(texture_atlas_layout(tile_size_x = 160, tile_size_y = 232, columns = 1, rows = 1))]
    pub templates_layout: Handle<TextureAtlasLayout>,
    #[asset(texture_atlas_layout(tile_size_x = 146, tile_size_y = 129, columns = 1, rows = 1))]
    pub cards_layout: Handle<TextureAtlasLayout>,
}

impl Plugin for DisplayCardsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InBattle), cards_ui_set_up.in_set(UiSet));
    }
}

/// Spawns UI entities for the cards,
/// When cards are spawned another system modifies these entities,
/// revealing them until and updating the art of the card, when the resource 
/// "ChooseACard" is empty it goes back to being hidden
fn cards_ui_set_up(
    mut commands: Commands,
    cards_sprite: Res<CardsSprite>,
) {

    let template_uncommon = cards_sprite.cards_sprites.get("card_uncommon").expect("card_uncommon not loaded");

    let container = NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceEvenly,
            align_items: AlignItems::Center,
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(10.0),
            bottom: Val::Percent(7.5),
            ..default()
        },
        
        ..default()
    };
    
    let card1 = NodeBundle {
        style: Style {
            width: Val::Px(160.),
            height: Val::Px(232.), 
            ..default()
        },
        background_color: Color::srgb(0.65, 0.65, 0.65).into(),
        ..default()
    };

    let card2 = NodeBundle {
        style: Style {
            width: Val::Px(160.),
            height: Val::Px(232.), 
            ..default()
        },
        background_color: Color::srgb(0.65, 0.65, 0.65).into(),
        ..default()
    };

    let card3 = NodeBundle {
        style: Style {
            width: Val::Px(160.),
            height: Val::Px(232.), 
            ..default()
        },
        background_color: Color::srgb(0.65, 0.65, 0.65).into(),
        ..default()
    };

    let template1 = ImageBundle {
        style: Style {
            position_type: PositionType::Relative,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        image: UiImage::new(template_uncommon.clone()),
        ..default()
    };

    let template2 = ImageBundle {
        style: Style {
            position_type: PositionType::Relative,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        image: UiImage::new(template_uncommon.clone()),
        ..default()
    };

    let template3 = ImageBundle {
        style: Style {
            position_type: PositionType::Relative,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        image: UiImage::new(template_uncommon.clone()),
        ..default()
    };


    let parent = commands.spawn(container).id();
    let card1 = commands.spawn(card1).id();
    let card2 = commands.spawn(card2).id();
    let card3 = commands.spawn(card3).id();
    let template1 = commands.spawn(template1).id();
    let template2 = commands.spawn(template2).id();
    let template3 = commands.spawn(template3).id();


    commands.entity(parent).push_children(&[card1, card2, card3]);

    commands.entity(card1).push_children(&[template1]);
    commands.entity(card2).push_children(&[template2]);
    commands.entity(card3).push_children(&[template3]);
}
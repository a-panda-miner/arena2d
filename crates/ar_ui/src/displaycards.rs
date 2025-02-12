use ar_core::{AppState, CardsTemplates, ChooseACard, UiSet};
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_asset_loader::prelude::*;

pub struct DisplayCardsPlugin;

#[derive(Component, Debug)]
pub struct CardUiMarker;

#[derive(Component, Debug)]
pub struct CardArtMarker;

#[derive(Component, Debug)]
pub struct CardTemplateMarker;

#[derive(Resource, Debug)]
pub struct CardUiHelper {
    pub card_ui_id: [Entity; 3],
    pub card_template_id: [Entity; 3],
    pub card_art_id: [Entity; 3],
}

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
        app.add_systems(OnEnter(AppState::InBattle), (cards_ui_set_up).in_set(UiSet))
            .add_systems(
                FixedUpdate,
                (display_hide_available_cards)
                    .in_set(UiSet)
                    .run_if(resource_changed::<ChooseACard>),
            );
    }
}

/// Spawns UI entities for the cards,
/// When cards are spawned another system modifies these entities,
/// revealing them and updating the art of the card, when the resource
/// `ChooseACard` is empty it goes back to being hidden
fn cards_ui_set_up(mut commands: Commands, cards_sprite: Res<CardsSprite>) {
    let template_uncommon = cards_sprite
        .cards_sprites
        .get("card_uncommon")
        .expect("card_uncommon not loaded");

    let fireball_art = cards_sprite
        .cards_sprites
        .get("fireball")
        .expect("fireball art not loaded");

    let stamina_art = cards_sprite
        .cards_sprites
        .get("stamina")
        .expect("stamina art not loaded");

    let health_art = cards_sprite
        .cards_sprites
        .get("health")
        .expect("health art not loaded");

    let background_color: Color = Color::srgba_u8(155, 188, 15, 255);

    let container = Node {
        flex_direction: FlexDirection::Row,
        justify_content: JustifyContent::SpaceEvenly,
        align_items: AlignItems::Center,
        position_type: PositionType::Absolute,
        width: Val::Percent(100.0),
        height: Val::Percent(10.0),
        bottom: Val::Percent(7.5),
        ..default()
    };

    let card_container = Node {
        width: Val::Px(160.),
        height: Val::Px(232.),
        ..default()
    };

    let template_node = Node {
        position_type: PositionType::Relative,
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        ..default()
    };
    let template_image = ImageNode::new(template_uncommon.clone());

    let card_art_fireball = ImageNode::new(fireball_art.clone());
    let card_art_stamina = ImageNode::new(stamina_art.clone());
    let card_art_health = ImageNode::new(health_art.clone());

    let card_style = Node {
        display: Display::Grid,
        width: Val::Percent(91.25),
        height: Val::Percent(55.6),
        margin: UiRect {
            left: Val::Percent(4.5),
            right: Val::Percent(4.5),
            top: Val::Percent(7.75),
            bottom: Val::Percent(0.0),
        },
        ..default()
    };

    let parent = commands
        .spawn(container)
        .insert(Name::new("cards_ui_container"))
        .id();

    let card1 = commands
        .spawn(card_container.clone())
        .insert(BackgroundColor(background_color))
        .insert(Visibility::Hidden)
        .insert(Name::new("card1"))
        .insert(CardUiMarker)
        .id();
    let card2 = commands
        .spawn(card_container.clone())
        .insert(BackgroundColor(background_color))
        .insert(Visibility::Hidden)
        .insert(Name::new("card2"))
        .insert(CardUiMarker)
        .id();
    let card3 = commands
        .spawn(card_container)
        .insert(BackgroundColor(background_color))
        .insert(Visibility::Hidden)
        .insert(Name::new("card3"))
        .insert(CardUiMarker)
        .id();

    let template1 = commands
        .spawn(template_node.clone())
        .insert(template_image.clone())
        .insert(CardTemplateMarker)
        .insert(Name::new("template1"))
        .id();
    let template2 = commands
        .spawn(template_node.clone())
        .insert(template_image.clone())
        .insert(CardTemplateMarker)
        .insert(Name::new("template2"))
        .id();
    let template3 = commands
        .spawn(template_node)
        .insert(template_image)
        .insert(Name::new("template3"))
        .insert(CardTemplateMarker)
        .id();

    let card_art1 = commands
        .spawn(card_art_fireball)
        .insert(card_style.clone())
        .insert(Name::new("card_art1"))
        .insert(CardArtMarker)
        .id();
    let card_art2 = commands
        .spawn(card_art_health)
        .insert(card_style.clone())
        .insert(Name::new("card_art2"))
        .insert(CardArtMarker)
        .id();
    let card_art3 = commands
        .spawn(card_art_stamina)
        .insert(card_style)
        .insert(Name::new("card_art3"))
        .insert(CardArtMarker)
        .id();

    commands.entity(parent).add_children(&[card1, card2, card3]);

    commands.entity(card1).add_children(&[template1]);
    commands.entity(card2).add_children(&[template2]);
    commands.entity(card3).add_children(&[template3]);

    commands.entity(template1).add_children(&[card_art1]);
    commands.entity(template2).add_children(&[card_art2]);
    commands.entity(template3).add_children(&[card_art3]);

    let card_ui_helper = CardUiHelper {
        card_ui_id: [card1, card2, card3],
        card_template_id: [template1, template2, template3],
        card_art_id: [card_art1, card_art2, card_art3],
    };

    commands.insert_resource(card_ui_helper);
}

/// Display/Hide available cards
/// Runs when the `AvailableCards` resource is updated
#[allow(clippy::type_complexity)]
fn display_hide_available_cards(
    mut card_ui_query: Query<&mut Visibility, With<CardUiMarker>>,
    mut card_template_query: Query<
        (&mut Visibility, &mut ImageNode),
        (
            With<CardTemplateMarker>,
            Without<CardUiMarker>,
            Without<CardArtMarker>,
        ),
    >,
    mut card_art_query: Query<
        (&mut Visibility, &mut ImageNode),
        (With<CardArtMarker>, Without<CardUiMarker>),
    >,
    cards_helper: Res<CardUiHelper>,
    cards_sprites: Res<CardsSprite>,
    chosen_cards: Res<ChooseACard>,
    cards_res: Res<CardsTemplates>,
) {
    let mut available_cards = 0;
    if !chosen_cards.cards.is_empty() {
        for i in 0..3 {
            if chosen_cards.cards[0][i].is_some() {
                available_cards += 1;
            }
        }
    }

    let available_cards = available_cards;

    if available_cards == 0 {
        for mut visibility in &mut card_ui_query {
            *visibility = Visibility::Hidden;
        }
        for (mut visibility, _) in &mut card_art_query {
            *visibility = Visibility::Hidden;
        }
        for (mut visibility, _) in &mut card_template_query {
            *visibility = Visibility::Hidden;
        }
    } else if available_cards <= 3 {
        for i in 0..available_cards as usize {
            let mut card_ui_visibility = card_ui_query
                .get_mut(cards_helper.card_ui_id[i])
                .expect("Cards Helper");
            *card_ui_visibility = Visibility::Visible;

            let (mut card_template_visibility, mut card_template_image) = card_template_query
                .get_mut(cards_helper.card_template_id[i])
                .unwrap();

            *card_template_visibility = Visibility::Visible;
            let card_name = chosen_cards.cards[0][i].clone().expect("Chosen Cards OOB");
            let card_template_res = cards_res.cards.get(&card_name).expect("Card doesn't exist");
            let card_sprite = card_template_res.sprite.clone();
            let card_rarity = "card_uncommon".to_string();
            *card_template_image = ImageNode {
                image: cards_sprites
                    .cards_sprites
                    .get(card_rarity.as_str())
                    .unwrap_or_else(|| panic!("Card Template not found {:?}", card_rarity))
                    .clone(),
                ..Default::default()
            };

            let (mut card_art_visibility, mut card_art_image) =
                card_art_query.get_mut(cards_helper.card_art_id[i]).unwrap();
            *card_art_visibility = Visibility::Visible;

            *card_art_image = ImageNode {
                image: cards_sprites
                    .cards_sprites
                    .get(card_sprite.as_str())
                    .unwrap_or_else(|| panic!("Card Art not found {:?}", card_sprite))
                    .clone(),
                ..Default::default()
            };
        }
    } else {
        info!("Available Cards: {:?}", available_cards);
    }
}

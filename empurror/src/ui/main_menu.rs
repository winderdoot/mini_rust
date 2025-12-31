use bevy::camera::visibility::RenderLayers;
use bevy::prelude::*;
use bevy::{
    picking::hover::Hovered,
};

use bevy::{ui::*};

use crate::game_logic::game_states::AppState;
use crate::scene::orbit_camera::{GAME_LAYER, OrbitCamera};
use crate::ui::{panels::*, button_update::*};

const MAIN_MENU_FONTSZ: f32 = 35.0;


/* Components */
#[derive(Component, Default)]
pub struct NewGameButton;
#[derive(Component, Default)]
pub struct MainMenuText;

#[derive(Component, Default)]
pub struct MainMenuPanel;

pub fn spawn_main_menu(
    mut commands: Commands
) {
    let text = (
            Text::new("Fur the Empurror: Paws of Dominion"),
            TextFont {
                font_size: MAIN_MENU_FONTSZ,
                ..default()
            },
            TextColor(Color::WHITE),
        );

    let menu = (
        Node {
            display: Display::Flex,
            position_type: PositionType::Relative,
            /* Children */
            align_items: AlignItems::Center,
            align_self: AlignSelf::Center,
            justify_self: JustifySelf::Center,
            align_content: AlignContent::Center,
            flex_direction: FlexDirection::Column,
            row_gap: px(50),
            padding: UiRect::all(px(10)),
            ..Default::default()
        },
        MainMenuPanel,
        children![
            text,
            button::<NewGameButton>(Display::Flex)
        ]
    );

    let container = (
        Node {
            display: Display::Flex,
            width: auto(),
            height: auto(),
            position_type: PositionType::Absolute,
            align_self: AlignSelf::Center,
            justify_self: JustifySelf::Center,
            /* Children */
            align_items: AlignItems::Start,
            flex_direction: FlexDirection::Column,
            // padding: UiRect::new(TPL_PADDING, px(0), TPL_PADDING, TPL_PADDING),
            ..Default::default()
        },
        children![
            menu
        ]
    );

    commands.spawn(container);
}

pub fn update_new_game_button(
    mut button: Single<
        (
            Option<&Pressed>,
            &Hovered,
            Option<&InteractionDisabled>,
            &mut BackgroundColor,
            &Children,
        ),
        With<NewGameButton>
    >,
    button_ent: Single<Entity, With<NewGameButton>>,
    mut text_query: Query<&mut Text>,
    mut commands: Commands,
) {
    let (pressed, hovered, disabled, color, children) = &mut *button;
    let Ok(mut text) = text_query.get_mut(children[0]) else {
        return;
    };

    let button_text = String::from("New Game");
    set_button_style(&button_text, &button_text, disabled.is_some(), hovered.get(), pressed.is_some(), color, &mut text);

    if pressed.is_some() && !disabled.is_some() {
        /* Start the game */

        commands
            .entity(*button_ent)
            .remove::<Pressed>();

        commands.trigger(NewGame);
    }
}

/* Events */
#[derive(Event, Debug)]
pub struct NewGame;

pub fn start_new_game(
    _event: On<NewGame>,
    mut s_nodes: ParamSet<(
        Single<&mut Node, With<EndTurnButton>>,
        Single<&mut Node, With<TreasuryPanel>>,
        Single<&mut Node, With<MainMenuPanel>>
    )>,
    camera: Single<Entity, With<OrbitCamera>>,
    mut next_state: ResMut<NextState<AppState>>,
    mut commands: Commands
) {
    /* Hacky: enable other top level ui */
    let end_turn_button = &mut *s_nodes.p0();
    end_turn_button.display = Display::Flex;

    let treasury = &mut *s_nodes.p1();
    treasury.display = Display::Flex;

    let main_menu = &mut *s_nodes.p2();
    main_menu.display = Display::None;

    commands
        .entity(*camera)
        .insert(RenderLayers::from_layers(&[GAME_LAYER]));

    next_state.set(AppState::InGame);
}
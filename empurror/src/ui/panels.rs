use bevy::color::palettes::{css::*, tailwind::*};
use bevy::prelude::*;
use bevy::{
    input_focus::{
        tab_navigation::{TabIndex},
    },
    picking::hover::Hovered,
};

use bevy_ui_widgets::{Button};


use crate::game_logic::resources::ResourceType;
use crate::scene::assets::{EmpireAssets, Icons};

const TPL_PADDING: Val = Val::Px(20.0);
const PANEL_COLOR: Color = Color::linear_rgba(0.1, 0.1, 0.1, 1.0);
const PANEL_COLOR_TR: Color = Color::linear_rgba(0.1, 0.1, 0.1, 0.5);
pub const BUTTON_COLOR: Color = Color::linear_rgba(0.2, 0.2, 0.2, 1.0);
pub const BUTTON_COLOR_HOVER: Color = Color::linear_rgba(0.3, 0.3, 0.3, 1.0);
pub const BUTTON_COLOR_PRESS: Color = Color::linear_rgba(0.4, 0.4, 0.4, 1.0);

/* Reource/Component definitions */

#[derive(Component)]
pub struct UIProvincePanel;
#[derive(Component)]
pub struct UIDetailedProvincePanel;
#[derive(Component)]
pub struct UIBasicProvincePanel;
#[derive(Component)]

/* Basic panel  */
pub struct UIProvinceFlag;
#[derive(Component)]
pub struct UIProvinceEmpireName;
#[derive(Component)]
pub struct UIProvinceType;
#[derive(Component)]
pub struct UIProvincePopulation;

/* Claim panel */
#[derive(Component)]
pub struct UIClaimProvincePanel;
#[derive(Component)]
pub struct ClaimProvinceButton;

/* Treasury */
#[derive(Component)]
pub struct TreasuryPanel;


/* Detail panel */
#[derive(Component)]
pub struct BuildHouseButton;
#[derive(Component)]
pub struct BuildResourceBuildingButton;
#[derive(Component)]
pub struct UIProductionText;
#[derive(Component)]
pub struct UIUpkeepText;
#[derive(Component)]
pub struct UIHousesText;
#[derive(Component)]
pub struct UIBuildHouseText;
#[derive(Component)]
pub struct UIBuildResourceBuildingText;
#[derive(Component)]
pub struct UIResidentsText;

/* Treasury panel */
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UIResourceType {
    Regular(ResourceType),
    Pops
}
#[derive(Component)]
pub struct UIResourceIncomeText(pub UIResourceType);
#[derive(Component)]
pub struct UIResourceTotalText(pub UIResourceType);

/* New turn button */
#[derive(Component)]
pub struct EndTurnButton;

/* Systems */

fn rounded_container(direction: FlexDirection, gap: Val) -> impl Bundle {
    (
        Node {
            display: Display::None,
            width: auto(),
            height: auto(),
            position_type: PositionType::Relative,
            border: UiRect::all(px(2.0)),
            /* Children */
            align_items: AlignItems::Start,
            flex_direction: direction,
            column_gap: gap,
            row_gap: gap,
            padding: UiRect::all(px(10)),
            ..Default::default()
        },
        BackgroundColor(PANEL_COLOR),
        BorderColor::all(Color::WHITE),
        BorderRadius::all(px(15))
    )
}

fn province_hover_panel(
    empire_assets: &Res<EmpireAssets>
) -> impl Bundle {

    let flag = (
        UIProvinceFlag,
        Node {
            width: px(128),
            height: px(80),
            ..Default::default()
        },
        ImageNode {
            image: empire_assets.flags[0].clone(),
            ..Default::default()
        }
    );

    let description = (
        Node {
            position_type: PositionType::Relative,
            align_self: AlignSelf::Start,
            /* Children */
            align_items: AlignItems::Start,
            flex_direction: FlexDirection::Column,
            row_gap: px(5),
            ..Default::default()
            
        },
        children![
            (
                UIProvinceEmpireName,
                Text::new("The Mew-nited Syndicate"),
                TextFont { 
                   font_size: 20.0,
                    ..Default::default()
                },
                TextColor(Color::WHITE),
                TextLayout::new_with_justify(Justify::Left),
            ),
            (
                UIProvinceType,
                Text::new("Plains"),
                TextFont { 
                   font_size: 16.0,
                    ..Default::default()
                },
                TextColor(Color::WHITE),
                TextLayout::new_with_justify(Justify::Left),
            ),
            (
                UIProvincePopulation,
                Text::new("Population: 0"),
                TextFont { 
                   font_size: 16.0,
                    ..Default::default()
                },
                TextColor(Color::WHITE),
                TextLayout::new_with_justify(Justify::Left),
            )
        ]
    );

    let hover_province_panel = (
        UIBasicProvincePanel,
        rounded_container(FlexDirection::Row, px(10)),
        children![
            flag,
            description
        ]
    );

    hover_province_panel
}

fn build_house_button() -> impl Bundle {
    (
        Node {
            border: UiRect::all(px(2)),
            padding: UiRect::all(px(5)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BuildHouseButton,
        Button,
        Hovered::default(),
        TabIndex(0),
        BorderColor::all(Color::WHITE),
        BorderRadius::all(px(15)),
        BackgroundColor(BUTTON_COLOR),
        children![(
            Text::new("Build House"),
            TextFont {
                font_size: 22.0,
                ..default()
            },
            TextColor(Color::WHITE),
        )],
    )
}

fn build_resc_building_button() -> impl Bundle {
    (
        Node {
            border: UiRect::all(px(2)),
            padding: UiRect::all(px(5)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BuildResourceBuildingButton,
        Button,
        Hovered::default(),
        TabIndex(0),
        BorderColor::all(Color::WHITE),
        BorderRadius::all(px(15)),
        BackgroundColor(BUTTON_COLOR),
        children![(
            Text::new("Build Farm"),
            TextFont {
                font_size: 22.0,
                ..default()
            },
            TextColor(Color::WHITE),
        )],
    )
}

fn province_detail_panel() -> impl Bundle {
    let production = String::from("Production:\nGrain: +5");
    let upkeep = String::from("Upkeep:\nGrain: -2");

    let resources = (
        Node {
            /* Children */
            align_items: AlignItems::Start,
            flex_direction: FlexDirection::Column,
            row_gap: px(5),
            ..Default::default()
        },
        children![
            (
                UIProductionText,
                Text::new(production),
                TextFont { 
                   font_size: 18.0,
                    ..Default::default()
                },
                TextColor(Color::WHITE),
                TextLayout::new_with_justify(Justify::Left),
            ),
            (
                UIUpkeepText,
                Text::new(upkeep),
                TextFont { 
                   font_size: 18.0,
                    ..Default::default()
                },
                TextColor(Color::WHITE),
                TextLayout::new_with_justify(Justify::Left),
            ),
        ]
    );
    let houses = (
        UIHousesText,
        Text::new("Houses: 3/3"),
        TextFont { 
            font_size: 18.0,
            ..Default::default()
        },
        TextColor(Color::WHITE),
        TextLayout::new_with_justify(Justify::Left),
    );

    let buttons = (
        Node {
            /* Children */
            align_items: AlignItems::Start,
            flex_direction: FlexDirection::Row,
            column_gap: px(5),
            ..Default::default()
        },
        children![
            build_house_button(),
            build_resc_building_button()
        ]
    );

    let residents = (
        Node {
            align_items: AlignItems::Start,
            flex_direction: FlexDirection::Column,
            row_gap: px(5),
            ..Default::default()
        },
        children![
            (
                UIResidentsText,
                Text::new("Assign residents:\n- 4/5 +"),
                TextFont { 
                   font_size: 18.0,
                    ..Default::default()
                },
                TextColor(Color::WHITE),
                TextLayout::new_with_justify(Justify::Left),
            ),
        ]
    );

    let container = (
        UIDetailedProvincePanel,
        rounded_container(FlexDirection::Column, px(5)),
        children![
            resources,
            houses,
            buttons,
            residents
        ]
    );

    container
}

fn claim_province_button() -> impl Bundle {
    (
        Node {
            border: UiRect::all(px(2)),
            padding: UiRect::all(px(5)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        ClaimProvinceButton,
        Button,
        Hovered::default(),
        TabIndex(0),
        BorderColor::all(Color::WHITE),
        BorderRadius::all(px(15)),
        BackgroundColor(BUTTON_COLOR),
        children![(
            Text::new("Claim Province"),
            TextFont {
                font_size: 22.0,
                ..default()
            },
            TextColor(Color::WHITE),
        )],
    )
}

fn province_claim_panel() -> impl Bundle {
    let container = (
        UIClaimProvincePanel,
        claim_province_button()
    );

    container
}

pub fn spawn_province_panel_group(
    empire_assets: Res<EmpireAssets>,
    mut commands: Commands
) {
    let hover = province_hover_panel(&empire_assets);
    let detail = province_detail_panel();
    let claim = province_claim_panel();

    let container = (
        UIProvincePanel,
        Node {
            display: Display::None,
            width: auto(),
            height: auto(),
            position_type: PositionType::Absolute,
            right: px(0),
            top: px(0),
            /* Children */
            align_items: AlignItems::End,
            flex_direction: FlexDirection::Column,
            row_gap: px(10),
            padding: UiRect::new(px(0), TPL_PADDING, TPL_PADDING, TPL_PADDING),
            ..Default::default()
        },
        children![
            hover,
            detail,
            claim
        ]
    );

    commands.spawn(container);
}

pub fn spawn_end_turn_button(
    mut commands: Commands
) {
    let button = (
        Node {
            border: UiRect::all(px(2)),
            padding: UiRect::all(px(5)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        EndTurnButton,
        Button,
        Hovered::default(),
        TabIndex(0),
        BorderColor::all(Color::WHITE),
        BorderRadius::all(px(15)),
        BackgroundColor(BUTTON_COLOR),
        children![(
            Text::new("End Turn"),
            TextFont {
                font_size: 22.0,
                ..default()
            },
            TextColor(Color::WHITE),
        )],
    );

    let container = (
        Node {
            display: Display::None,
            width: auto(),
            height: auto(),
            position_type: PositionType::Absolute,
            right: px(0),
            top: px(0),
            /* Children */
            align_items: AlignItems::End,
            flex_direction: FlexDirection::ColumnReverse,
            row_gap: px(10),
            padding: UiRect::new(px(0), TPL_PADDING, TPL_PADDING, TPL_PADDING),
            ..Default::default()
        },
        children![
            button
        ]
    );

    commands.spawn(container);
}

pub fn format_resource(val: f32) -> String {
    format!("{:.0}", val)
}

pub fn format_income(val: f32) -> String {
    if val >= 0.0 {
        format!("+ {:.0}", val.abs())
    }
    else {
        format!("- {:.0}", val.abs())
    }
}

fn resource_view(
    icon: &Handle<Image>,
    total: f32, income: f32,
    typ: &UIResourceType
) -> (impl Bundle, impl Bundle, impl Bundle) {
    let icon = (
        Node {
            width: px(30),
            height: px(30),
            ..Default::default()
        },
        ImageNode {
            image: icon.clone(),
            ..Default::default()
        }
    );

    let total = (
        UIResourceTotalText(typ.clone()),
        Text::new(format_resource(total)),
        TextFont { 
            font_size: 17.0,
            ..Default::default()
        },
        TextColor(Color::WHITE),
        TextLayout::new_with_justify(Justify::Left),
    );

    let income_color = if income < 0.0 {
        Color::Srgba(ROSE_500)
    } else {
        Color::Srgba(LIMEGREEN)
    };

    let income = (
        UIResourceIncomeText(typ.clone()),
        Text::new(format_income(income)),
        TextFont { 
            font_size: 17.0,
            ..Default::default()
        },
        TextColor(income_color),
        TextLayout::new_with_justify(Justify::Left),
    );

    (icon, total, income)
}

pub fn spawn_treasury_panel(
    mut commands: Commands,
    icons: Res<Icons>
) {
    let (pops_icon, pops_total, pops_income) = resource_view(&icons.pops, 14.0, 3.0, &UIResourceType::Pops);
    let (grain_icon, grain_total, grain_income) = resource_view(
        &icons.grain,
        231.0, 
        -12.0,
        &UIResourceType::Regular(ResourceType::Grain)
    );
    let (lumber_icon, lumber_total, lumber_income) = resource_view(
        &icons.lumber,
        43.0,
        7.0,
        &UIResourceType::Regular(ResourceType::Lumber)
    );
    let (stone_icon, stone_total, stone_income) = resource_view(
        &icons.stone,
        2.0,
        0.0,
        &UIResourceType::Regular(ResourceType::Stone)
    );
    let (gold_icon, gold_total, gold_income) = resource_view(
        &icons.gold,
        21.0,
        37.0,
        &UIResourceType::Regular(ResourceType::Gold)
    );
    let empty = Node {
        width: px(5),
        height: px(5),
        ..Default::default()
    };

    let main_panel = (
        Node {
            width: auto(),
            height: auto(),
            top: TPL_PADDING,
            left: TPL_PADDING,
            position_type: PositionType::Absolute,
            align_self: AlignSelf::Center,
            border: UiRect::all(Val::Px(2.0)),
            /* Children */
            align_items: AlignItems::Start,
            flex_direction: FlexDirection::Row,
            column_gap: px(10),
            padding: UiRect::all(px(10)),
            ..Default::default()
        },
        BackgroundColor(PANEL_COLOR),
        BorderColor::all(Color::WHITE),
        BorderRadius::all(px(25)),
        children![
            pops_icon, pops_total, pops_income, empty.clone(),
            grain_icon, grain_total, grain_income, empty.clone(),
            lumber_icon, lumber_total, lumber_income, empty.clone(),
            stone_icon, stone_total, stone_income, empty.clone(),
            gold_icon, gold_total, gold_income, empty
        ]
    );

    commands.spawn(main_panel);
}

use bevy::{picking::hover::Hovered, prelude::*, ui::*};

use std::cmp::{max};
use crate::{game_logic::{armies::{ProvinceArmies, SoldierType}, empire::*, province::*, turns::EndTurn}, scene::hex_grid::*, ui::{panel_update::*, panels::*}};

fn set_button_style(
    mess: &str,
    disabled_mess: &str,
    disabled: bool,
    hovered: bool,
    pressed: bool,
    color: &mut BackgroundColor,
    text: &mut Text,
) {
    match (disabled, hovered, pressed) {
        // Disabled button
        (true, _, _) => {
            **text = disabled_mess.to_string();
            *color = BUTTON_COLOR_DISABLED.into();
        }

        // Pressed and hovered button
        (false, true, true) => {
            **text = mess.to_string();
            *color = BUTTON_COLOR_PRESS.into();
        }

        // Hovered, unpressed button
        (false, true, false) => {
            **text = mess.to_string();
            *color = BUTTON_COLOR_HOVER.into();
        }

        // Unhovered button (either pressed or not).
        (false, false, _) => {
            **text = mess.to_string();
            *color = BUTTON_COLOR.into();
        }
    }
}


pub fn update_claim_button(
    mut button: Single<
        (
            Option<&Pressed>,
            &Hovered,
            Option<&InteractionDisabled>,
            &mut BackgroundColor,
            &Children,
        ),
        With<ClaimProvinceButton>
    >,
    button_ent: Single<Entity, With<ClaimProvinceButton>>,
    mut text_query: Query<&mut Text>,
    pick: Res<PickedProvince>,
    empires: Res<Empires>,
    mut q_empires: Query<&mut Empire>,
    mut commands: Commands
) {
    let (pressed, hovered, disabled, color, children) = &mut *button;
    let Ok(mut text) = text_query.get_mut(children[0]) else {
        return;
    };
    
    let claim_text = format!("Claim Province ({})", resource_str(&House::build_cost()));
    set_button_style(&claim_text, &claim_text, disabled.is_some(), hovered.get(), pressed.is_some(), color, &mut text);

    if pressed.is_some() && !disabled.is_some() {
        let PickedProvince::Selected(province) = *pick else {
            error!("Missing selected province!");
            return;
        };
        let Some(player_empire) = empires.player_empire() else {
            error!("Missing player empire");
            return;
        };
        let Ok(mut empire_c) = q_empires.get_mut(*player_empire) else {
            error!("Player empire component missing");
            return;
        };
        let cost = House::build_cost();
        /* Assume that validation of this action happened before */
        empire_c.remove_resources(&cost);

        commands.entity(*button_ent).remove::<Pressed>();
        /* This event triggers other events for calculating incomes for province and empire */
        commands.trigger(ProvinceClaimed { empire: *player_empire, province });
    }
}

pub fn update_build_house_button(
    mut button: Single<
        (
            Option<&Pressed>,
            &Hovered,
            Option<&InteractionDisabled>,
            &mut BackgroundColor,
            &Children,
        ),
        With<BuildHouseButton>
    >,
    button_ent: Single<Entity, With<BuildHouseButton>>,
    mut text_query: Query<&mut Text>,
    pick: Res<PickedProvince>,
    empires: Res<Empires>,
    mut q_empires: Query<(Entity, &mut Empire)>,
    mut commands: Commands,
) {
    let (pressed, hovered, disabled, color, children) = &mut *button;
    let Ok(mut text) = text_query.get_mut(children[0]) else {
        return;
    };
    
    let house_text = format!("Build House ({})", resource_str(&House::build_cost()));
    set_button_style(&house_text, &house_text, disabled.is_some(), hovered.get(), pressed.is_some(), color, &mut text);

    if pressed.is_some() && !disabled.is_some() {
        let PickedProvince::Selected(province) = *pick else {
            error!("Missing selected province!");
            return;
        };
        let Some(player_empire) = empires.player_empire() else {
            error!("Missing player empire");
            return;
        };
        let Ok((empire_e, mut empire_c)) = q_empires.get_mut(*player_empire) else {
            error!("Player empire component missing");
            return;
        };
        let cost = House::build_cost();
        empire_c.remove_resources(&cost);

        commands.entity(*button_ent).remove::<Pressed>();
        commands.trigger(HouseAdded { province }); /* Causes the province to calculate it's income too */
        commands.trigger(PopsIncomeChanged { empire: empire_e });
    }
}

pub fn update_build_resource_building_button(
    mut button: Single<
        (
            Option<&Pressed>,
            &Hovered,
            Option<&InteractionDisabled>,
            &mut BackgroundColor,
            &Children,
        ),
        With<BuildResourceBuildingButton>
    >,
    button_ent: Single<Entity, With<BuildResourceBuildingButton>>,
    mut text_query: Query<&mut Text>,
    pick: Res<PickedProvince>,
    q_provinces: Query<(&Province, &ControlledBy)>,
    empires: Res<Empires>,
    mut q_empires: Query<&mut Empire>,
    mut commands: Commands,
) {
    let (pressed, hovered, disabled, color, children) = &mut *button;
    let Ok(mut text) = text_query.get_mut(children[0]) else {
        return;
    };
    
    let PickedProvince::Selected(prov) = *pick else {
        return;
    };
    let building_name =
        q_provinces
            .get(prov)
            .map(|(p, _)| p.building_name())
            .unwrap_or(String::from("Invalid building"));
    let building_type = 
        q_provinces
            .get(prov)
            .map(|(p, _)| p.special_building_type())
            .unwrap_or(Some(SpecialBuilding::Farm))
            .unwrap_or(SpecialBuilding::Farm);
    let build_cost = building_type.build_cost();

    let button_text = format!("Build {} ({})", building_name, resource_str(&build_cost));
    set_button_style(&button_text, &button_text, disabled.is_some(), hovered.get(), pressed.is_some(), color, &mut text);

    if pressed.is_some() && !disabled.is_some() {
        let PickedProvince::Selected(province) = *pick else {
            error!("Missing selected province!");
            return;
        };
        let Some(player_empire) = empires.player_empire() else {
            error!("Missing player empire");
            return;
        };
        let Ok(mut empire_c) = q_empires.get_mut(*player_empire) else {
            error!("Player empire component missing");
            return;
        };

        empire_c.remove_resources(&build_cost);

        commands.entity(*button_ent).remove::<Pressed>();
        commands.trigger(SpecialBuildingAdded { province, castle: false }); /* Causes the province to calculate it's income too */
        commands.trigger(ResourceIncomeChanged { empire: *player_empire });
    }
}

pub fn update_build_castle_button(
    mut button: Single<
        (
            Option<&Pressed>,
            &Hovered,
            Option<&InteractionDisabled>,
            &mut BackgroundColor,
            &Children,
        ),
        With<BuildCastleButton>
    >,
    button_ent: Single<Entity, With<BuildCastleButton>>,
    mut text_query: Query<&mut Text>,
    pick: Res<PickedProvince>,
    q_provinces: Query<&Province>,
    empires: Res<Empires>,
    mut q_empires: Query<&mut Empire>,
    mut commands: Commands,
) {
    let (pressed, hovered, disabled, color, children) = &mut *button;
    let Ok(mut text) = text_query.get_mut(children[0]) else {
        return;
    };
    
    let PickedProvince::Selected(_) = *pick else {
        return;
    };
    let build_cost = SpecialBuilding::Castle.build_cost();

    let button_text = format!("Build {} ({})", SpecialBuilding::Castle, resource_str(&build_cost));
    set_button_style(&button_text, &button_text, disabled.is_some(), hovered.get(), pressed.is_some(), color, &mut text);

    if pressed.is_some() && !disabled.is_some() {
        let PickedProvince::Selected(province) = *pick else {
            error!("Missing selected province!");
            return;
        };
        let Ok(province_c) = q_provinces.get(province) else {
            error!("Missing province component");
            return;
        };
        let Some(player_empire) = empires.player_empire() else {
            error!("Missing player empire");
            return;
        };
        let Ok(mut empire_c) = q_empires.get_mut(*player_empire) else {
            error!("Player empire component missing");
            return;
        };

        empire_c.remove_resources(&build_cost);
        /* A bit hacky, whatever  */
        empire_c.add_free_pops(province_c.get_pops());

        commands.entity(*button_ent).remove::<Pressed>();
        commands.trigger(SpecialBuildingAdded { province, castle: true }); /* Causes the province to calculate it's income too */
        commands.trigger(ResourceIncomeChanged { empire: *player_empire });
        commands.trigger(PopsIncomeChanged { empire: *player_empire });
    }
}

pub fn update_recruit_button(
    mut button: Single<
        (
            Option<&Pressed>,
            &Hovered,
            Option<&InteractionDisabled>,
            &mut BackgroundColor,
            &Children,
        ),
        With<RecruitSoldierButton>
    >,
    button_ent: Single<Entity, With<RecruitSoldierButton>>,
    mut text_query: Query<&mut Text>,
    pick: Res<PickedProvince>,
    empires: Res<Empires>,
    mut commands: Commands,
) {
    let (pressed, hovered, disabled, color, children) = &mut *button;
    let Ok(mut text) = text_query.get_mut(children[0]) else {
        return;
    };
    
    let PickedProvince::Selected(_) = *pick else {
        return;
    };
    let recruit_cost = SoldierType::Infantry.recruit_cost();

    let button_text = format!("Recruit {} ({})", SoldierType::Infantry, resource_str(&recruit_cost));
    set_button_style(&button_text, &button_text, disabled.is_some(), hovered.get(), pressed.is_some(), color, &mut text);

    if pressed.is_some() && !disabled.is_some() {
        let PickedProvince::Selected(province) = *pick else {
            error!("Missing selected province!");
            return;
        };
        let Some(player_empire) = empires.player_empire() else {
            error!("Missing player empire");
            return;
        };

        commands.entity(*button_ent).remove::<Pressed>();
        commands.trigger(SoldierRecruited { soldier: SoldierType::Infantry, empire: *player_empire, province })
    }
}

pub fn update_end_turn_button(
    mut button: Single<
        (
            Option<&Pressed>,
            &Hovered,
            Option<&InteractionDisabled>,
            &mut BackgroundColor,
            &Children,
        ),
        With<EndTurnButton>
    >,
    button_ent: Single<Entity, With<EndTurnButton>>,
    mut text_query: Query<&mut Text>,
    empires: Res<Empires>,
    q_empires: Query<&Empire>,
    mut commands: Commands,
) {
    let (pressed, hovered, disabled, color, children) = &mut *button;
    let Ok(mut text) = text_query.get_mut(children[0]) else {
        return;
    };

    let button_text = String::from("End Turn");
    set_button_style(&button_text, &button_text, disabled.is_some(), hovered.get(), pressed.is_some(), color, &mut text);

    if pressed.is_some() && !disabled.is_some() {
        let Some(player_empire) = empires.player_empire() else {
            error!("Missing player empire");
            return;
        };
        let Ok(empire_c) = q_empires.get(*player_empire) else {
            error!("Player empire component missing");
            return;
        };

        commands.entity(*button_ent).remove::<Pressed>();
        commands.trigger(EndTurn { empire_id: empire_c.id });
    }
}

pub fn update_create_army_button(
    mut button: Single<
        (
            Option<&Pressed>,
            &Hovered,
            Option<&InteractionDisabled>,
            &mut BackgroundColor,
            &Children,
        ),
        With<CreateArmyButton>
    >,
    button_ent: Single<Entity, With<CreateArmyButton>>,
    mut text_query: Query<&mut Text>,
    empires: Res<Empires>,
    pick: Res<PickedProvince>,
    mut commands: Commands,
) {
    let (pressed, hovered, disabled, color, children) = &mut *button;
    let Ok(mut text) = text_query.get_mut(children[0]) else {
        return;
    };

    let button_text = String::from("Create new army");
    set_button_style(&button_text, &button_text, disabled.is_some(), hovered.get(), pressed.is_some(), color, &mut text);

    if pressed.is_some() && !disabled.is_some() {
        let Some(player_empire) = empires.player_empire() else {
            error!("Missing player empire");
            return;
        };
        let PickedProvince::Selected(province_e) = *pick else {
            error!("{}:{} Missing province entity", file!(), line!());
            return;
        };

        commands.entity(*button_ent).remove::<Pressed>();
        commands.trigger(ArmyCreated { empire: *player_empire, province: province_e })
    }
}

pub fn update_disband_army_button(
    mut button: Single<
        (
            Option<&Pressed>,
            &Hovered,
            Option<&InteractionDisabled>,
            &mut BackgroundColor,
            &Children,
        ),
        With<DisbandArmyButton>
    >,
    button_ent: Single<Entity, With<DisbandArmyButton>>,
    mut text_query: Query<&mut Text>,
    mut army_panel: Single<&mut ArmiesPanel>,
    pick: Res<PickedProvince>,
    q_provinces: Query<&ProvinceArmies, With<Province>>,
    mut commands: Commands,
) {
    let (pressed, hovered, disabled, color, children) = &mut *button;
    let Ok(mut text) = text_query.get_mut(children[0]) else {
        return;
    };

    let button_text = String::from("Disband army");
    set_button_style(&button_text, &button_text, disabled.is_some(), hovered.get(), pressed.is_some(), color, &mut text);

    if pressed.is_some() && !disabled.is_some() {
        let PickedProvince::Selected(province_e) = *pick else {
            error!("{}:{} Missing province entity", file!(), line!());
            return;
        };
        let Ok(armies_c) = q_provinces.get(province_e) else {
            error!("{}:{} Missing province armies vector", file!(), line!());
            return;
        };
        let Some(army_ent) = armies_c.armies().get(army_panel.curr_army as usize) else {
            error!("{}:{} Invalid current army index", file!(), line!());
            return;
        };

        army_panel.curr_army = max(1, army_panel.curr_army) - 1;

        commands.entity(*button_ent).remove::<Pressed>();
        commands.trigger(ArmyDisbanded { army: *army_ent });
    }
}
use bevy::{picking::hover::Hovered, platform::collections::HashMap, prelude::*, ui::*};

use crate::{game_logic::{empire::*, province::*, resources::*}, game_systems::GameSystems, scene::{assets::*, hex_grid::*}, ui::panels::*};

pub fn resource_str(map: &HashMap<ResourceType, f32>) -> String {
    map
        .iter()
        .map(|(k ,v)| format!("{}: {}", *k, *v))
        .collect::<Vec<String>>()
        .join(" ")
}

/// Horrible, horrible code
fn update_basic_province_panel(
    province_ent: &Entity,
    q_provinces: &Query<(&Province, Option<&ControlledBy>, Option<&ProvinceBuildings>)>,
    _: &Query<Option<&House>>,
    q_empires: &Query<(&Empire, &Controls)>,
    empire_assets: &Res<EmpireAssets>,
    nodes: &mut ParamSet<(
        Single<&mut Node, With<UIProvincePanel>>,
        Single<&mut Node, With<UIBasicProvincePanel>>,
        Single<&mut Node, With<UIDetailedProvincePanel>>,
        Single<(&mut Node, &mut ImageNode), With<UIProvinceFlag>>,
        Single<&mut Node, With<UIClaimProvincePanel>>,
        Single<&mut Node, With<BuildHouseButton>>,
        Single<&mut Node, With<BuildResourceBuildingButton>>,
        Single<&mut Node, With<ClaimProvinceButton>>,
    )>,
    text: &mut ParamSet<(
        Single<&mut Text, With<UIProvinceEmpireName>>,
        Single<&mut Text, With<UIProvinceType>>,
        Single<&mut Text, With<UIProvincePopulation>>,
        Single<&mut Text, With<UIProductionText>>,
        Single<&mut Text, With<UIUpkeepText>>,
        Single<&mut Text, With<UIHousesText>>,
        Single<&mut Text, With<UIResidentsText>>,
    )>,
) {
    let Ok((p_prov, controlled_by, _)) = q_provinces.get(*province_ent) else {
        return;
    };
    let tpl = &mut *nodes.p0();
    tpl.display = Display::Flex;
    let basic = &mut *nodes.p1();
    basic.display = Display::Flex;
    let details = &mut *nodes.p2();
    details.display = Display::None;
    let claims = &mut *nodes.p4();
    claims.display = Display::None;

    let name;
    let flag;
    if let Some(cb) = controlled_by {
        if let Ok((empire_c, _)) = q_empires.get(cb.0) {
            name = empire_c.name.clone();
            flag = Some(empire_assets.flags[empire_c.id as usize].clone());
        } 
        else {
            error!("Empire entity not found!");
            return;
        }
    } 
    else {
        flag = None;
        name = String::from("Unclaimed territory")
    };

    let empire_name = &mut *(*text).p0();
    empire_name.0 = name;
    
    let (flag_node, flag_image) = &mut *nodes.p3();
    match flag {
        Some(h) => {
            flag_node.display = Display::Flex;
            flag_image.image = h;
        },
        None => {
            flag_node.display = Display::None;
        }
    }
    let p_type = &mut *text.p1();
    p_type.0 = format!("{}", p_prov.ptype);
    let p_population = &mut *text.p2();

    p_population.0 = format!("Population: {}", p_prov.get_pops()); // TODO CALCULATE POPS
}

fn update_claims_panel(
    province: &Entity,
    q_empire: &Query<(&Empire, &Controls)>,
    q_provinces: &Query<(&Province, Option<&ControlledBy>, Option<&ProvinceBuildings>)>,
    q_prov_trans: &Query<&Transform, With<Province>>,
    q_prov_owner: &Query<&ControlledBy, With<Province>>,
    empires: &Res<Empires>,
    nodes: &mut ParamSet<(
        Single<&mut Node, With<UIProvincePanel>>,
        Single<&mut Node, With<UIBasicProvincePanel>>,
        Single<&mut Node, With<UIDetailedProvincePanel>>,
        Single<(&mut Node, &mut ImageNode), With<UIProvinceFlag>>,
        Single<&mut Node, With<UIClaimProvincePanel>>,
        Single<&mut Node, With<BuildHouseButton>>,
        Single<&mut Node, With<BuildResourceBuildingButton>>,
        Single<&mut Node, With<ClaimProvinceButton>>,
    )>,
    buttons: &mut ParamSet<(
        Single<Entity, With<ClaimProvinceButton>>,
        Single<Entity, With<BuildHouseButton>>,
        Single<Entity, With<BuildResourceBuildingButton>>,
    )>,
    commands: &mut Commands,
    grid: &Res<HexGrid>,
) {
    let Some(player_empire) = empires.player_empire() else {
        error!("Player empire not found!");
        return;
    };
    let Ok(t) = q_prov_trans.get(*province) else {
        return;
    };
    let hex = grid.layout.world_pos_to_hex(Vec2::new(t.translation.x, t.translation.z));
    let is_adjacent_and_non_water = hex
        .all_neighbors()
        .iter()
        .any(|h| {
            let Some(ent) = grid.get_entity(h) else {
                return false;
            };
            let Ok((p, _, _)) = q_provinces.get(*ent) else {
                return false;
            };
            if let ProvinceType::Water = p.ptype {
                return false;
            }
            let Ok(owner) = q_prov_owner.get(*ent) else {
                return false;
            };
            return owner.0 == *player_empire;
        });
    
    if !is_adjacent_and_non_water {
        return;
    }

    let Ok((prov, _, _)) = q_provinces.get(*province) else {
        error!("Province entity lacks province component");
        return;
    };

    let Ok((empire_c, _)) = q_empire.get(*player_empire) else {
        error!("Empire component missing");
        return;
    };
    let claim_button = *buttons.p0();
    let house_cost = House::build_cost();
    /* TODO: Check if we have enough resources */
    if !empire_c.can_afford(&house_cost) || !empire_c.has_free_pops() {
        commands
            .entity(claim_button)
            .insert(InteractionDisabled);
    }
    else {
        commands
            .entity(claim_button)
            .remove::<InteractionDisabled>();
    }

    let claims = &mut *nodes.p4();
    claims.display = Display::Flex;
    
    return;
}

fn update_detailed_province_panel(
    prov: &Province,
    empire: &Empire,
    nodes: &mut ParamSet<(
        Single<&mut Node, With<UIProvincePanel>>,
        Single<&mut Node, With<UIBasicProvincePanel>>,
        Single<&mut Node, With<UIDetailedProvincePanel>>,
        Single<(&mut Node, &mut ImageNode), With<UIProvinceFlag>>,
        Single<&mut Node, With<UIClaimProvincePanel>>,
        Single<&mut Node, With<BuildHouseButton>>,
        Single<&mut Node, With<BuildResourceBuildingButton>>,
        Single<&mut Node, With<ClaimProvinceButton>>,
    )>,
    text: &mut ParamSet<(
        Single<&mut Text, With<UIProvinceEmpireName>>,
        Single<&mut Text, With<UIProvinceType>>,
        Single<&mut Text, With<UIProvincePopulation>>,
        Single<&mut Text, With<UIProductionText>>,
        Single<&mut Text, With<UIUpkeepText>>,
        Single<&mut Text, With<UIHousesText>>,
        Single<&mut Text, With<UIResidentsText>>,
    )>,
    buttons: &mut ParamSet<(
        Single<Entity, With<ClaimProvinceButton>>,
        Single<Entity, With<BuildHouseButton>>,
        Single<Entity, With<BuildResourceBuildingButton>>,
    )>,
    commands: &mut Commands
) {
    let income = prov.get_income();
    let income_text = &mut *text.p3();

    income_text.0 = format!("Income: {}", resource_str(income));

    let upkeep = prov.get_upkeep();
    let upkeep_text = &mut *text.p4();

    upkeep_text.0 = format!("Upkeep: {}", resource_str(upkeep)); 

    let houses_text = &mut *text.p5();
    houses_text.0 = format!("Houses: {}/{}", prov.get_houses(), MAX_HOUSES);

    let residents_text = &mut *text.p6();
    residents_text.0 = format!("Assign residents:\n- {}/{} +", prov.get_pops(), prov.get_max_pops());
    
    let house_button = *buttons.p1();
    let house_cost = House::build_cost();

    /* Can we build a house */
    if prov.get_houses() >= MAX_HOUSES || !empire.can_afford(&house_cost) {
        commands
            .entity(house_button)
            .insert(InteractionDisabled);
    }
    else {
        commands
            .entity(house_button)
            .remove::<InteractionDisabled>();
    }

    /* Can we build a special/resource building */
    let button = *buttons.p2();
    let can_build = prov
        .special_building_type()
        .as_ref()
        .map(SpecialBuilding::build_cost)
        .map(|cost| empire.can_afford(&cost))
        .unwrap_or(false);

    if !can_build {
        // info!("Cannot build building!");
        commands
            .entity(button)
            .insert(InteractionDisabled);
    }
    else {
        // info!("Can build building!");
        commands
            .entity(button)
            .remove::<InteractionDisabled>();
    }
    
    let detailed = &mut *nodes.p2();
    detailed.display = Display::Flex;
}

/* Terrible code, I am in tears  */
pub fn update_province_panel_group(
    picked: Res<PickedProvince>,
    q_provinces: Query<(&Province, Option<&ControlledBy>, Option<&ProvinceBuildings>)>,
    q_prov_trans: Query<&Transform, With<Province>>,
    q_prov_owner: Query<&ControlledBy, With<Province>>,
    q_houses: Query<Option<&House>>,
    q_empire: Query<(&Empire, &Controls)>,
    empire_assets: Res<EmpireAssets>,
    empires: Res<Empires>,
    mut nodes: ParamSet<(
        Single<&mut Node, With<UIProvincePanel>>,
        Single<&mut Node, With<UIBasicProvincePanel>>,
        Single<&mut Node, With<UIDetailedProvincePanel>>,
        Single<(&mut Node, &mut ImageNode), With<UIProvinceFlag>>,
        Single<&mut Node, With<UIClaimProvincePanel>>,
        Single<&mut Node, With<BuildHouseButton>>,
        Single<&mut Node, With<BuildResourceBuildingButton>>,
        Single<&mut Node, With<ClaimProvinceButton>>,
    )>,
    mut text: ParamSet<(
        Single<&mut Text, With<UIProvinceEmpireName>>,
        Single<&mut Text, With<UIProvinceType>>,
        Single<&mut Text, With<UIProvincePopulation>>,
        Single<&mut Text, With<UIProductionText>>,
        Single<&mut Text, With<UIUpkeepText>>,
        Single<&mut Text, With<UIHousesText>>,
        Single<&mut Text, With<UIResidentsText>>,
    )>,
    mut buttons: ParamSet<(
        Single<Entity, With<ClaimProvinceButton>>,
        Single<Entity, With<BuildHouseButton>>,
        Single<Entity, With<BuildResourceBuildingButton>>,
    )>,
    mut commands: Commands,
    grid: Res<HexGrid>,
) {
    match *picked {
        PickedProvince::None => {
            let tpl = &mut *nodes.p0();
            tpl.display = Display::None;
        },
        PickedProvince::Hovered(hovered) => {
            update_basic_province_panel(&hovered, &q_provinces, &q_houses, &q_empire, &empire_assets, &mut nodes, &mut text);
        },
        PickedProvince::Selected(selected) => {
            update_basic_province_panel(&selected, &q_provinces, &q_houses, &q_empire, &empire_assets, &mut nodes, &mut text);
            let Ok((prov, controlled_by, _)) = q_provinces.get(selected) else {
                return;
            };
            /* Claim province button */
            let Some(controlled_by) = controlled_by else {
                update_claims_panel(&selected, &q_empire, &q_provinces, &q_prov_trans, &q_prov_owner, &empires, &mut nodes, &mut buttons, &mut commands, &grid);
                return;
            };
            let Ok((empire_c, _)) = q_empire.get(controlled_by.0) else {
                return;
            };
            if empire_c.id != PLAYER_EMPIRE {
                return;
            }
            /* Display/modify detailed province panel */
            update_detailed_province_panel(prov, &empire_c, &mut nodes, &mut text, &mut buttons, &mut commands);
        },
    }
}

pub fn update_treasury_panel(
    empires: Res<Empires>,
    q_empires: Query<&Empire>,
    mut s_text: ParamSet<(
        Query<(&mut Text, &UIResourceTotalText)>,
        Query<(&mut Text, &UIResourceIncomeText)>,
    )>
) {
    let Some(player_empire) = empires.get_entity(PLAYER_EMPIRE) else {
        error!("Missing player empire entity");
        return;
    };

    let Ok(empire_c) = q_empires.get(*player_empire) else {
        error!("Missing player empire component");
        return;
    };
    let total_text = &mut s_text.p0();
    total_text
        .iter_mut()
        .for_each(|(mut text, resource)| {
            match resource.0 {
                UIResourceType::Regular(inner) => {
                    let total = empire_c.get_total(&inner);
                    text.0 = format_resource(total);
                },
                UIResourceType::Pops => {
                    text.0 = format!("{}, free: {}", empire_c.get_pops(), empire_c.get_free_pops());
                },
            }
        });

    let income_text = &mut s_text.p1();
    income_text
        .iter_mut()
        .for_each(|(mut text, resource)| {
            match resource.0 {
                UIResourceType::Regular(inner) => {
                    let total = empire_c.get_total(&inner);
                    text.0 = format_income(total);
                },
                UIResourceType::Pops => {
                    let total = empire_c.get_pops();
                    text.0 = format_income(total as f32);
                },
            }
        });
}

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
            *color = BUTTON_COLOR.into();
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
    
        commands.entity(*button_ent).remove::<Pressed>();
        commands.trigger(ProvinceClaimed { empire: *player_empire, province });
        commands.trigger(ProvinceIncomeChanged { province });
        commands.trigger(ResourceIncomeChanged { empire: *player_empire });
        commands.trigger(PopsIncomeChanged { empire: *player_empire });
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

        commands.entity(*button_ent).remove::<Pressed>();
        commands.trigger(HouseAdded { province });
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

    let button_text = format!("Build {} ({})", building_name, resource_str(&building_type.build_cost()));
    set_button_style(&button_text, &button_text, disabled.is_some(), hovered.get(), pressed.is_some(), color, &mut text);

    if pressed.is_some() && !disabled.is_some() {
        let PickedProvince::Selected(province) = *pick else {
            error!("Missing selected province!");
            return;
        };
        let Ok((_, controlled_by)) = q_provinces.get(province) else {
            error!("Missing components");
            return;
        };
        commands.entity(*button_ent).remove::<Pressed>();
        commands.trigger(SpecialBuildingAdded { province });
        commands.trigger(ProvinceIncomeChanged { province });
        commands.trigger(ResourceIncomeChanged { empire: controlled_by.0 });
    }
}
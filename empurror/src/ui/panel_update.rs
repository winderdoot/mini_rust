use bevy::{color::palettes::{css::*, tailwind::*}, input::keyboard::Key, platform::collections::HashMap, prelude::*, ui::*};

use crate::{game_logic::{empire::*, province::*, resources::*, turns::Turns}, scene::{assets::*, hex_grid::*}, ui::panels::*};

pub fn resource_str(map: &HashMap<ResourceType, f32>) -> String {
    map
        .iter()
        .map(|(k ,v)| format!("{}: {}", *k, *v))
        .collect::<Vec<String>>()
        .join(" ")
}

pub fn income_color(val: f32) -> Color {
    if val == 0.0 {
        Color::Srgba(WHITE)
    }
    else if val > 0.0 {
        Color::Srgba(LIMEGREEN)
    }
    else {
        Color::Srgba(ROSE_500)
    }
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
    turns: &Res<Turns>
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

    let Ok((_, _, _)) = q_provinces.get(*province) else {
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
    commands: &mut Commands,
    turns: &Res<Turns>
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
    let can_afford = prov
        .special_building_type()
        .as_ref()
        .map(SpecialBuilding::build_cost)
        .map(|cost| empire.can_afford(&cost))
        .unwrap_or(false);

    if !can_afford || !prov.can_build_special_building(){
        commands
            .entity(button)
            .insert(InteractionDisabled);
    }
    else {
        commands
            .entity(button)
            .remove::<InteractionDisabled>();
    }
    
    let detailed = &mut *nodes.p2();
    detailed.display = Display::Flex;
}

pub fn assign_residents_interaction(
    keyboard: Res<ButtonInput<Key>>,
    pick: Res<PickedProvince>,
    panel_node: Single<&Node, With<UIDetailedProvincePanel>>,
    mut q_empires: Query<&mut Empire>,
    mut q_provinces: Query<&mut Province>,
    empires: Res<Empires>,
    mut commands: Commands,
    turns: Res<Turns>
) {
    if panel_node.display != Display::Flex {
        return;
    }
    let increase = keyboard.just_released(Key::Character("+".into()));
    let decrease = keyboard.just_released(Key::Character("-".into()));
    if !(increase ^ decrease) {
        return;
    }

    let PickedProvince::Selected(prov_ent) = *pick else {
        return;
    };
    let Ok(mut prov) = q_provinces.get_mut(prov_ent) else {
        return;
    };
    let Some(player_empire) = empires.get_entity(PLAYER_EMPIRE) else {
        error!("Missing player empirr entity");
        return;
    };
    let Ok(mut empire_c) = q_empires.get_mut(*player_empire) else {
        error!("Missing player empire component");
        return;
    };
    if increase && empire_c.has_free_pops() {
        prov.try_add_pop().then(|| empire_c.try_remove_free_pop());
    }
    if decrease && prov.try_remove_pop() {
        empire_c.add_free_pop();
    }

    commands.trigger(ProvinceIncomeChanged { province: prov_ent });
    commands.trigger(ResourceIncomeChanged { empire: *player_empire });
    commands.trigger(PopsIncomeChanged { empire: *player_empire });
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
    turns: Res<Turns>
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
                update_claims_panel(&selected, &q_empire, &q_provinces, &q_prov_trans, &q_prov_owner, &empires, &mut nodes, &mut buttons, &mut commands, &grid, &turns);
                return;
            };
            let Ok((empire_c, _)) = q_empire.get(controlled_by.0) else {
                return;
            };
            if empire_c.id != PLAYER_EMPIRE {
                return;
            }
            /* Display/modify detailed province panel */
            update_detailed_province_panel(prov, &empire_c, &mut nodes, &mut text, &mut buttons, &mut commands, &turns);
        },
    }
}

pub fn update_treasury_panel(
    empires: Res<Empires>,
    q_empires: Query<&Empire>,
    mut s_text: ParamSet<(
        Query<(&mut Text, &UIResourceTotalText)>,
        Query<(&mut Text, &mut TextColor, &UIResourceIncomeText)>,
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
        .for_each(|(mut text, mut color, resource)| {
            match resource.0 {
                UIResourceType::Regular(inner) => {
                    let income = empire_c.get_income(&inner);
                    text.0 = format_income(income);
                    color.0 = income_color(income);
                },
                UIResourceType::Pops => {
                    let income = empire_c.get_pops_income();
                    text.0 = format_income(income as f32);
                    color.0 = income_color(income as f32);
                },
            }
        });
}


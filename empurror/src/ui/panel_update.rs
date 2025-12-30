use bevy::{color::palettes::{css::*, tailwind::*}, input::keyboard::Key, platform::collections::HashMap, prelude::*, ui::*};

use strum::IntoEnumIterator;
use std::cmp::*;

use crate::{game_logic::{armies::{Army, ProvinceArmies, SoldierType}, empire::*, province::*, resources::*, turns::Turns}, scene::{assets::*, hex_grid::*}, ui::panels::*};

pub fn resource_str(map: &HashMap<ResourceType, f32>) -> String {
    map
        .iter()
        .map(|(k, v)| {
            if *v > 0.0 {
                format!("{}: {}", *k, *v)
            }
            else {
                String::new()
            } 
        })
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

#[derive(Event, Debug)]
pub struct UpdateClaimsPanel {
    pub province: Entity
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
            error!("{}:{} Empire entity not found!", file!(), line!());
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

pub fn update_claims_panel(
    event: On<UpdateClaimsPanel>,
    q_empire: Query<(&Empire, &Controls)>,
    q_provinces: Query<(&Province, Option<&ControlledBy>, Option<&ProvinceArmies>)>,
    q_armies: Query<&Army>,
    empires: Res<Empires>,
    mut nodes: ParamSet<(
        Single<&mut Node, With<UIClaimProvincePanel>>,
        Single<&mut Node, With<ClaimProvinceButton>>,
    )>,
    mut buttons: ParamSet<(
        Single<Entity, With<ClaimProvinceButton>>,
    )>,
    mut commands: Commands,
    grid: Res<HexGrid>,
    _turns: Res<Turns>
) {
    let province_e = event.province;
    let Ok((province_c, controlled_by, armies_c)) = q_provinces.get(province_e) else {
        error!("{}:{} Province entity lacks province component", file!(), line!());
        return;
    };
    if let Some(_) = controlled_by {
        return;
    };

    let Some(player_empire) = empires.player_empire() else {
        error!("{}:{} Player empire not found!", file!(), line!());
        return;
    };
    let hex = province_c.hex();
    let is_adjacent_and_non_water = hex
        .all_neighbors()
        .iter()
        .any(|h| {
            let Some(ent) = grid.get_entity(h) else {
                return false;
            };
            let Ok((p, controlled_by, _)) = q_provinces.get(*ent) else {
                return false;
            };
            if let ProvinceType::Water = p.ptype {
                return false;
            }
            let Some(owner) = controlled_by else {
                return false;
            };
            return owner.0 == *player_empire;
        });
    let has_player_armies = 
    if let Some(armies_c) = armies_c {
        armies_c
            .iter()
            .any(|army_e| {
                let Ok(army_c) = q_armies.get(army_e) else {
                    error!("{}:{} Army component missing", file!(), line!());
                    return false;
                };
                let Some(player_empire_e) = empires.get_entity(PLAYER_EMPIRE) else {
                    error!("{}:{} Player empire entity missing", file!(), line!());
                    return false;
                };
                return *player_empire_e == army_c.empire();
            })
    }
    else {
        false
    };

    if !has_player_armies && !is_adjacent_and_non_water {
        return;
    }

    let Ok((empire_c, _)) = q_empire.get(*player_empire) else {
        error!("{}:{} Empire component missing", file!(), line!());
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

    let claims = &mut *nodes.p0();
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
    _turns: &Res<Turns>
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
    if prov.has_castle() {
        residents_text.0 = String::new();
    }
    else {
        residents_text.0 = format!("Assign residents:\n- {}/{} +", prov.get_pops(), prov.get_max_pops());
    }
    
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
    _turns: Res<Turns>
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
    if prov.has_castle() {
        return;
    }
    let Some(player_empire) = empires.get_entity(PLAYER_EMPIRE) else {
        error!("{}:{} Missing player empirr entity", file!(), line!());
        return;
    };
    let Ok(mut empire_c) = q_empires.get_mut(*player_empire) else {
        error!("{}:{} Missing player empire component", file!(), line!());
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
    // q_prov_trans: Query<&Transform, With<Province>>,
    // q_prov_owner: Query<&ControlledBy, With<Province>>,
    q_houses: Query<Option<&House>>,
    q_empire: Query<(&Empire, &Controls)>,
    empire_assets: Res<EmpireAssets>,
    // empires: Res<Empires>,
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
                /* Change this to an event, because the caller system ran out of available parameters (max 16) */
                commands.trigger(UpdateClaimsPanel { province: selected });
 
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

/* Also handles the castle button  */
pub fn update_recruit_panel(
    picked: Res<PickedProvince>,
    q_provinces: Query<(&Province, &ControlledBy)>,
    q_empires: Query<&Empire>,
    mut nodes: ParamSet<(
        Single<&mut Node, With<RecruitPanel>>,
        Single<&mut Node, With<BuildCastleButton>>,
        Single<&mut Node, With<RecruitSoldierButton>>,
    )>,
    mut text: ParamSet<(
        Single<&mut Text, With<RecruitedSoldiersText>>,
    )>,
    mut buttons: ParamSet<(
        Single<Entity, With<RecruitSoldierButton>>,
        Single<Entity, With<BuildCastleButton>>,
    )>,
    mut commands: Commands,
    _turns: Res<Turns>
) {
    let castle_button = &mut *nodes.p1();
    castle_button.display = Display::None;
    let recruit_panel = &mut *nodes.p0();
    recruit_panel.display = Display::None;

    let PickedProvince::Selected(province) = *picked else {
        return;
    };
    let Ok((province_c, controlled_by)) = q_provinces.get(province) else {
        return;
    };
    /* I discovered you can call .entity() on these wrapper structs with entities */
    let Ok(empire_c) = q_empires.get(controlled_by.entity()) else {
        return;
    };
    if empire_c.id != PLAYER_EMPIRE {
        return;
    }
    if !province_c.has_special_building() {
        /* We can display build castle button */
        let castle_button = &mut *nodes.p1();
        castle_button.display = Display::Flex;
        let castle_button_ent = *buttons.p1();

        let castle_cost = SpecialBuilding::Castle.build_cost();
        if empire_c.can_afford(&castle_cost) {
            commands
                .entity(castle_button_ent)
                .remove::<InteractionDisabled>();
        }
        else {
            commands
                .entity(castle_button_ent)
                .insert(InteractionDisabled);
        }
        return;
    }
    else if !province_c.has_castle() {
        return;
    }

    let recruit_panel = &mut *nodes.p0();
    recruit_panel.display = Display::Flex;

    let soldiers_text = &mut *text.p0();
    soldiers_text.0 = format!("Recruited soldiers: {}/{}", province_c.get_pops(), province_c.get_max_pops());

    let recruit_button = &mut *nodes.p2();
    recruit_button.display = Display::Flex;
    let recruit_button_ent = *buttons.p0();

    let recruit_cost = SoldierType::Infantry.recruit_cost();
    if empire_c.can_afford(&recruit_cost) && empire_c.has_free_pops() && province_c.has_pops_room() {
        commands
            .entity(recruit_button_ent)
            .remove::<InteractionDisabled>();
    }
    else {
        commands
            .entity(recruit_button_ent)
            .insert(InteractionDisabled);
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
        error!("{}:{} Missing player empire entity", file!(), line!());
        return;
    };

    let Ok(empire_c) = q_empires.get(*player_empire) else {
        error!("{}:{} Missing player empire component", file!(), line!());
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
                    text.0 = format!("{}, free: {}, soldiers: {}", empire_c.get_pops(), empire_c.get_free_pops(), empire_c.get_soldiers());
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

pub fn update_armies_panel(
    keyboard: Res<ButtonInput<KeyCode>>,
    picked: Res<PickedProvince>,
    mut q_provinces: Query<(&mut Province, Option<&ControlledBy>, Option<&mut ProvinceArmies>)>,
    mut q_armies: Query<&mut Army>,
    q_empires: Query<&Empire>,
    // empires: Res<Empires>,
    mut nodes: ParamSet<(
        Single<(&mut Node, &mut ArmiesPanel)>,
    )>,
    mut text: ParamSet<(
        Single<&mut Text, With<UnassignedSoldiersText>>,
        Single<&mut Text, With<ArmyTextPre>>,
        Single<(&mut Text, &mut TextFont, &mut TextColor), With<ArmyTextSelected>>,
        Single<&mut Text, With<ArmyTextPost>>,
    )>,
    mut buttons: ParamSet<(
        Single<Entity, With<CreateArmyButton>>,
        Single<Entity, With<DisbandArmyButton>>,
    )>,
    mut commands: Commands,
    _turns: Res<Turns>
) {
    let (tpl_node, _) = &mut *nodes.p0();
    tpl_node.display = Display::None;

    let PickedProvince::Selected(province) = *picked else {
        return;
    };
    let Ok((mut province_c, controlled_by_o, armies_c)) = q_provinces.get_mut(province) else {
        return;
    };
    if let Some(owner) = controlled_by_o {
        let Ok(empire_c) = q_empires.get(owner.entity()) else {
            error!("{}:{} Empire component missing", file!(), line!());
            return;
        };
        if empire_c.id != PLAYER_EMPIRE {
            return;
        }
    }

    
    let player_armies =
    if let Some(armies_c) = armies_c {

        armies_c
            .iter()
            .filter(|army_e| {
                let Ok(army_c) = q_armies.get(*army_e) else {
                    error!("{}:{} Missing army component", file!(), line!());
                    return false;
                };
                let Ok(empire_c) = q_empires.get(army_c.empire()) else {
                    error!("{}:{} Missing empire component", file!(), line!());
                    return false;
                };

                return empire_c.id == PLAYER_EMPIRE;
            })
            .collect::<Vec<Entity>>()
    }
    else {
        Vec::<Entity>::new()
    };
    let armies_count = player_armies.len();
    if province_c.soldier_count() == 0 && armies_count == 0 {
        return;
    }
    
    // let Some(player_empire_e) = empires.get_entity(PLAYER_EMPIRE) else {
    //     error!("{}:{} Missing player empire enitty", file!(), line!());
    //     return;
    // };
    // let Ok(mut player_empire_c) = q_empires.get_mut(*player_empire_e) else {
    //     error!("{}:{} Missing player empire component", file!(), line!());
    //     return;
    // };
    
    /* Display the panel */
    tpl_node.display = Display::Flex;

    /* Update the text for stationed soldiers */
    let avail_soldiers_text = &mut *text.p0();
    let soldier_count = province_c
            .soldiers_iter()
            .fold(HashMap::<SoldierType, u32>::new(), |mut map, soldier| {
                map
                    .entry(soldier.stype)
                    .and_modify(|count| *count += 1)
                    .or_insert(1);

                map
            });
    
    avail_soldiers_text.0 = format!("Unassigned units:\n{}", 
        SoldierType::iter()
            .map(|typ| {
                format!("{}: {}\n", typ, *soldier_count.get(&typ).unwrap_or(&0))
            })
            .collect::<Vec<String>>()
            .join("")
    );

    /* Manage the armies list */
    let mut army_moved = false; /* Hacky dependency */
    let (_, army_panel) = &mut *nodes.p0();
    army_panel.armies = armies_count as u32;
    
    if keyboard.just_released(KeyCode::KeyK) {
        army_panel.curr_army = max(army_panel.curr_army, 1) - 1;
    }
    if keyboard.just_released(KeyCode::KeyJ) && (armies_count > 0) {
        army_panel.curr_army = min(army_panel.curr_army + 1, army_panel.armies - 1);
    }

    if armies_count == 0 {
        let pre_text = &mut *text.p1();
        pre_text.0 = String::from("(No armies stationed here)");

        let (sel_text, _, _) = &mut *text.p2();
        sel_text.0 = String::new();

        let post_text = &mut *text.p3();
        post_text.0 = String::new();
    }
    else {
        /* Pre text */
        let pre_text = &mut *text.p1();
        pre_text.0 = String::from("Stationed armies: (K - up, J - down, M - move)\n");

        player_armies
            .iter()
            .take(army_panel.curr_army as usize)
            .for_each(|army_ent| {
                let Ok(army_c) = q_armies.get(*army_ent) else {
                    error!("{}:{} Missing army component", file!(), line!());
                    return;
                };

                let moved_text = if army_c.moved() { " (moved)" } else { "" };
                pre_text.0.push_str(&format!("{}: {} units{}\n", army_c, army_c.soldier_count(), moved_text));
            });

        /* Selected text */
        let sel_army_ent = player_armies[army_panel.curr_army as usize];
        let Ok(mut sel_army_c) = q_armies.get_mut(sel_army_ent) else {
            error!("{}:{} Missing army component", file!(), line!());
            return;
        };

        /* Needed by some other part of this function */
        army_moved = sel_army_c.moved();
        /* Carry on... */
        
        let moved_text = if sel_army_c.moved() { " (moved)" } else { "" };
        let (sel_text, text_font, text_color) = &mut *text.p2();
        sel_text.0 = format!("{}: {} units{} (H/L to remove/add units)", *sel_army_c, sel_army_c.soldier_count(), moved_text);
        /* These parameters could be the same from the beginning and stay unmodified */
        text_font.font_size = ARMY_SEL_FONTSIZE;
        text_color.0 = ARMY_SEL_COLOR;

        /* Modify the army if it's not locked/moved */
        if !sel_army_c.moved() && sel_army_c.soldier_count() > 1 && keyboard.just_released(KeyCode::KeyH) {
            if let Some(soldier) = sel_army_c.try_remove_soldier() {
                province_c.add_soldier(soldier);
            }
        }
        else if !sel_army_c.moved() && keyboard.just_released(KeyCode::KeyL) {
            let soldier_opt = province_c.try_remove_soldier_type(&sel_army_c.soldier_type());
            if let Some(soldier) = soldier_opt {
                sel_army_c.try_add_soldier(soldier);
            }
        }

        /* Post text */
        let post_text = &mut *text.p3();
        post_text.0 = String::new();

        player_armies
            .iter()
            .skip((army_panel.curr_army + 1) as usize)
            .take((army_panel.armies - army_panel.curr_army - 1) as usize)
            .for_each(|army_ent| {
                let Ok(army_c) = q_armies.get(*army_ent) else {
                    error!("{}:{} Missing army component", file!(), line!());
                    return;
                };

                let moved_text = if army_c.moved() { " (moved)" } else { "" };
                post_text.0.push_str(&format!("{}: {} units{}\n", army_c, army_c.soldier_count(), moved_text));
            });

    }

    /* Handle the buttons */
    let disband_but_ent = &mut *buttons.p1();
    if armies_count == 0 || army_moved {
        commands
            .entity(*disband_but_ent)
            .insert(InteractionDisabled);
    }
    else {
        commands
            .entity(*disband_but_ent)
            .remove::<InteractionDisabled>();
    }

    let create_but_ent = &mut *buttons.p0();
    if province_c.soldier_count() == 0 {
        commands
            .entity(*create_but_ent)
            .insert(InteractionDisabled);
    }
    else {
        commands
            .entity(*create_but_ent)
            .remove::<InteractionDisabled>();
    }
}

pub fn update_diplomacy_panel(
    mut empires: ResMut<Empires>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut s_nodes: ParamSet<(
        Single<&mut Node, With<DiplomacyPanel>>,
    )>,
    mut s_text: ParamSet<(
        Single<&mut Text, With<DiplomacyText>>,
    )>,
) {
    let node = &mut *s_nodes.p0();
    let enabled = node.display == Display::Flex;

    if keyboard.just_released(KeyCode::KeyB) {
        node.display = match enabled {
            true => Display::None,
            false => Display::Flex,
        }
    }
    
    let enabled = node.display == Display::Flex;
    if enabled {
        let text = &mut *s_text.p0();
        text.0 = 
        (1..empires.count())
            .map(|id| {
                let at_peace = empires.at_peace(PLAYER_EMPIRE, id);
                let at_war = empires.at_war(PLAYER_EMPIRE, id);
                let peace_turns = empires.peace_time(PLAYER_EMPIRE, id).unwrap_or(2137);
                let status =
                if at_peace {
                    format!("AT PEACE for {} turns", peace_turns)
                }
                else if at_war {
                    String::from("AT WAR")
                }
                else {
                    String::new()
                };
                let action =
                if at_peace {
                    String::new()
                }
                else if at_war {
                    format!("press {} to make peace", id)
                }
                else {
                    format!("press {} to wage war", id)
                };

                format!("Empire {}: {} {}\n", id, status, action)
            })
            .collect::<Vec<String>>()
            .join("");

        let keycode_to_id: HashMap<KeyCode, u32> = [
            (KeyCode::Digit1, 1),
            (KeyCode::Digit2, 2),
            (KeyCode::Digit3, 3),
            (KeyCode::Digit4, 4),
            (KeyCode::Digit5, 5),
            (KeyCode::Digit6, 6),
            (KeyCode::Digit7, 7),
            (KeyCode::Digit8, 8),
            (KeyCode::Digit9, 9),
        ].into();

        keyboard
            .get_just_released()
            .for_each(|k| {
                if let Some(empire_id) = keycode_to_id.get(k) {
                    if empires.count() <= *empire_id {
                        return;
                    }
                    if empires.at_war(PLAYER_EMPIRE, *empire_id) {
                        /* Try to make peace */
                        empires.set_peace(PLAYER_EMPIRE, *empire_id, 10);
                    }
                    else if !empires.at_peace(PLAYER_EMPIRE, *empire_id) {
                        /* Wage war */
                        empires.set_war(PLAYER_EMPIRE, *empire_id);
                    }
                }
            });
    }
}
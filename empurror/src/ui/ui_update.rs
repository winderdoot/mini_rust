use bevy::prelude::*;

use crate::{game_logic::{empire::{Empire, PLAYER_EMPIRE}, province::*}, scene::{assets::EmpireAssets, hex_grid::PickedProvince}, ui::panels::{UIBasicProvincePanel, UIDetailedProvincePanel, UIProvinceEmpireName, UIProvinceFlag, UIProvincePanel, UIProvincePopulation, UIProvinceType}};

/// Horrible, horrible code
fn update_basic_province_panel(
    province_ent: &Entity,
    q_provinces: &Query<(&Province, Option<&ControlledBy>, Option<&ProvinceBuildings>)>,
    q_houses: &Query<Option<&House>>,
    q_empires: &Query<&Empire>,
    empire_assets: &Res<EmpireAssets>,
    nodes: &mut ParamSet<(
        Single<&mut Node, With<UIProvincePanel>>,
        Single<&mut Node, With<UIBasicProvincePanel>>,
        Single<&mut Node, With<UIDetailedProvincePanel>>,
        Single<(&mut Node, &mut ImageNode), With<UIProvinceFlag>>
    )>,
    text: &mut ParamSet<(
        Single<&mut Text, With<UIProvinceEmpireName>>,
        Single<&mut Text, With<UIProvinceType>>,
        Single<&mut Text, With<UIProvincePopulation>>,
    )>,
) {
    let Ok((p_prov, controlled_by, buildings)) = q_provinces.get(*province_ent) else {
        return;
    };
    let tpl = &mut *nodes.p0();
    tpl.display = Display::Flex;
    let basic = &mut *nodes.p1();
    basic.display = Display::Flex;

    let name;
    let flag;
    if let Some(cb) = controlled_by {
        if let Ok(empire_c) = q_empires.get(cb.0) {
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

    let empire_name = &mut *text.p0();
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

    let pops = 
    if let Some(buildings) = buildings {
        buildings
            .get_buildings()
            .flat_map(|building_ent| q_houses.get(*building_ent))
            .flatten()
            .map(|house| house.residents)
            .sum()
    } else {
        0
    };

    p_population.0 = format!("Population: {}", pops); // TODO CALCULATE POPS
}

pub fn update_province_panel_group(
    picked: Res<PickedProvince>,
    q_provinces: Query<(&Province, Option<&ControlledBy>, Option<&ProvinceBuildings>)>,
    q_houses: Query<Option<&House>>,
    q_empires: Query<&Empire>,
    empire_assets: Res<EmpireAssets>,
    mut nodes: ParamSet<(
        Single<&mut Node, With<UIProvincePanel>>,
        Single<&mut Node, With<UIBasicProvincePanel>>,
        Single<&mut Node, With<UIDetailedProvincePanel>>,
        Single<(&mut Node, &mut ImageNode), With<UIProvinceFlag>>
    )>,
    mut text: ParamSet<(
        Single<&mut Text, With<UIProvinceEmpireName>>,
        Single<&mut Text, With<UIProvinceType>>,
        Single<&mut Text, With<UIProvincePopulation>>,
    )>,
) {
    match *picked {
        PickedProvince::None => {
            let tpl = &mut *nodes.p0();
            tpl.display = Display::None;
            let basic = &mut *nodes.p1();
            basic.display = Display::None;
            let detailed = &mut *nodes.p2();
            detailed.display = Display::None;
        },
        PickedProvince::Hovered(hovered) => {
            update_basic_province_panel(&hovered, &q_provinces, &q_houses, &q_empires, &empire_assets, &mut nodes, &mut text);
        },
        PickedProvince::Selected(selected) => {
            update_basic_province_panel(&selected, &q_provinces, &q_houses, &q_empires, &empire_assets, &mut nodes, &mut text);
            let Ok((p_prov, controlled_by, buildings)) = q_provinces.get(selected) else {
                return;
            };
            let Some(controlled_by) = controlled_by else {
                return;
            };
            let Ok(empire_c) = q_empires.get(controlled_by.0) else {
                return;
            };
            if empire_c.id != PLAYER_EMPIRE {
                return;
            }

            let detailed = &mut *nodes.p2();
            detailed.display = Display::Flex;

            
        },
    }
}
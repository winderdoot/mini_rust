use bevy::{prelude::*};

use crate::game_logic::{armies::*, empire::Empire, game_states::{ArmyMovementView, GridViewMode}, province::*, recently_moved::RecentlyMoved};
use crate::scene::{orbit_camera::OrbitCamera, hex_grid::*};

fn default_prov_material(
    province: &Entity,
    q_provinces: &mut Query<(&mut MeshMaterial3d<StandardMaterial>, &Province, Option<&ControlledBy>)>,
    q_empires: &Query<&Empire>,
    s_grid: &State<GridViewMode>,
    s_movement: &State<ArmyMovementView>,
    movement: &ArmyMovement,
    settings: &HexGridSettings,
) -> Option<Handle<StandardMaterial>> {
    let Ok((_, province_c, controlled_o)) = q_provinces.get_mut(*province) else {
        return None;
    };

    let material =
    if *s_movement.get() == ArmyMovementView::On && movement.province_reachable(province) {
        settings.reachable_material(&province_c.ptype).clone()
    }
    else if *s_grid.get() == GridViewMode::Empire && let Some(controlled_by) = controlled_o {
        let Ok(empire_c) = q_empires.get(controlled_by.entity()) else {
            error!("{}:{} Missing empire component", file!(), line!());
            return None;
        };
        settings.empire_material(&province_c.ptype, empire_c.id).clone()
    }
    else {
        settings.terrain_material(&province_c.ptype).clone()
    };
    let Some(material) = material else {
        error!("{}:{} missing material", file!(), line!());
        return None;
    };

    Some(material)
}

/* I've scrapped the generic system for tile hovering and accepted code duplication, 
 * because I need more control here over what happens here. */

pub fn cursor_enter_tile(
    event: On<Pointer<Over>>,
    mut q_provinces: Query<(&mut MeshMaterial3d<StandardMaterial>, &Province)>,
    camera_moved: Single<&RecentlyMoved, With<OrbitCamera>>,
    settings: Res<HexGridSettings>,
    mut picked: ResMut<PickedProvince>
) {
    if let PickedProvince::Selected(_) = *picked {
        return;
    }
    if camera_moved.0 {
        return;
    }

    /* Change material of entered tile  */
    let Ok((mut mat, ty)) = q_provinces.get_mut(event.entity) else {
        return;
    };
    let Some(material) = settings.hover_material(&ty.ptype) else {
        error!("{}:{} missing material", file!(), line!());
        return;
    };
    mat.0 = material;
    /* Just hope very hard that the event's come in order and the framerate is high */
    *picked = PickedProvince::Hovered(event.entity);
}

pub fn cursor_exit_tile(
    event: On<Pointer<Out>>,
    mut q_provinces: Query<(&mut MeshMaterial3d<StandardMaterial>, &Province, Option<&ControlledBy>)>,
    q_empires: Query<&Empire>,
    view_mode: Res<State<GridViewMode>>,
    move_mode: Res<State<ArmyMovementView>>,
    movement: Res<ArmyMovement>,
    settings: Res<HexGridSettings>,
    mut picked: ResMut<PickedProvince>
) {
    if let PickedProvince::Selected(_) | PickedProvince::None = *picked {
        return;
    }

    /* Change material of exited tile  */
    let Some(def_mat) = default_prov_material(&event.entity, &mut q_provinces, &q_empires, &view_mode, &move_mode, &movement, &settings) else {
        return;
    };
    let Ok((mut mat, _, _)) = q_provinces.get_mut(event.entity) else {
        return;
    };
    mat.0 = def_mat.clone();
    /* Just hope very hard that the event's come in order and the framerate is high */
    *picked = PickedProvince::None;
}

pub fn cursor_select_tile(
    event: On<Pointer<Press>>,
    mut q_provinces: Query<(&mut MeshMaterial3d<StandardMaterial>, &Province, Option<&ControlledBy>)>,
    q_empires: Query<&Empire>,
    view_mode: Res<State<GridViewMode>>,
    move_mode: Res<State<ArmyMovementView>>,
    mut movement: ResMut<ArmyMovement>,
    mut next_state: ResMut<NextState<ArmyMovementView>>,
    settings: Res<HexGridSettings>,
    mut picked: ResMut<PickedProvince>,
    mut commands: Commands
) {
    if event.button != PointerButton::Primary {
        return;
    }

    if *move_mode.get() == ArmyMovementView::On && !movement.province_reachable(&event.entity) {
        return;
    }
    
    match *picked {
        PickedProvince::Hovered(hovered) => {
            /* Reset the material for the previously hovered tile to default */
            let Some(def_mat) = default_prov_material(&hovered, &mut q_provinces, &q_empires, &view_mode, &move_mode, &movement, &settings) else {
                return;
            };
            let Ok((mut hov_mat, _, _)) = q_provinces.get_mut(hovered) else {
                return;
            };
            hov_mat.0 = def_mat.clone();
        },
        PickedProvince::Selected(old_selected) => {
            /* Deselect the old province and return */
            let Some(def_mat) = default_prov_material(&old_selected, &mut q_provinces, &q_empires, &view_mode, &move_mode, &movement, &settings) else {
                return;
            };
            let Ok((mut old_mat, _, _)) = q_provinces.get_mut(old_selected) else {
                return;
            };
            old_mat.0 = def_mat.clone();

            *picked = PickedProvince::None;

            return;
        },
        PickedProvince::None => {}
    };

    if *move_mode.get() == ArmyMovementView::On {
        let Some(army_e) = movement.pop_army() else {
            error!("{}:{} Missing army entity", file!(), line!());
            return;
        };
        commands.trigger(ArmyMoved { army: army_e, province: event.entity });
        *picked = PickedProvince::Selected(event.entity);
        next_state.set(ArmyMovementView::Off);

        return;
    }

    /* Change material of selected tile  */
    let Ok((mut material, ty, _)) = q_provinces.get_mut(event.entity) else {
        return;
    };
    let Some(mat) = settings.select_material(&ty.ptype) else {
        error!("{}:{} ", file!(), line!());
        return;
    };
    material.0 = mat;
    *picked = PickedProvince::Selected(event.entity);

}

pub fn reset_province_materials(
    pick: Res<PickedProvince>,
    movement: Res<ArmyMovement>,
    s_movement: Res<State<ArmyMovementView>>,
    s_grid: Res<State<GridViewMode>>,
    settings: Res<HexGridSettings>,
    q_provinces: Query<(Entity, &Province, Option<&ControlledBy>)>,
    q_empires: Query<&Empire>,
    mut commands: Commands
) {
    q_provinces
        .iter()
        .for_each(|(province_e, province_c, controlled_o)| {
            let material_o =
            if let PickedProvince::Selected(selected) = *pick && selected == province_e {
                settings.select_material(&province_c.ptype).clone()
            }
            else if *s_movement.get() == ArmyMovementView::On && movement.province_reachable(&province_e) {
                settings.reachable_material(&province_c.ptype).clone()
            }
            else if *s_grid.get() == GridViewMode::Empire && let Some(controlled_by) = controlled_o {
                let Ok(empire_c) = q_empires.get(controlled_by.entity()) else {
                    error!("{}:{} Missing empire component", file!(), line!());
                    return;
                };
                settings.empire_material(&province_c.ptype, empire_c.id).clone()
            }
            else {
                settings.terrain_material(&province_c.ptype).clone()
            };
            let Some(material) = material_o else {
                error!("{}:{} missing material", file!(), line!());
                return;
            };

            commands
                .entity(province_e)
                .insert(MeshMaterial3d(material));
        })
}


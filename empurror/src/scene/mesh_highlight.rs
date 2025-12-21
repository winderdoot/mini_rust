use bevy::{prelude::*};

use crate::game_logic::{empire::{Controls, Empire}, game_states::GridViewMode, province::*, recently_moved::RecentlyMoved};
use crate::scene::{orbit_camera::OrbitCamera, hex_grid::*};

#[derive(Component, Default)]
pub struct Highlightable {
    pub highlighted: bool
}

#[derive(Component, Default)]
pub struct Selectable {
    pub selected: bool
}

fn default_prov_material(
    province: &Entity,
    q_provinces: &mut Query<(&mut MeshMaterial3d<StandardMaterial>, &Province, Option<&ControlledBy>)>,
    q_empires: &Query<&Empire>,
    view_mode: &Res<State<GridViewMode>>,
    settings: &Res<HexGridSettings>,
) -> Option<Handle<StandardMaterial>> {
    let Ok((_, ty, opt)) = q_provinces.get_mut(*province) else {
        return None;
    };
    if *view_mode.get() == GridViewMode::Empire && let Some(controlled_by) = opt {
        let Ok(empire) = q_empires.get(controlled_by.0) else {
            return None;
        };
        Some(settings.empire_material(&ty.ptype, empire.id))
    }
    else {
        Some(settings.province_material(&ty.ptype))
    }
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
    mat.0 = settings.hover_material(&ty.ptype);
    /* Just hope very hard that the event's come in order and the framerate is high */
    *picked = PickedProvince::Hovered(event.entity);
}

pub fn cursor_exit_tile(
    event: On<Pointer<Out>>,
    mut q_provinces: Query<(&mut MeshMaterial3d<StandardMaterial>, &Province, Option<&ControlledBy>)>,
    q_empires: Query<&Empire>,
    view_mode: Res<State<GridViewMode>>,
    settings: Res<HexGridSettings>,
    mut picked: ResMut<PickedProvince>
) {
    if let PickedProvince::Selected(_) | PickedProvince::None = *picked {
        return;
    }

    /* Change material of exited tile  */
    let Some(def_mat) = default_prov_material(&event.entity, &mut q_provinces, &q_empires, &view_mode, &settings) else {
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
    settings: Res<HexGridSettings>,
    mut picked: ResMut<PickedProvince>
) {
    if event.button != PointerButton::Primary {
        return;
    }
    
    match *picked {
        PickedProvince::Hovered(hovered) => {
            /* Reset the material for the previously hovered tile to default */
            let Some(def_mat) = default_prov_material(&hovered, &mut q_provinces, &q_empires, &view_mode, &settings) else {
                return;
            };
            let Ok((mut hov_mat, _, _)) = q_provinces.get_mut(hovered) else {
                return;
            };
            hov_mat.0 = def_mat.clone();
        },
        PickedProvince::Selected(old_selected) => {
            /* Deselect the old province and return */
            let Some(def_mat) = default_prov_material(&old_selected, &mut q_provinces, &q_empires, &view_mode, &settings) else {
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

    /* Change material of selected tile  */
    let Ok((mut material, ty, _)) = q_provinces.get_mut(event.entity) else {
        return;
    };
    material.0 = settings.select_material(&ty.ptype);
    *picked = PickedProvince::Selected(event.entity);

}

pub fn set_empire_materials(
    settings: Res<HexGridSettings>,
    q_empires: Query<(&Empire, &Controls)>,
    q_provinces: Query<&Province>,
    mut commands: Commands
) {
    q_empires
        .iter()
        .for_each(|(empire, controls)| {
            controls.get_provinces()
                .for_each(|tile_ent| {
                    let prov = &q_provinces.get(*tile_ent).unwrap().ptype;
                    commands
                        .entity(*tile_ent)
                        .insert(MeshMaterial3d(settings.empire_material(prov, empire.id).clone()));
                })
        })
}

pub fn set_terrain_materials(
    settings: Res<HexGridSettings>,
    q_provinces: Query<(Entity, &Province)>,
    mut commands: Commands
) {
    q_provinces
        .iter()
        .for_each(|(tile_ent, p)| {
            commands
                .entity(tile_ent)
                .insert(MeshMaterial3d(settings.province_material(&p.ptype).clone()));
        })
}


use bevy::{picking::backend::HitData, prelude::*};

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

pub fn tile_hover<E: EntityEvent>(
    event: On<E>,
    mut query: Query<(&mut MeshMaterial3d<StandardMaterial>, &mut Highlightable, &Province, Option<&ControlledBy>)>,
    q_empires: Query<&Empire>,
    camera_moved: Single<&RecentlyMoved, With<OrbitCamera>>,
    view_mode: Res<State<GridViewMode>>,
    settings: Res<HexGridSettings>
) {
    let entity = event.event_target();
    if let Ok((mut material, mut h, prov, empire)) = query.get_mut(entity) {
        h.highlighted = !h.highlighted;

        if h.highlighted && !camera_moved.0{
            material.0 = settings.hover_material(&prov.prov_type).clone();
        }
        else {
            let default_mat = 
            if *view_mode.get() == GridViewMode::Empire && let Some(controlled_by) = empire {
                let empire_id = q_empires.get(controlled_by.0).unwrap().id;
                settings.empire_material(&prov.prov_type, empire_id)
            } 
            else {
                settings.province_material(&prov.prov_type)
            };
            material.0 = default_mat;
        }
    }
}

pub fn tile_select(
    event: On<Pointer<Press>>,
    mut query: Query<(&mut MeshMaterial3d<StandardMaterial>, &mut Highlightable, &Province, Option<&ControlledBy>)>,
    q_empires: Query<&Empire>,
    camera_moved: Single<&RecentlyMoved, With<OrbitCamera>>,
    view_mode: Res<State<GridViewMode>>,
    settings: Res<HexGridSettings>,
    hex_grid: ResMut<HexGrid>,
    mut commands: Commands
) {
    if event.button != PointerButton::Primary {
        return;
    }
    /* TODO: Rethink tile hovering. Suck it up, whatever... It's going to be worth it I hope */

    info!("Selected!");
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
                    let prov = &q_provinces.get(*tile_ent).unwrap().prov_type;
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
                .insert(MeshMaterial3d(settings.province_material(&p.prov_type).clone()));
        })
}


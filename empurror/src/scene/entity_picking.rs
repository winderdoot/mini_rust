use bevy::{prelude::*};

use crate::scene::hex_grid::*;
use crate::game_logic::province::*;
use crate::game_logic::game_states::*;
use crate::scene::orbit_camera::OrbitCamera;
use crate::scene::recently_moved::RecentlyMoved;

#[derive(Component)]
pub struct Highlightable {
    pub highlighted: bool
}

impl Default for Highlightable {
    fn default() -> Self {
        Self { highlighted: false }
    }
}

pub fn tile_hover<E: EntityEvent>(
    event: On<E>,
    mut query: Query<(&mut MeshMaterial3d<StandardMaterial>, &mut Highlightable, &Province)>,
    camera_moved: Single<&RecentlyMoved, With<OrbitCamera>>,
    settings: Res<HexGridSettings>
) {
    let entity = event.event_target();
    if let Ok((mut material, mut h, prov)) = query.get_mut(entity) {
        h.highlighted = !h.highlighted;

        if h.highlighted && !camera_moved.0{
            material.0 = settings.hover_material(&prov.prov_type).clone();
        }
        else {
            material.0 = settings.province_material(&prov.prov_type)
        }
    }
}   
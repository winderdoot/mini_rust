use bevy::{prelude::*};

use crate::{game_logic::{game_states::GridViewMode, province::Province}, scene::{mesh_highlight::*}, system_sets::*};

/* Systems */
fn toggle_province_view(
    keyboard: Res<ButtonInput<KeyCode>>,
    view: Res<State<GridViewMode>>,
    mut next_view: ResMut<NextState<GridViewMode>>
) {
    if keyboard.just_pressed(KeyCode::KeyV) {
        match view.get() {
            GridViewMode::Terrain => next_view.set(GridViewMode::Empire),
            GridViewMode::Empire => next_view.set(GridViewMode::Terrain),
        }
    }
}

fn unselect_province(
    keyboard: Res<ButtonInput<KeyCode>>,
    q_selected: Query<&Selectable, With<Province>>,
) {

}

/* Init Plugin */
pub struct UIControlsPlugin;

impl Plugin for UIControlsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, 
                toggle_province_view
                .in_set(UpdateSystems::UIControls)
            )
            .add_systems(OnEnter(GridViewMode::Empire),
                set_empire_materials
            )
            .add_systems(OnExit(GridViewMode::Empire),
                set_terrain_materials
            );  
    }
}
use bevy::{prelude::*};
use bevy_ui_widgets::UiWidgetsPlugins;

use crate::{game_logic::game_states::GridViewMode, scene::mesh_highlight::*, game_systems::*, ui::{panels::*, ui_update::*}};

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

/* Init Plugin */
pub struct GameUIPlugin;

impl Plugin for GameUIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(UiWidgetsPlugins)
            .add_systems(Startup, 
                (
                    spawn_province_panel_group,
                    spawn_treasury_panel
                )
                .in_set(StartupSystems::CreateUI)
            )
            .add_systems(Update, 
                (
                    toggle_province_view,
                    update_province_panel_group,
                    update_claim_button,
                    update_claim_button_depress
                )
                .in_set(UpdateSystems::UIUpdate)
            )
            .add_systems(OnEnter(GridViewMode::Empire),
                set_empire_materials
            )
            .add_systems(OnExit(GridViewMode::Empire),
                set_terrain_materials
            );  
    }
}
use bevy::{prelude::*};
use bevy_ui_widgets::UiWidgetsPlugins;

use crate::{game_logic::game_states::*, scene::mesh_highlight::*, game_systems::*, ui::{panels::*, panel_update::*, button_update::*, views::*}};

/* Init Plugin */
pub struct GameUIPlugin;

impl Plugin for GameUIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(UiWidgetsPlugins)
            .add_systems(Startup, 
                (
                    spawn_province_panel_group,
                    spawn_treasury_panel,
                    spawn_end_turn_button,
                    spawn_units_panel_group
                )
                .in_set(StartupSystems::CreateUI)
            )
            .add_systems(Update, 
                (
                    toggle_province_view,
                    toggle_movement_view,
                    assign_residents_interaction,
                    update_treasury_panel,
                    update_province_panel_group,
                    update_recruit_panel,
                    update_claim_button,
                    update_build_house_button,
                    update_build_resource_building_button,
                    update_build_castle_button,
                    update_recruit_button,
                    update_end_turn_button,
                    update_armies_panel,
                    update_create_army_button,
                    update_disband_army_button
                )
                .in_set(UpdateSystems::UIUpdate)
            )
            .add_systems(OnEnter(GridViewMode::Empire),
                reset_province_materials
            )
            .add_systems(OnExit(GridViewMode::Empire),
                reset_province_materials
            )
            .add_systems(OnEnter(ArmyMovementView::On),
                reset_province_materials
            )
            .add_systems(OnExit(ArmyMovementView::On),
                reset_province_materials
            )
            .add_observer(update_claims_panel);
    }
}
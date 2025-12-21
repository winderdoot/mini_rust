use bevy::prelude::*;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum StartupSystems {
    LoadAssets,
    CreateEmpires,
    CreateHexGrid,
    AssignEmpireProvinces,
    InitTurns,
    CreateUI,
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum UpdateSystems {
    Camera,
    UIUpdate,
    OnMessage
}

/* Init Plugin */

pub struct SystemSetsPlugin;

/* Making many system sets even if intended for only a single system, allows
 * registering systems anywhere in code, including other plugins that relate
 * to specific game systems. */
impl Plugin for SystemSetsPlugin {
    fn build(&self, app: &mut App) {
        app
            .configure_sets(Startup, (
                StartupSystems::LoadAssets,
                StartupSystems::CreateEmpires,
                StartupSystems::CreateHexGrid,
                StartupSystems::AssignEmpireProvinces,
                StartupSystems::InitTurns,
                StartupSystems::CreateUI
            ).chain())
            .configure_sets(Update, (
                UpdateSystems::Camera,
                UpdateSystems::UIUpdate,
                UpdateSystems::OnMessage
            ));
    }
}
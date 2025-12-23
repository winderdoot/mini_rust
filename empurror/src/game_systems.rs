use bevy::{platform::collections::HashMap, prelude::*};
use bevy_ecs::system::SystemId;

use crate::ui::panel_update::*;

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

/// Solves the problem of registering/running one shot sytems
#[derive(Resource)]
pub struct GameSystems {
    map: HashMap<String, SystemId>
}

impl GameSystems {
    pub fn new() -> Self {
        Self {
            map: HashMap::new()
        }
    }

    pub fn get(&self, name: &str) -> Option<&SystemId> {
        self.map.get(name)
    }

    pub fn add(mut self, name: &str, id: SystemId) -> Self {
        self.map.insert(name.to_string(), id);
        self
    }
}

/* Init Plugin */

pub struct GameSystemsPlugin;

/* Making many system sets even if intended for only a single system, allows
 * registering systems anywhere in code, including other plugins that relate
 * to specific game systems. */
impl Plugin for GameSystemsPlugin {
    fn build(&self, app: &mut App) {
        // let game_systems = GameSystems::new()
        //     .add(stringify!(update_claim_button), app.register_system(update_claim_button))
        //     .add(stringify!(update_build_house_button), app.register_system(update_build_house_button));

        app
            // .insert_resource(game_systems)
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
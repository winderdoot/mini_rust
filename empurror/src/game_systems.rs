use bevy::{platform::collections::HashMap, prelude::*};
use bevy_ecs::system::SystemId;

use crate::{game_logic::empire::*, scene::mesh_highlight::*};

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum StartupSystems {
    LoadAssets,
    CreateEmpires,
    CreateHexGrid,
    AssignEmpireProvinces,
    InitTurns, /* Here we will have to enable ui, so we mind need a new state  */
    CreateUI,
    CreateAI,
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
        let game_systems = GameSystems::new()
            .add(stringify!(calculate_all_provinces_income), app.register_system(calculate_all_provinces_income))
            .add(stringify!(reset_province_materials), app.register_system(reset_province_materials));

        app
            .insert_resource(game_systems)
            .configure_sets(Startup, (
                StartupSystems::LoadAssets,
                StartupSystems::CreateEmpires,
                StartupSystems::CreateHexGrid,
                StartupSystems::AssignEmpireProvinces,
                StartupSystems::InitTurns,
                StartupSystems::CreateUI,
                StartupSystems::CreateAI
            ).chain())
            .configure_sets(Update, (
                UpdateSystems::Camera,
                UpdateSystems::UIUpdate,
                UpdateSystems::OnMessage
            ));
    }
}
use bevy::prelude::*;

use crate::{game_logic::empire::Empires, game_systems::StartupSystems};

#[derive(Resource)]
pub struct Turns {
    pub completed_rounds: u32,
    pub empire_count: u32, /* Duplicate info from Empires resource */
    pub current_empire: u32
}

/// Used to mark the end of a turn, so that empire actions can be calculated and have proper effect.
/// A NewTurn message is supposed to be emitted afterwards
#[derive(Event, Debug)]
pub struct EndTurn {
    pub empire_id: u32
}

#[derive(Event, Debug)]
pub struct NewTurn {
    pub empire_id: u32
}

/* Systems  */
fn init_turns(
    mut commands: Commands,
    empires: Res<Empires>
) {
    commands.insert_resource(
        Turns {
            completed_rounds: 0,
            empire_count: empires.count,
            current_empire: 0,
        }
    );
}

/* Init Plugin */
/// Main gameplay plugin that sets up most gameplay systems
pub struct TurnGameplayPlugin;

/* Making many system sets even if intended for only a single system, allows
 * registering systems anywhere in code, including other plugins that relate
 * to specific game systems. */
impl Plugin for TurnGameplayPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, init_turns.in_set(StartupSystems::InitTurns));
    }
}
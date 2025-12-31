
use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    MainMenu,
    InGame,
    GameOver
}

// #[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, SubStates)]
// #[source(AppState = AppState::InGame)]
// #[states(scoped_entities)]
// pub enum IsPlayerTurn {
//     #[default]
//     Yes,
//     No
// }

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GridViewMode {
    #[default]
    Terrain,
    Empire,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum ArmyMovementView {
    #[default]
    Off,
    On,
}

// #[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, SubStates)]
// #[source(AppState = AppState::InGame)]
// #[states(scoped_entities)]
// pub enum IsPaused {
//     #[default]
//     Running,
//     Paused,
// }

/* Init Plugin */
pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<AppState>()
            .init_state::<GridViewMode>()
            .init_state::<ArmyMovementView>();
    }
}

/* Systems */



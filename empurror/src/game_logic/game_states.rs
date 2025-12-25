
use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    Menu,
    #[default]
    InGame,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, SubStates)]
#[source(AppState = AppState::InGame)]
#[states(scoped_entities)]
pub enum IsPlayerTurn {
    #[default]
    Yes,
    No
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, SubStates)]
#[source(AppState = AppState::InGame)]
#[states(scoped_entities)]
pub enum GridViewMode {
    #[default]
    Terrain,
    Empire,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, SubStates)]
#[source(AppState = AppState::InGame)]
#[states(scoped_entities)]
pub enum ArmyMovementView {
    #[default]
    Off,
    On,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, SubStates)]
#[source(AppState = AppState::InGame)]
#[states(scoped_entities)]
pub enum IsPaused {
    #[default]
    Running,
    Paused,
}

/* Init Plugin */
pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_state::<AppState>(AppState::InGame)
            .add_sub_state::<IsPlayerTurn>()
            .add_sub_state::<GridViewMode>()
            .add_sub_state::<IsPaused>()
            .add_sub_state::<ArmyMovementView>();
    }
}

/* Systems */



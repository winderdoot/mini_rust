
use bevy::{dev_tools::states::*, prelude::*};
use bevy::ui::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    Menu,
    #[default]
    InGame,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, SubStates)]
#[source(AppState = AppState::InGame)]
#[states(scoped_entities)]
pub enum ViewMode {
    #[default]
    Terrain,
    Empire,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, SubStates)]
#[source(AppState = AppState::InGame)]
#[states(scoped_entities)]
pub enum IsPaused {
    #[default]
    Running,
    Paused,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum CameraState {
    Moving,
    #[default]
    InGame,
}

/* Init Plugin */
pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_state::<AppState>(AppState::InGame)
            .add_sub_state::<ViewMode>()
            .add_sub_state::<IsPaused>();
    }
}

/* Systems */



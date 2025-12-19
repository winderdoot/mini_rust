use bevy::prelude::*;

use crate::game_logic::{empire::claim_province, province::{add_house, add_resource_building}};

pub struct GameEventPlugin;

/* Init Plugin */
impl Plugin for GameEventPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_observer(add_house)
            .add_observer(add_resource_building)
            .add_observer(claim_province);
    }
}
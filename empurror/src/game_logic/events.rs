use bevy::prelude::*;

use crate::game_logic::{empire::*, province::*, armies::*};

pub struct GameEventPlugin;

/* Init Plugin for registering observers */
impl Plugin for GameEventPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_observer(add_house)
            .add_observer(add_special_building)
            .add_observer(claim_province)
            .add_observer(calculate_province_income)
            .add_observer(calculate_empire_resource_net_income)
            .add_observer(calculate_empire_pops_income)
            .add_observer(recruit_soldier)
            .add_observer(create_army)
            .add_observer(disband_army)
            .add_observer(move_army)
            .add_observer(reset_armies_moves)
            .add_observer(update_army_model)
            .add_observer(bankrupt_empire)
            .add_observer(occupy_province);
    }
}
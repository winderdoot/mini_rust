use bevy::prelude::*;

use crate::{game_logic::empire::*, game_systems::{GameSystems, StartupSystems}, ai::core::*};

#[derive(Resource)]
pub struct Turns {
    completed_rounds: u32,
    empire_count: u32, /* Duplicate info from Empires resource */
    current_empire: u32,
}

impl Turns {
    pub fn is_player_turn(&self) -> bool {
        self.current_empire == PLAYER_EMPIRE
    }

    pub fn full_rounds(&self) -> u32 {
        self.completed_rounds
    }
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
    empires: Res<Empires>,
    systems: Res<GameSystems>
) {
    commands.insert_resource(
        Turns {
            completed_rounds: 0,
            empire_count: empires.count,
            current_empire: PLAYER_EMPIRE
        }
    );

    /* We need to calculate the inital upkeeps & incomes for all provinces in each empire */
    let Some(system) = systems.get(stringify!(calculate_all_provinces_income)) else {
        error!("Missing game system");
        return;
    };
    commands.run_system(*system);
    commands.trigger(NewTurn { empire_id: PLAYER_EMPIRE });
}

fn turn_ended(
    event: On<EndTurn>,
    mut turns: ResMut<Turns>,
    mut empires: ResMut<Empires>,
    mut q_empires: Query<&mut Empire>,
    mut commands: Commands
) {
    /* When turn ends, the income is already calculated, However, when turn stars
     * the playing empire needs to recalculate it's income. */
    let Some(empire_ent) = empires.get_entity(event.empire_id) else {
        error!("Missing empire entity");
        return;
    };
    let Ok(mut empire_c) = q_empires.get_mut(*empire_ent) else {
        error!("Missing empire component");
        return;
    };
    empire_c.apply_income();

    if event.empire_id == turns.empire_count - 1 {
        turns.completed_rounds += 1;
        empires.update_peace_time();
    }

    let next_empire = (event.empire_id + 1) % turns.empire_count;
    commands.trigger(NewTurn { empire_id: next_empire });
}

fn turn_started(
    event: On<NewTurn>,
    mut turns: ResMut<Turns>,
    empires: Res<Empires>,
    mut q_empires: Query<(Entity, &mut Empire, Option<&Controls>)>,
    mut commands: Commands
) {
    let Some(empire_ent) = empires.get_entity(event.empire_id) else {
        error!("Missing empire entity");
        return;
    };
    let Ok((empire_e, mut empire_c, controls)) = q_empires.get_mut(*empire_ent) else {
        error!("Missing empire component");
        return;
    };
    if controls.is_none_or(|controls| controls.is_empty()) {
        /* Empire lost control of all provinces - game over for them */
        empire_c.loose();
    }
    
    turns.current_empire = event.empire_id;

    if !empire_c.is_playing() {
        commands.trigger(EndTurn { empire_id: event.empire_id });
        return;
    }
    if empire_c.is_bankrupt() {
        if empire_c.id == PLAYER_EMPIRE {
            /* Game over screen */
            todo!("game over screen");
        }
        else {
            commands.trigger(EmpireBankrupt { empire: empire_e });
            commands.trigger(EndTurn { empire_id: event.empire_id });
        }
        
        return;
    }    

    commands.trigger(ResetEmpireArmies { empire: *empire_ent });
    commands.trigger(ResourceIncomeChanged { empire: *empire_ent });
    commands.trigger(PopsIncomeChanged { empire: *empire_ent });
    
    if empire_c.id == PLAYER_EMPIRE {
        /* Enable UI  */
    }
    else {
        commands.trigger(AIPlayTurn { empire_id: event.empire_id });
        commands.trigger(EndTurn { empire_id: event.empire_id });
    }
}

/* Init Plugin */
pub struct TurnGameplayPlugin;

/* Making many system sets even if intended for only a single system, allows
 * registering systems anywhere in code, including other plugins that relate
 * to specific game systems. */
impl Plugin for TurnGameplayPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_observer(turn_ended)
            .add_observer(turn_started)
            .add_systems(Startup, init_turns.in_set(StartupSystems::InitTurns));
    }
}
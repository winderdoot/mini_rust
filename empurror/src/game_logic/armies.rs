use bevy::{prelude::*, platform::collections::*};

use strum_macros::{Display, EnumIter};
use hexx::{Hex};

use crate::{game_logic::{empire::{Empire, Empires}, province::*, resources::*}, scene::hex_grid::*};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, EnumIter, Display)]
pub enum SoldierType {
    Infantry
}

impl SoldierType {
    pub fn recruit_cost(&self) -> HashMap<ResourceType, f32> {
        match self {
            SoldierType::Infantry => [(ResourceType::Gold, 0.0), (ResourceType::Grain, 1.0)].into(),
        }
    }

    pub fn march_budget(&self) -> u32 {
        match self {
            SoldierType::Infantry => 10,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Soldier {
    pub stype: SoldierType,
    pub home_province: Entity
}

impl Soldier {
    pub fn new_infantry(province: &Entity) -> Self {
        Self {
            stype: SoldierType::Infantry,
            home_province: province.clone()
        }
    }
}

#[derive(Component, Debug)]
pub struct Army {
    atype: SoldierType,
    soldiers: Vec<Soldier>,
    empire: Entity,
    id: u32,
    moved: bool /* If moved this turn */
}

impl Army {
    pub fn new(soldier: Soldier, empire: Entity, id: u32) -> Self {
        Self {
            atype: soldier.stype,
            soldiers: vec![soldier],
            empire,
            id,
            moved: false
        }
    }

    pub fn try_remove_soldiers(&mut self, amount: usize) -> usize {
        let count = self.soldier_count();
        let removed = std::cmp::min(count, amount);
        self.soldiers.truncate(count - removed);
        removed
    }

    pub fn empire(&self) -> Entity {
        self.empire.clone()
    }

    pub fn march_budget(&self) -> u32 {
        if self.soldiers.len() < 10 {
            return self.atype.march_budget();
        }

        return self.atype.march_budget() / 2;
    }

    pub fn soldier_type(&self) -> SoldierType {
        self.atype.clone()
    }

    pub fn try_add_soldier(&mut self, soldier: Soldier) -> bool {
        if soldier.stype != self.atype {
            return false;
        }
        self.soldiers.push(soldier);

        true
    }

    pub fn try_remove_soldier(&mut self) -> Option<Soldier> {
        self.soldiers.pop()
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn soldiers(&self) -> &Vec<Soldier> {
        &self.soldiers
    }

    pub fn soldiers_mut(&mut self) -> &mut Vec<Soldier> {
        &mut self.soldiers
    }
    
    pub fn soldiers_iter(&self) -> impl Iterator<Item = &Soldier> {
        self.soldiers.iter()
    }

    pub fn soldier_count(&self) -> usize {
        self.soldiers.len()
    }

    pub fn set_moved(&mut self) {
        self.moved = true;
    }

    pub fn moved(&self) -> bool {
        self.moved
    }

    pub fn reset_moved(&mut self) {
        self.moved = false;
    }
}

impl std::fmt::Display for Army {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Army {} ({})", self.id, self.atype)
    }
}

#[derive(Component)]
// #[require(M)]
pub struct ArmyModel;

/* Relationships */

/* Source of truth in ArmyProvince <-> ProvinceArmies */
#[derive(Component, Deref)]
#[relationship(relationship_target = ProvinceArmies)]
pub struct ArmyProvince(pub Entity);

#[derive(Component)]
#[relationship_target(relationship = ArmyProvince)]
pub struct ProvinceArmies(Vec<Entity>);

impl ProvinceArmies {
    pub fn armies_mut(&mut self) -> &mut Vec<Entity> {
        &mut self.0
    }

    pub fn count(&self) -> usize {
        self.0.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = Entity> {
        self.0.iter().cloned()
    }

    pub fn armies(&self) -> &Vec<Entity> {
        &self.0
    }
}

/* Events */
#[derive(Event, Debug)]
pub struct ArmyMoved {
    pub army: Entity,
    pub province: Entity
}

/* Systems */

pub fn move_army(
    event: On<ArmyMoved>,
    mut q_armies: Query<(&mut Army, &ArmyProvince)>,
    mut q_provinces: Query<(&mut Province, Option<&ControlledBy>, Option<&ProvinceArmies>)>,
    q_empires: Query<&Empire>,
    empires: Res<Empires>,
    mut commands: Commands
) {
    
    let Ok((army_c, starting_province)) = q_armies.get_mut(event.army) else {
        error!("{}:{} Missing army component", file!(), line!());
        return;
    };
    let starting_prov = starting_province.entity();
    std::mem::drop(starting_province);
    
    /* Check if this tile has armies we should fight first */

    let Ok((province_c, controlled_by, prov_armies_c)) = q_provinces.get_mut(event.province) else {
        error!("{}:{} MIssing province component", file!(), line!());
        return;
    };
    if let Some(controlled_by) = controlled_by && controlled_by.entity() != army_c.empire() {
        let Ok(enter_empire) = q_empires.get(army_c.empire()) else {
            error!("{}:{} Missing empire components", file!(), line!());
            return;
        };
        let Ok(station_empire) = q_empires.get(controlled_by.entity()) else {
            error!("{}:{} Missing empire components", file!(), line!());
            return;
        };

        if !empires.at_war(enter_empire.id, station_empire.id) {
            error!("{}:{} Invalid state: empires are meant to be at war", file!(), line!());
            return;
        }

        let mut attack_count = army_c.soldier_count();
        std::mem::drop(army_c);

        if let Some(stationed_armies_c) = prov_armies_c {
            let stationed_armies = stationed_armies_c
                .armies()
                .iter()
                .cloned()
                .collect::<Vec<Entity>>();

            /* There are stationed armies to defend the province */
            let defeated_armies = stationed_armies
                .iter()
                .filter_map(|army_e| {
                    if attack_count == 0 {
                        return None;
                    }
                    let Ok((mut defending_army, army_prov)) = q_armies.get_mut(army_e.clone()) else {
                        error!("{}:{} Missing army component", file!(), line!());
                        return None;
                    };

                    let removed = defending_army.try_remove_soldiers(attack_count);
                    attack_count -= removed;
                    
                    /* Remove killed soldiers from their home province */
                    (0..removed)
                        .for_each(|i| {
                            let Some(soldier) = defending_army.try_remove_soldier() else {
                                error!("{}:{} BAD!", file!(), line!());
                                return;
                            };

                            let Ok((mut province_c, _, _)) = q_provinces.get_mut(army_prov.entity()) else {
                                error!("{}:{} Missing province component", file!(), line!());
                                return;
                            };
                            province_c.try_remove_pop();
                        });

                    if defending_army.soldier_count() == 0 {
                        return Some(*army_e)
                    }
                    else {
                        None
                    }
                })
                .collect::<Vec<Entity>>();
            
            defeated_armies
                .iter()
                .for_each(|army_e| {
                    commands
                    .entity(*army_e)
                    .despawn();
            });

            let Ok((mut army_c, army_prov)) = q_armies.get_mut(event.army) else {
                error!("{}:{} Missing army component", file!(), line!());
                return;
            };
            let army_count = army_c.soldier_count();
            let killed_attackers = army_count - attack_count;

            (0..killed_attackers)
                .for_each(|i| {
                    let Some(soldier) = army_c.try_remove_soldier() else {
                        error!("{}:{} BAD!", file!(), line!());
                        return;
                    };

                    let Ok((mut province_c, _, _)) = q_provinces.get_mut(army_prov.entity()) else {
                        error!("{}:{} Missing province component", file!(), line!());
                        return;
                    };
                    province_c.try_remove_pop();
                });

            army_c.try_remove_soldiers(killed_attackers);
        
            if army_c.soldier_count() > 0 {
                commands.trigger(ProvinceOcuppied { province: event.province, empire: army_c.empire() });
            }
            else {
                commands
                    .entity(event.army)
                    .despawn();

                commands.trigger(ProvinceIncomeChanged { province: event.province });
                /* TODO: Make sure that when a player army is killed, it cannot be selected anywhere so we dont get panics */
            }
        }
        else {
            let Ok((army_c, _)) = q_armies.get_mut(event.army) else {
                error!("{}:{} Missing army component", file!(), line!());
                return;
            };

            /* No armies present, the province is occupied by the entering army  */
            commands.trigger(ProvinceOcuppied { province: event.province, empire: army_c.empire() });
        }
    }

    let Ok((mut army_c, _)) = q_armies.get_mut(event.army) else {
        error!("{}:{} Missing army component", file!(), line!());
        return;
    };

    if army_c.soldier_count() > 0 {
        army_c.set_moved();
    
        commands
            .entity(event.army)
            .insert(ArmyProvince(event.province));
    }

    commands.trigger(ProvinceArmyChanged { province: starting_prov }); /* Source */
    commands.trigger(ProvinceArmyChanged { province: event.province }); /* Destination */
}

pub fn get_reachable_tiles(
    army_c: &Army,
    province_c: &Province,
    q_provinces: &Query<(&Province, Option<&ControlledBy>)>,
    q_empires: &Query<&Empire>,
    grid: &Res<HexGrid>,
    empires: &Empires,
) -> HashSet<Entity> {

    let budget = army_c.march_budget();
    let const_func = |hex: Hex| {
        let Some(province_e) = grid.get_entity(&hex) else {
            return None;
        };
        let Ok((province_c, controlled_by)) = q_provinces.get(*province_e) else {
            return None;
        };
        if let Some(owner) = controlled_by &&
           let Ok(owner_c) = q_empires.get(owner.entity()) && 
           let Ok(army_empire_c) = q_empires.get(army_c.empire()) {
            
            /* Cannot march into controlled territory unless waging war */
            if owner_c.id != army_empire_c.id && !empires.at_war(owner_c.id, army_empire_c.id) {
                return None;
            }
        }

        return province_c.march_cost();
    };

    let hex_set = hexx::algorithms::field_of_movement(province_c.hex(), budget, const_func);

    return hex_set
        .into_iter()
        .filter_map(|hex| grid.get_entity(&hex))
        .cloned()
        .collect();
}
use bevy::{prelude::*, platform::collections::*};

use strum_macros::{Display, EnumIter};

use crate::game_logic::resources::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, EnumIter, Display)]
pub enum SoldierType {
    Infantry
}

impl SoldierType {
    pub fn recruit_cost(&self) -> HashMap<ResourceType, f32> {
        match self {
            SoldierType::Infantry => [(ResourceType::Gold, 1.0), (ResourceType::Grain, 5.0)].into(),
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
    pub atype: SoldierType,
    pub soldiers: Vec<Soldier>,
    pub empire: Entity,
    pub id: u32,
    pub locked: bool /* If moved this turn */
}

impl Army {
    pub fn new(soldier: Soldier, empire: Entity, id: u32) -> Self {
        Self {
            atype: soldier.stype,
            soldiers: vec![soldier],
            empire,
            id,
            locked: false
        }
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
        return &self.soldiers
    }
    
    pub fn soldiers_iter(&self) -> impl Iterator<Item = &Soldier> {
        return self.soldiers.iter()
    }

    pub fn soldier_count(&self) -> usize {
        self.soldiers.len()
    }

    pub fn moved(&self) -> bool {
        self.locked
    }

    pub fn reset_moved(&mut self) {
        self.locked = false;
    }
}

impl std::fmt::Display for Army {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Army {} ({})", self.id, self.atype)
    }
}

/* Relationships */

/* Source of truth in ArmyProvince <-> ProvinceArmies */
#[derive(Component, Deref)]
#[relationship(relationship_target = ProvinceArmies)]
pub struct ArmyProvince(pub Entity);

#[derive(Component)]
#[relationship_target(relationship = ArmyProvince)]
pub struct ProvinceArmies(Vec<Entity>);

impl ProvinceArmies {
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
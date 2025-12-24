use bevy::{prelude::*, platform::collections::*};

use strum_macros::{Display, EnumIter};

use crate::game_logic::resources::*;

#[derive(Copy, Clone, Debug, Display, PartialEq, Eq, Hash, EnumIter)]
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
    pub moved: bool /* If moved this turn */
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
}

impl std::fmt::Display for Army {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Army {}", self.id)
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
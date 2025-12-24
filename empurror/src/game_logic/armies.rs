use bevy::{prelude::*, platform::collections::*};

use strum_macros::Display;

use crate::game_logic::resources::*;

#[derive(Debug, Display, PartialEq, Eq)]
pub enum SoldierType {
    Infantry
}

#[derive(Component)]
pub struct Soldier {
    pub stype: SoldierType,
    pub home_province: Entity
}

impl SoldierType {
    pub fn recruit_cost(&self) -> HashMap<ResourceType, f32> {
        match self {
            SoldierType::Infantry => [(ResourceType::Gold, 1.0), (ResourceType::Grain, 5.0)].into(),
        }
    }
}

#[derive(Component)]
pub struct Army {
    pub atype: SoldierType,
    pub soldiers: Vec<Soldier>
}

/* Relationships */

/* Source of truth in ArmyProvince <-> ProvinceArmies */
#[derive(Component, Deref)]
#[relationship(relationship_target = ProvinceArmies)]
pub struct ArmyProvince(pub Entity);

#[derive(Component)]
#[relationship_target(relationship = ArmyProvince)]
pub struct ProvinceArmies(Vec<Entity>);


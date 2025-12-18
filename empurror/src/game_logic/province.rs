use bevy::prelude::*;
use std::cmp::Eq;

use crate::game_logic::empire::Controls;
use crate::scene::mesh_highlight::*;

#[derive(Hash, Debug, PartialEq, Eq, Clone)]
pub enum ProvinceType {
    Water,
    BlackSoil,
    Plains,
    Woods,
    Desert,   
    Hills,
    Mountains,
}

/* Source of truth in the ControlledBy <-> Controls relationship */
#[derive(Component, Deref)]
#[relationship(relationship_target = Controls)]
pub struct ControlledBy(pub Entity);

#[derive(Component)]
#[require(Highlightable)]
pub struct Province {
    pub prov_type: ProvinceType,
}

impl Province {
    pub fn from_type(t: &ProvinceType) -> Self {
        Self {
            prov_type: t.clone()
        }
    }
}

/* Source of truth in LocatedIn <-> ProvinceBuildings  */
#[derive(Component, Deref)]
#[relationship(relationship_target = ProvinceBuildings)]
pub struct LocatedIn(pub Entity);

#[derive(Component)]
#[relationship_target(relationship = LocatedIn)]
pub struct ProvinceBuildings(Vec<Entity>);


/* Buildings */
#[derive(Component, Default)]
pub struct Building;

#[derive(Component)]
#[require(Building)]
pub struct House {
    pub population: u32,
    pub max_population: u32
}

#[derive(Event, Debug)]
pub struct HouseAdded {
    pub province: Entity
}

/* Systems */
pub fn add_house(
    event: On<HouseAdded>
) {
    
}


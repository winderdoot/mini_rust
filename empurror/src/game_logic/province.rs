use bevy::prelude::*;
use std::cmp::Eq;

use crate::game_logic::empire::*;
use crate::scene::entity_picking::*;

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

use bevy::prelude::*;
use bevy::ecs::entity::EntityHashSet;
use bevy::ecs::system::RunSystemOnce;

use crate::game_logic::empire::*;

#[derive(Hash, Debug, PartialEq, std::cmp::Eq)]
pub enum ProvinceType {
    Water,
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
pub struct Province {
    pub prov_type: ProvinceType,
}

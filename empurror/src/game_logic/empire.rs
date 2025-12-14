use bevy::prelude::*;
use crate::game_logic::province::*;

#[derive(Component, Deref)]
#[relationship_target(relationship = ControlledBy)]
pub struct Controls(Vec<Entity>);


#[derive(Component)]
pub struct Empire;

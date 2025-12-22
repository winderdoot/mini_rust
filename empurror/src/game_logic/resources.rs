use bevy::prelude::*;
use std::cmp;

#[derive(Clone, Copy, Debug, PartialEq, cmp::Eq, Hash)]
pub enum ResourceType {
    Grain,
    Lumber,
    Stone,
    Gold,
}

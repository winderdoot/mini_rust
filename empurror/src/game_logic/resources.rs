use bevy::prelude::*;
use strum_macros::EnumIter;
use std::{cmp, fmt};

#[derive(Clone, Copy, Debug, PartialEq, cmp::Eq, Hash, EnumIter)]
pub enum ResourceType {
    Grain,
    Lumber,
    Stone,
    Gold,
}

impl fmt::Display for ResourceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
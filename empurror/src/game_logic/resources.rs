use bevy::{platform::collections::HashMap, prelude::*};
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

pub fn resource_string(map: &HashMap<ResourceType, f32>) -> String {
    map
        .iter()
        .map(|(k, v)| {
            format!("{}: {} ", k, *v)
        })
        .collect::<Vec<String>>()
        .join("")
}
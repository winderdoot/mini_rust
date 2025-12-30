use bevy::{platform::collections::HashMap, prelude::*};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use std::{cmp, fmt};

use crate::game_logic::empire::resource_amount;

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

pub fn add_resources(a: &HashMap<ResourceType, f32>, b: &HashMap<ResourceType, f32>) -> HashMap<ResourceType, f32> {
    ResourceType::iter()
        .map(|k| {
            let total = resource_amount(a, &k) + resource_amount(b, &k);

            (k, total)
        })
        .collect()
}
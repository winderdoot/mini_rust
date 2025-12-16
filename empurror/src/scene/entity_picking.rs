use bevy::{prelude::*};

#[derive(Component)]
pub struct Highlightable {
    pub highlighted: bool
}

impl Default for Highlightable {
    fn default() -> Self {
        Self { highlighted: false }
    }
}
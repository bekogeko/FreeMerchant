use bevy::prelude::{Component, Entity};

#[derive(Clone, Component, Debug, Eq, Hash, PartialEq)]
pub struct City {
    pub name: String,
}

#[derive(Component)]
pub struct Player {
    pub position: Entity,
}

impl City {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}

use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Default, Resource)]
pub struct Lobby {
    pub entities: HashMap<Entity, Entity>,
}

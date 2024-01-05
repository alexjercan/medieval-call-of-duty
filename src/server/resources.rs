use bevy::prelude::*;
use bevy_renet::renet::ClientId;
use std::collections::HashMap;

#[derive(Debug, Default, Resource)]
pub struct Lobby {
    pub players: HashMap<ClientId, Entity>,
    pub characters: HashMap<ClientId, Entity>,
}

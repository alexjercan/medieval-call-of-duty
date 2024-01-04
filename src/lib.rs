use bevy::prelude::*;
use std::collections::HashMap;
use bevy_renet::renet::ClientId;
use serde::{Serialize, Deserialize};

pub const PROTOCOL_ID: u64 = 0;

#[derive(Debug, Default, Resource)]
pub struct Lobby {
    pub players: HashMap<ClientId, Entity>,
}

#[derive(Debug, Serialize, Deserialize, Component)]
pub enum ServerMessages {
    PlayerConnected { id: ClientId },
    PlayerDisconnected { id: ClientId },
}

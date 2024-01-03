use bevy::prelude::*;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use bevy_renet::renet::ClientId;

pub const PROTOCOL_ID: u64 = 0;

#[derive(Debug, Default, Resource)]
pub struct Lobby {
    players: HashMap<ClientId, Entity>,
}

#[derive(Debug, Serialize, Deserialize, Component)]
enum ServerMessages {
    PlayerConnected { id: ClientId },
    PlayerDisconnected { id: ClientId },
}

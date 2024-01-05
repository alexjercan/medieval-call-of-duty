use bevy::prelude::*;
use bevy_renet::renet::ClientId;

#[derive(Debug, Component)]
pub struct Player {
    pub id: ClientId,
}

#[derive(Debug, Component)]
pub struct Character;

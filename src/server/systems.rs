use crate::{EntityType, ServerChannel, ServerMessage};

use super::components::*;
use super::resources::*;
use bevy::prelude::*;
use bevy_renet::renet::{RenetServer, ServerEvent};

// TODO: Create the world using rapier
pub fn setup() {}

pub fn handle_server_events(
    mut commands: Commands,
    mut events: EventReader<ServerEvent>,
    mut lobby: ResMut<Lobby>,
    mut server: ResMut<RenetServer>,
) {
    for event in events.read() {
        match event {
            ServerEvent::ClientConnected { client_id } => {
                debug!("Client connected: {}", client_id);

                let player_entity = commands.spawn(Player { id: *client_id }).id();

                for (id, _) in lobby.players.iter() {
                    let message =
                        bincode::serialize(&ServerMessage::PlayerConnected { id: *id }).unwrap();
                    server.send_message(*client_id, ServerChannel::ServerMessage, message);
                }

                lobby.players.insert(*client_id, player_entity);

                let message =
                    bincode::serialize(&ServerMessage::PlayerConnected { id: *client_id }).unwrap();
                server.broadcast_message(ServerChannel::ServerMessage, message);
            }
            ServerEvent::ClientDisconnected { client_id, reason } => {
                debug!("Client disconnected: {} ({})", client_id, reason);

                if let Some(player_entity) = lobby.players.remove(client_id) {
                    commands.entity(player_entity).despawn();
                }

                let message =
                    bincode::serialize(&ServerMessage::PlayerDisconnected { id: *client_id })
                        .unwrap();
                server.broadcast_message(ServerChannel::ServerMessage, message);
            }
        }
    }
}

pub fn handle_spawn_players(
    mut commands: Commands,
    mut lobby: ResMut<Lobby>,
    mut server: ResMut<RenetServer>,
    player_query: Query<&Player>,
) {
    for player in player_query.iter() {
        if !lobby.characters.contains_key(&player.id) {
            // TODO: Add controller, health, etc.
            let character_entity = commands.spawn(Character).id();

            lobby.characters.insert(player.id, character_entity);

            // TODO: Send initial state (probably random transform)
            let message = bincode::serialize(&ServerMessage::EntityCreate {
                entity_type: EntityType::Character,
                entity: character_entity,
                translation: [0.0, 0.0, 0.0],
            })
            .unwrap();
            server.broadcast_message(ServerChannel::ServerMessage, message);
        }
    }
}

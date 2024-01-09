use super::resources::*;
use crate::{ClientChannel, ClientMessage, ServerChannel, ServerMessage};
use bevy::{gltf::*, prelude::*};
use bevy_rapier3d::prelude::*;
use bevy_renet::renet::{RenetServer, ServerEvent};
use std::collections::hash_map::Entry::Vacant;

pub fn setup(
    mut commands: Commands,
    world: ResMut<WorldAssets>,
    gltf_assets: Res<Assets<Gltf>>,
    gltf_mesh_assets: Res<Assets<GltfMesh>>,
    gltf_node_assets: Res<Assets<GltfNode>>,
    mesh_assets: Res<Assets<Mesh>>,
) {
    if let Some(gltf) = gltf_assets.get(&world.playground) {
        for node in &gltf.nodes {
            let node = gltf_node_assets.get(node).unwrap();
            if let Some(gltf_mesh) = node.mesh.clone() {
                let gltf_mesh = gltf_mesh_assets.get(&gltf_mesh).unwrap();
                for mesh_primitive in &gltf_mesh.primitives {
                    let mesh = mesh_assets.get(&mesh_primitive.mesh).unwrap();
                    commands.spawn((
                        Collider::from_bevy_mesh(mesh, &ComputedColliderShape::TriMesh).unwrap(),
                        RigidBody::Fixed,
                        TransformBundle::from_transform(node.transform),
                    ));
                }
            }
        }
    }
}

pub fn handle_server_events(mut events: EventReader<ServerEvent>, mut server: ResMut<RenetServer>) {
    for event in events.read() {
        match event {
            ServerEvent::ClientConnected { client_id } => {
                println!("Client connected: {}", client_id);

                let message =
                    bincode::serialize(&ServerMessage::PlayerConnected { id: *client_id }).unwrap();
                server.broadcast_message(ServerChannel::ServerMessage, message);
            }
            ServerEvent::ClientDisconnected { client_id, reason } => {
                println!("Client disconnected: {} ({})", client_id, reason);

                let message =
                    bincode::serialize(&ServerMessage::PlayerDisconnected { id: *client_id })
                        .unwrap();
                server.broadcast_message(ServerChannel::ServerMessage, message);
            }
        }
    }
}

pub fn handle_client_messages(
    mut commands: Commands,
    mut lobby: ResMut<ServerLobby>,
    mut server: ResMut<RenetServer>,
) {
    for client_id in server.clients_id() {
        while let Some(message) = server.receive_message(client_id, ClientChannel::ClientMessage) {
            match bincode::deserialize(&message).unwrap() {
                ClientMessage::SpawnMe => {
                    if let Vacant(entry) = lobby.players.entry(client_id) {
                        println!("Spawning player for client {}", client_id);

                        let position = Vec3::new(0.0, 1.0, 0.0);

                        let entity = commands
                            .spawn((Transform::from_translation(position),))
                            .id();
                        entry.insert(entity);

                        let message = bincode::serialize(&ServerMessage::SpawnPlayer {
                            server_entity: entity,
                            position,
                        })
                        .unwrap();
                        server.broadcast_message_except(
                            client_id,
                            ServerChannel::ServerMessage,
                            message,
                        );

                        let message = bincode::serialize(&ServerMessage::SpawnHim {
                            server_entity: entity,
                            position,
                        })
                        .unwrap();
                        server.send_message(client_id, ServerChannel::ServerMessage, message);
                    }
                }
            }
        }
    }
}

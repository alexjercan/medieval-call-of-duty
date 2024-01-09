use super::resources::*;
use crate::{ClientChannel, ClientMessage, ServerChannel, ServerMessage};
use bevy::{gltf::*, prelude::*};
use bevy_rapier3d::prelude::*;
use bevy_renet::renet::RenetClient;

pub fn setup(
    mut commands: Commands,
    world: ResMut<WorldAssets>,
    gltf_assets: Res<Assets<Gltf>>,
    gltf_mesh_assets: Res<Assets<GltfMesh>>,
    gltf_node_assets: Res<Assets<GltfNode>>,
    mesh_assets: Res<Assets<Mesh>>,
) {
    if let Some(gltf) = gltf_assets.get(&world.playground) {
        let scene = gltf.scenes.first().unwrap().clone();

        commands.spawn(SceneBundle { scene, ..default() });

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

pub fn initial_spawn(mut client: ResMut<RenetClient>) {
    let message = bincode::serialize(&ClientMessage::SpawnMe).unwrap();
    client.send_message(ClientChannel::ClientMessage, message);
}

pub fn handle_server_messages(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut client: ResMut<RenetClient>,
) {
    while let Some(message) = client.receive_message(ServerChannel::ServerMessage) {
        let server_message = bincode::deserialize(&message).unwrap();
        match server_message {
            ServerMessage::PlayerConnected { id } => {
                println!("Player {} connected.", id);
            }
            ServerMessage::PlayerDisconnected { id } => {
                println!("Player {} disconnected.", id);
            }
            ServerMessage::SpawnPlayer {
                server_entity,
                position,
            } => {
                println!("Spawning player at {:?}", position);
            }
            ServerMessage::SpawnHim {
                server_entity,
                position,
            } => {
                println!("Spawning him at {:?}", position);
            }
        }
    }
}

use crate::{EntityType, ServerChannel, ServerMessage};

use super::components::*;
use super::resources::*;
use bevy::{prelude::*, render::mesh::shape::Plane};
use bevy_renet::renet::RenetClient;

/// set up a simple 3D scene
pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(Plane::from_size(5.0))),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..Default::default()
    });
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}

pub fn handle_server_messages(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut client: ResMut<RenetClient>,
    mut lobby: ResMut<Lobby>,
) {
    while let Some(message) = client.receive_message(ServerChannel::ServerMessage) {
        let server_message = bincode::deserialize(&message).unwrap();
        match server_message {
            ServerMessage::PlayerConnected { id } => {
                debug!("Player {} connected.", id);
            }
            ServerMessage::PlayerDisconnected { id } => {
                println!("Player {} disconnected.", id);
            }
            ServerMessage::EntityCreate {
                entity_type: EntityType::Character,
                entity,
                translation,
            } => {
                debug!("Character {:?} created.", entity);

                let character_entity = commands
                    .spawn((
                        NetworkEntity { id: entity },
                        PbrBundle {
                            mesh: meshes.add(Mesh::from(shape::Capsule::default())),
                            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                            transform: Transform::from_xyz(
                                translation[0],
                                translation[1],
                                translation[2],
                            ),
                            ..Default::default()
                        },
                    ))
                    .id();

                lobby.entities.insert(entity, character_entity);
            }
            ServerMessage::EntityRemove { entity } => {
                debug!("Entity {:?} removed.", entity);

                if let Some(entity) = lobby.entities.remove(&entity) {
                    commands.entity(entity).despawn();
                }
            }
        }
    }
}

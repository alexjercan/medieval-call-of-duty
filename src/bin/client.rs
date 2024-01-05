use bevy::{prelude::*, render::mesh::shape::Plane};
use bevy_renet::{
    renet::{
        transport::{ClientAuthentication, NetcodeClientTransport},
        RenetClient,
    },
    transport::NetcodeClientPlugin,
    RenetClientPlugin,
};
use log::debug;
use medieval_call_of_duty::{
    connection_config, Character, Lobby, Player, ServerChannel, ServerMessages, PROTOCOL_ID,
};
use std::{net::UdpSocket, time::SystemTime};

fn main() {
    let server_addr = "127.0.0.1:5000".parse().unwrap();
    let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    let client_id = current_time.as_millis() as u64;
    let authentication = ClientAuthentication::Unsecure {
        client_id,
        protocol_id: PROTOCOL_ID,
        server_addr,
        user_data: None,
    };

    let transport = NetcodeClientTransport::new(current_time, authentication, socket).unwrap();
    let client = RenetClient::new(connection_config());

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RenetClientPlugin)
        .add_plugins(NetcodeClientPlugin)
        .init_resource::<Lobby>()
        .insert_resource(client)
        .insert_resource(transport)
        .add_systems(Startup, setup)
        .add_systems(Update, handle_server_messages)
        .run();
}

/// set up a simple 3D scene
fn setup(
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

fn handle_server_messages(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut client: ResMut<RenetClient>,
    mut lobby: ResMut<Lobby>,
) {
    while let Some(message) = client.receive_message(ServerChannel::ServerMessages) {
        let server_message = bincode::deserialize(&message).unwrap();
        match server_message {
            ServerMessages::PlayerConnected { id } => {
                debug!("Player {} connected.", id);

                let player_entity = commands.spawn(Player { id }).id();

                lobby.players.insert(id, player_entity);
            }
            ServerMessages::PlayerDisconnected { id } => {
                println!("Player {} disconnected.", id);

                if let Some(player_entity) = lobby.players.remove(&id) {
                    commands.entity(player_entity).despawn();
                }
            }
            ServerMessages::PlayerCreate {
                entity: _,
                id,
                translation,
            } => {
                debug!("Player {} created.", id);

                let character_entity = commands
                    .spawn((
                        Character { id },
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

                lobby.characters.insert(id, character_entity);
            }
            ServerMessages::PlayerRemove { id } => {
                debug!("Player {} removed.", id);

                if let Some(character_entity) = lobby.characters.remove(&id) {
                    commands.entity(character_entity).despawn();
                }
            }
        }
    }
}

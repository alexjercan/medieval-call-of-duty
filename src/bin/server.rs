use medieval_call_of_duty::{Lobby, PROTOCOL_ID, ServerMessages};
use std::{net::UdpSocket, time::SystemTime};
use bevy::{prelude::*, app::ScheduleRunnerPlugin, utils::Duration};
use bevy_renet::{RenetServerPlugin, renet::{RenetServer, ConnectionConfig, transport::{NetcodeServerTransport, ServerConfig, ServerAuthentication}, ServerEvent, ClientId, DefaultChannel}, transport::NetcodeServerPlugin};

#[derive(Debug, Component)]
struct Player {
    id: ClientId,
}

fn main() {
    let public_addr = "127.0.0.1:5000".parse().unwrap();
    let socket = UdpSocket::bind(public_addr).unwrap();
    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let server_config = ServerConfig {
        current_time,
        max_clients: 64,
        protocol_id: PROTOCOL_ID,
        public_addresses: vec![public_addr],
        authentication: ServerAuthentication::Unsecure,
    };

    let transport = NetcodeServerTransport::new(server_config, socket).unwrap();
    let server = RenetServer::new(ConnectionConfig::default());

    App::new()
        .add_plugins(
            MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(
                1.0 / 60.0,
            ))),
        )
        .add_plugins(RenetServerPlugin)
        .add_plugins(NetcodeServerPlugin)
        .init_resource::<Lobby>()
        .insert_resource(server)
        .insert_resource(transport)
        .add_systems(Startup, setup)
        .add_systems(Update, handle_server_events)
        .run();
}

fn setup(mut commands: Commands) {

}

fn handle_server_events(
    mut commands: Commands,
    mut events: EventReader<ServerEvent>,
    mut lobby: ResMut<Lobby>,
    mut server: ResMut<RenetServer>
) {
    for event in events.read() {
        match event {
            ServerEvent::ClientConnected { client_id } => {
                println!("Client connected: {}", client_id);

                let player_entity = commands.spawn(Player { id: *client_id }).id();

                for (id, _) in lobby.players.iter() {
                    let message = bincode::serialize(&ServerMessages::PlayerConnected { id: *id }).unwrap();
                    server.send_message(*client_id, DefaultChannel::ReliableOrdered, message);
                }

                lobby.players.insert(*client_id, player_entity);

                let message = bincode::serialize(&ServerMessages::PlayerConnected { id: *client_id }).unwrap();
                server.broadcast_message(DefaultChannel::ReliableOrdered, message);
            }
            ServerEvent::ClientDisconnected { client_id, reason } => {
                println!("Client disconnected: {} ({})", client_id, reason);

                if let Some(player_entity) = lobby.players.remove(client_id) {
                    commands.entity(player_entity).despawn();
                }

                let message = bincode::serialize(&ServerMessages::PlayerDisconnected { id: *client_id }).unwrap();
                server.broadcast_message(DefaultChannel::ReliableOrdered, message);
            }
        }
    }
}

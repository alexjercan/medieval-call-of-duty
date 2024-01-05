use bevy::{app::ScheduleRunnerPlugin, prelude::*, utils::Duration};
use bevy_renet::{
    renet::{
        transport::{NetcodeServerTransport, ServerAuthentication, ServerConfig},
        RenetServer, ServerEvent,
    },
    transport::NetcodeServerPlugin,
    RenetServerPlugin,
};
use log::debug;
use medieval_call_of_duty::{
    connection_config, Character, Lobby, Player, ServerChannel, ServerMessages, PROTOCOL_ID,
};
use std::{net::UdpSocket, time::SystemTime};

fn main() {
    let public_addr = "127.0.0.1:5000".parse().unwrap();
    let socket = UdpSocket::bind(public_addr).unwrap();
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    let server_config = ServerConfig {
        current_time,
        max_clients: 64,
        protocol_id: PROTOCOL_ID,
        public_addresses: vec![public_addr],
        authentication: ServerAuthentication::Unsecure,
    };

    let transport = NetcodeServerTransport::new(server_config, socket).unwrap();
    let server = RenetServer::new(connection_config());

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
        .add_systems(Update, (handle_server_events, handle_spawn_players))
        .run();
}

fn setup() {}

fn handle_server_events(
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
                        bincode::serialize(&ServerMessages::PlayerConnected { id: *id }).unwrap();
                    server.send_message(*client_id, ServerChannel::ServerMessages, message);
                }

                lobby.players.insert(*client_id, player_entity);

                let message =
                    bincode::serialize(&ServerMessages::PlayerConnected { id: *client_id })
                        .unwrap();
                server.broadcast_message(ServerChannel::ServerMessages, message);
            }
            ServerEvent::ClientDisconnected { client_id, reason } => {
                debug!("Client disconnected: {} ({})", client_id, reason);

                if let Some(player_entity) = lobby.players.remove(client_id) {
                    commands.entity(player_entity).despawn();
                }

                let message =
                    bincode::serialize(&ServerMessages::PlayerDisconnected { id: *client_id })
                        .unwrap();
                server.broadcast_message(ServerChannel::ServerMessages, message);
            }
        }
    }
}

fn handle_spawn_players(
    mut commands: Commands,
    mut lobby: ResMut<Lobby>,
    mut server: ResMut<RenetServer>,
    player_query: Query<&Player>,
) {
    for player in player_query.iter() {
        if !lobby.characters.contains_key(&player.id) {
            let character_entity = commands.spawn(Character { id: player.id }).id();

            lobby.characters.insert(player.id, character_entity);

            let message = bincode::serialize(&ServerMessages::PlayerCreate {
                entity: character_entity,
                id: player.id,
                translation: [0.0, 0.0, 0.0],
            })
            .unwrap();
            server.broadcast_message(ServerChannel::ServerMessages, message);
        }
    }
}

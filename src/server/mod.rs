mod components;
mod resources;
mod systems;

use resources::*;
use systems::*;

use bevy::{app::ScheduleRunnerPlugin, prelude::*};
use bevy_renet::{
    renet::{
        transport::{NetcodeServerTransport, ServerAuthentication, ServerConfig},
        RenetServer,
    },
    transport::NetcodeServerPlugin,
    RenetServerPlugin,
};
use std::{
    net::UdpSocket,
    time::{Duration, SystemTime},
};

use crate::{connection_config, PROTOCOL_ID};

// TODO: Parameterize this with: max_clients, etc.
pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
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

        app.add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(
            Duration::from_secs_f64(1.0 / 60.0),
        )))
        .add_plugins(RenetServerPlugin)
        .add_plugins(NetcodeServerPlugin)
        .init_resource::<Lobby>()
        .insert_resource(server)
        .insert_resource(transport)
        .add_systems(Startup, setup)
        .add_systems(Update, (handle_server_events, handle_spawn_players));
    }
}

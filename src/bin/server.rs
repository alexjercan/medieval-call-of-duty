use medieval_call_of_duty::{Lobby, PROTOCOL_ID};
use std::{net::UdpSocket, time::SystemTime};
use bevy::{prelude::*, app::ScheduleRunnerPlugin, utils::Duration};
use bevy_renet::{RenetServerPlugin, renet::{RenetServer, ConnectionConfig, transport::{NetcodeServerTransport, ServerConfig, ServerAuthentication}}, transport::NetcodeServerPlugin};

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
        .run();
}

fn setup(mut commands: Commands) {
}

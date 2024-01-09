mod components;
mod resources;
mod systems;

use bevy::{app::ScheduleRunnerPlugin, prelude::*, winit::WinitPlugin};
use bevy_asset_loader::prelude::*;
use bevy_renet::{
    renet::{
        transport::{NetcodeServerTransport, ServerAuthentication, ServerConfig},
        RenetServer,
    },
    transport::NetcodeServerPlugin,
    RenetServerPlugin,
};
use resources::*;
use std::{
    net::UdpSocket,
    time::{Duration, SystemTime},
};
use systems::*;

use crate::{connection_config, PROTOCOL_ID};

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum ServerStates {
    #[default]
    AssetLoading,
    Playing,
}

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

        app.add_plugins(DefaultPlugins.build().disable::<WinitPlugin>())
            .add_plugins(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(
                1.0 / 60.0,
            )))
            .add_plugins(RenetServerPlugin)
            .add_plugins(NetcodeServerPlugin)
            .add_state::<ServerStates>()
            .add_loading_state(
                LoadingState::new(ServerStates::AssetLoading)
                    .continue_to_state(ServerStates::Playing)
                    .load_collection::<WorldAssets>(),
            )
            .init_resource::<ServerLobby>()
            .insert_resource(server)
            .insert_resource(transport)
            .add_systems(OnEnter(ServerStates::Playing), setup)
            .add_systems(
                Update,
                (handle_server_events, handle_client_messages)
                    .run_if(in_state(ServerStates::Playing)),
            );
    }
}

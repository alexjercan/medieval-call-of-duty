mod components;
mod resources;
mod systems;

use resources::*;
use systems::*;

use bevy::prelude::*;
use bevy_renet::{
    renet::{
        transport::{ClientAuthentication, NetcodeClientTransport},
        RenetClient
    },
    transport::NetcodeClientPlugin,
    RenetClientPlugin,
};
use std::{net::UdpSocket, time::SystemTime};

use crate::{PROTOCOL_ID, connection_config};

// TODO: Parameterize this with: ip, etc.
pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
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

        app
            .add_plugins(DefaultPlugins)
            .add_plugins(RenetClientPlugin)
            .add_plugins(NetcodeClientPlugin)
            .init_resource::<Lobby>()
            .insert_resource(client)
            .insert_resource(transport)
            .add_systems(Startup, setup)
            .add_systems(Update, handle_server_messages);
    }
}

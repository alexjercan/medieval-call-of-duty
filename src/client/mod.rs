mod components;
mod resources;
mod systems;

use crate::{connection_config, PROTOCOL_ID};
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use crate::controller::FpsControllerPlugin;
use bevy_rapier3d::prelude::*;
use bevy_renet::{
    renet::{
        transport::{ClientAuthentication, NetcodeClientTransport},
        RenetClient,
    },
    transport::NetcodeClientPlugin,
    RenetClientPlugin,
};
use resources::*;
use std::{net::UdpSocket, time::SystemTime};
use systems::*;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum ClientStates {
    #[default]
    AssetLoading,
    Playing,
}

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

        app.add_plugins(DefaultPlugins)
            .add_plugins(RenetClientPlugin)
            .add_plugins(NetcodeClientPlugin)
            .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
            .add_plugins(FpsControllerPlugin)
            .add_state::<ClientStates>()
            .add_loading_state(
                LoadingState::new(ClientStates::AssetLoading)
                    .continue_to_state(ClientStates::Playing)
                    .load_collection::<WorldAssets>(),
            )
            .insert_resource(AmbientLight {
                color: Color::WHITE,
                brightness: 0.5,
            })
            .insert_resource(client)
            .insert_resource(transport)
            .insert_resource(RapierConfiguration::default())
            .add_systems(OnEnter(ClientStates::Playing), (setup, initial_spawn))
            .add_systems(
                Update,
                (handle_server_messages).run_if(in_state(ClientStates::Playing)),
            );
    }
}

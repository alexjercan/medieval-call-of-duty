pub mod client;
pub mod server;

use bevy::prelude::*;
use bevy_renet::renet::{ChannelConfig, ClientId, ConnectionConfig, SendType};
use serde::{Deserialize, Serialize};
use std::time::Duration;

pub const PROTOCOL_ID: u64 = 0;

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize, Component, Resource)]
pub struct PlayerMoveInput {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
}

pub enum ClientChannel {
    MoveInput,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EntityType {
    Character,
}

#[derive(Debug, Clone, Serialize, Deserialize, Component)]
pub enum ServerMessage {
    PlayerConnected {
        id: ClientId,
    },
    PlayerDisconnected {
        id: ClientId,
    },
}

pub enum ServerChannel {
    ServerMessage,
}

impl From<ClientChannel> for u8 {
    fn from(channel_id: ClientChannel) -> Self {
        match channel_id {
            ClientChannel::MoveInput => 0,
        }
    }
}

impl ClientChannel {
    pub fn channels_config() -> Vec<ChannelConfig> {
        vec![ChannelConfig {
            channel_id: Self::MoveInput.into(),
            max_memory_usage_bytes: 5 * 1024 * 1024,
            send_type: SendType::ReliableOrdered {
                resend_time: Duration::ZERO,
            },
        }]
    }
}

impl From<ServerChannel> for u8 {
    fn from(channel_id: ServerChannel) -> Self {
        match channel_id {
            ServerChannel::ServerMessage => 0,
        }
    }
}

impl ServerChannel {
    pub fn channels_config() -> Vec<ChannelConfig> {
        vec![ChannelConfig {
            channel_id: Self::ServerMessage.into(),
            max_memory_usage_bytes: 10 * 1024 * 1024,
            send_type: SendType::ReliableOrdered {
                resend_time: Duration::from_millis(200),
            },
        }]
    }
}

pub fn connection_config() -> ConnectionConfig {
    ConnectionConfig {
        available_bytes_per_tick: 1024 * 1024,
        client_channels_config: ClientChannel::channels_config(),
        server_channels_config: ServerChannel::channels_config(),
    }
}

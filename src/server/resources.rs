use bevy::{prelude::*, gltf::*};
use std::collections::HashMap;
use bevy_asset_loader::prelude::*;
use bevy_renet::renet::ClientId;

#[derive(AssetCollection, Resource)]
pub struct WorldAssets {
    #[asset(path = "playground.glb")]
    pub playground: Handle<Gltf>,
}

#[derive(Debug, Default, Resource)]
pub struct ServerLobby {
    pub players: HashMap<ClientId, Entity>,
}

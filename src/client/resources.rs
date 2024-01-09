use bevy::{prelude::*, gltf::*};
use bevy_asset_loader::prelude::*;
use bevy_renet::renet::ClientId;

#[derive(AssetCollection, Resource)]
pub struct WorldAssets {
    #[asset(path = "playground.glb")]
    pub playground: Handle<Gltf>,
}

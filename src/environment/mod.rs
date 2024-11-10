mod base;
mod chunk_handler;

use bevy::gltf::GltfNode;
use bevy::prelude::*;
use crate::environment::base::EnvironmentBase;
use crate::environment::chunk_handler::ChunkHandlerPlugin;

#[derive(Component, Resource, Reflect, Debug)]
#[reflect(Component)]
pub struct Chunk {
    pub node: Handle<GltfNode>,
    pub x: i32,
    pub z: i32,
    pub loaded: bool,
    pub area: String,
    pub name: String,
    pub player_inbound: bool
}

pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((EnvironmentBase, ChunkHandlerPlugin));
    }
}
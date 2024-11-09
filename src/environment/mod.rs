mod base;
mod chunk_handler;

use bevy::prelude::*;
use crate::environment::base::EnvironmentBase;
use crate::environment::chunk_handler::ChunkHandlerPlugin;

pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((EnvironmentBase, ChunkHandlerPlugin));
    }
}
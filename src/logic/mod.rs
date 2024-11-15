mod loading_handler;

use bevy::prelude::*;
use crate::logic::loading_handler::LoadingHandlerPlugin;

pub struct LogicPlugin;

impl Plugin for LogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LoadingHandlerPlugin);
    }
}
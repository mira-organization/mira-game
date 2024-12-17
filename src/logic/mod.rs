mod loading;

use bevy::prelude::*;
use crate::logic::loading::LoadingPlugin;

pub struct LogicPlugin;

impl Plugin for LogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LoadingPlugin);
    }
}
mod base;

use bevy::prelude::*;
use crate::environment::base::EnvironmentBase;

pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EnvironmentBase);
    }
}
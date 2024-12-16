use bevy::prelude::*;
use crate::environment::test_room::TestRoomPlugin;

mod test_room;

pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TestRoomPlugin);
    }
}
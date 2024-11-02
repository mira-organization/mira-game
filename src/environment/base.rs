use bevy::prelude::*;
use crate::manager::{AppState, EnvironmentSets, InGameState};

pub struct EnvironmentBase;

impl Plugin for  EnvironmentBase {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame(InGameState::Playing)), initialize_map.in_set(EnvironmentSets));
    }
}

#[allow(dead_code, unused)] //Todo: remove if this method used all of his parameters
fn initialize_map(mut commands: Commands) {
    info!("Initializing map");
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::log::LogPlugin;
    use bevy::app::App;

    #[test]
    fn test_initialize_map() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, LogPlugin::default()));

        app.add_systems(Startup, initialize_map);
        app.update();
    }
}
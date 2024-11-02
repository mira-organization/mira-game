use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::plugin::{NoUserData, RapierPhysicsPlugin};
use bevy_rapier3d::prelude::{DebugRenderStyle, RapierDebugRenderPlugin};
use bevy_xpbd_3d::plugins::PhysicsPlugins;
use crate::entities::EntitiesPlugin;
use crate::environment::EnvironmentPlugin;
use crate::logic::LogicPlugin;

#[derive(Component, States, Debug, Clone, PartialEq, Eq, Hash)]
#[allow(dead_code)] //Todo: remove if this enum is completely in use.
pub enum AppState {
    SplashScreen,
    WaitingScreen,
    MainMenu(MainMenuState),
    InGame(InGameState),
    Quit
}

impl Default for AppState {
    fn default() -> Self {
        AppState::InGame(InGameState::default())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[allow(dead_code)] //Todo: remove if this enum is completely in use.
pub enum MainMenuState {
    Main,
    Settings,
}

impl Default for MainMenuState {
    fn default() -> Self {
        MainMenuState::Main
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[allow(dead_code)] //Todo: remove if this enum is completely in use.
pub enum InGameState {
    Playing,
    InUi,
    MapOpen,
}

impl Default for InGameState {
    fn default() -> Self {
        InGameState::Playing
    }
}

#[derive(SystemSet, Debug, Clone, Hash, PartialEq, Eq)]
pub struct AudioSets;

#[derive(SystemSet, Debug, Clone, Hash, PartialEq, Eq)]
pub struct UiSets;

#[derive(SystemSet, Debug, Clone, Hash, PartialEq, Eq)]
pub struct EntitySets;

#[derive(SystemSet, Debug, Clone, Hash, PartialEq, Eq)]
pub struct EnvironmentSets;

#[derive(SystemSet, Debug, Clone, Hash, PartialEq, Eq)]
pub struct AiSets;

#[derive(SystemSet, Debug, Clone, Hash, PartialEq, Eq)]
pub struct PlayerSets;

pub struct ManagerPlugin;

impl Plugin for ManagerPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>();

        app.add_plugins(WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::F1)));

        app.add_plugins(PhysicsPlugins::default())
            .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
            .add_plugins(plugin_init_rapier3d_debug());

        app.add_plugins((
            EnvironmentPlugin,
            LogicPlugin,
            EntitiesPlugin
        ));
    }
}

fn plugin_init_rapier3d_debug() -> RapierDebugRenderPlugin {
    RapierDebugRenderPlugin {
        enabled: false,
        style: DebugRenderStyle {
            collider_parentless_color: [0.0, 1.0, 1.0, 1.0],
            collider_dynamic_color: [305.0, 1.0, 0.5, 1.0],
            collider_fixed_color: [65.0, 1.0, 0.5, 1.0],
            collider_kinematic_color: [140.0, 1.0, 0.5, 1.0],
            sleep_color_multiplier: [0.0, 0.5, 0.5, 1.0],
            ..default()
        },
        ..default()
    }
}
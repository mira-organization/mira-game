use bevy::app::App;
use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::{DebugRenderStyle, NoUserData, RapierDebugRenderPlugin, RapierPhysicsPlugin};
use crate::entities::EntitiesPlugin;
use crate::environment::EnvironmentPlugin;
use crate::logic::LogicPlugin;

#[derive(Component, States, Default, Debug, Clone, PartialEq, Eq, Hash)]
#[allow(dead_code)]
pub enum AssetState {
    #[default]
    Loading,
    Ready
}

#[derive(Component, States, Debug, Clone, PartialEq, Eq, Hash)]
#[allow(dead_code)]
pub enum AppState {
    SplashScreen,
    TitleScreen,
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
#[allow(dead_code)]
pub enum MainMenuState {
    MainMenu,
    SettingsMenu,
    AccountMenu,
}

impl Default for MainMenuState {
    fn default() -> Self {
        MainMenuState::MainMenu
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[allow(dead_code)]
pub enum InGameState {
    Playing,
    InventoryOpen,
    MapOpen
}

impl Default for InGameState {
    fn default() -> Self {
        InGameState::Playing
    }
}

pub struct ManagerPlugin;

impl Plugin for ManagerPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>();
        app.init_state::<AssetState>();

        app.enable_state_scoped_entities::<AssetState>();

        app.add_plugins(WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::F3)));

        app
            .add_plugins((
                RapierPhysicsPlugin::<NoUserData>::default(),
                plugin_init_rapier3d_debug()
            ));

        app.add_plugins((
            LogicPlugin,
            EntitiesPlugin,
            EnvironmentPlugin
        ));
    }
}

fn plugin_init_rapier3d_debug() -> RapierDebugRenderPlugin {
    RapierDebugRenderPlugin {
        enabled: true,
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
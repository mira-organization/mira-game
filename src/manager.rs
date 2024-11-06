use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::plugin::{NoUserData, RapierPhysicsPlugin};
use bevy_rapier3d::prelude::{DebugRenderStyle, RapierDebugRenderPlugin};
use bevy_xpbd_3d::plugins::PhysicsPlugins;
use crate::entities::EntitiesPlugin;
use crate::environment::EnvironmentPlugin;
use crate::logic::LogicPlugin;

/// Enum for the states. states have his own internal states for handle in
/// bound states like [`MainMenu`] handle [`MainMenuState`] if there was called.
/// This enum is called at [`App::init_state`]
#[derive(Component, States, Debug, Clone, PartialEq, Eq, Hash)]
#[allow(dead_code)] //Todo: remove if this enum is completely in use.
pub enum AppState {
    SplashScreen,
    WaitingScreen,
    MainMenu(MainMenuState),
    InGame(InGameState),
    Quit
}

/// Load the default initialize value for [`AppState`].
impl Default for AppState {
    fn default() -> Self {
        AppState::InGame(InGameState::default())
    }
}

/// This is the control enum for [`AppState::MainMenu`]. This is only
/// called at the main enum and is needed for handle inner states.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[allow(dead_code)] //Todo: remove if this enum is completely in use.
pub enum MainMenuState {
    Main,
    Settings,
}

/// Load the default initialize vale for [`MainMenuState`].
impl Default for MainMenuState {
    fn default() -> Self {
        MainMenuState::Main
    }
}

/// This is the control enum for [`AppState::InGame`]. This is only
/// called at the main enum and is needed for handle inner states.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[allow(dead_code)] //Todo: remove if this enum is completely in use.
pub enum InGameState {
    Playing,
    InUi,
    MapOpen,
}

/// Load the default initialize vale for [`InGameState`].
impl Default for InGameState {
    fn default() -> Self {
        InGameState::Playing
    }
}

/// [`SystemSet`] for handle audio systems and put them to a set list.
#[derive(SystemSet, Debug, Clone, Hash, PartialEq, Eq)]
pub struct AudioSets;

/// [`SystemSet`] for handle ui systems and put them to a set list.
#[derive(SystemSet, Debug, Clone, Hash, PartialEq, Eq)]
pub struct UiSets;

/// [`SystemSet`] for handle entity systems and put them to a set list.
#[derive(SystemSet, Debug, Clone, Hash, PartialEq, Eq)]
pub struct EntitySets;

/// [`SystemSet`] for handle environment systems and put them to a set list.
#[derive(SystemSet, Debug, Clone, Hash, PartialEq, Eq)]
pub struct EnvironmentSets;

/// [`SystemSet`] for handle AI systems and put them to a set list.
#[derive(SystemSet, Debug, Clone, Hash, PartialEq, Eq)]
pub struct AiSets;

/// [`SystemSet`] for handle player systems and put them to a set list.
/// This has his own system because it needs to be seperated from entities.
#[derive(SystemSet, Debug, Clone, Hash, PartialEq, Eq)]
pub struct PlayerSets;

/// This Plugin is using [`Plugin`] from bevy for handle at runtime.
/// The [`ManagerPlugin`] is used for help the main class. All the
/// Game used code will be called her first. The main called only [`App::add_plugins`] function
/// and stored [`ManagerPlugin`].
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
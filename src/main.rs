mod manager;
mod entities;
mod environment;
mod logic;

use bevy::prelude::*;
use bevy::window::{WindowPlugin, Window, WindowResolution};
use bevy::render::settings::{WgpuSettings, Backends, WgpuFeatures, RenderCreation};
use bevy::app::AppExit;
use bevy::render::RenderPlugin;
use crate::manager::ManagerPlugin;

/// Main function
fn main() -> AppExit {
    let mut app = App::new();
    initialize_app(&mut app).run()
}

/// Function initialized the main game loop and set up [`manager`].
/// All the game logic can be found at [`manager`] or his sub
/// packages like [`entities`], [`logic`] or [`environment`].
fn initialize_app(app: &mut App) -> &mut App {
    app
        .add_plugins(DefaultPlugins.set(
            WindowPlugin {
                primary_window: Some(Window {
                    title: "Mira | Development State 0.1.0-alpha".to_string(),
                    resolution: WindowResolution::new(1270.0, 720.0),
                    ..default()
                }),
                ..default()
            }
        ).set(
            RenderPlugin {
                render_creation: RenderCreation::Automatic(create_gpu_settings()),
                ..default()
            }
        )).add_plugins(ManagerPlugin)
}

/// Function was out sourced for Unit testing and inclusion for
/// [`WgpuSettings`].
fn create_gpu_settings() -> WgpuSettings {
    WgpuSettings {
        features: WgpuFeatures::POLYGON_MODE_LINE,
        backends: Some(Backends::VULKAN),
        ..default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Unit Test for check if [`Backends::VULKAN`] enabled.
    #[test]
    fn test_app_uses_vulkan_backend() {
        let settings = create_gpu_settings();

        assert_eq!(settings.backends, Some(Backends::VULKAN));
        assert!(settings.features.contains(WgpuFeatures::POLYGON_MODE_LINE));
    }
}



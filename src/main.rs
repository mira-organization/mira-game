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

fn main() -> AppExit {
    let mut app = App::new();
    initialize_app(&mut app).run()
}

fn initialize_app(app: &mut App) -> &mut App {
    app
        .add_plugins(DefaultPlugins.set(
            WindowPlugin {
                primary_window: Some(Window {
                    title: "Mira | Development State 0.1.0-alpha".to_string(),
                    resolution: WindowResolution::new(1920.0, 1080.0),
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

    #[test]
    fn test_app_uses_vulkan_backend() {
        let settings = create_gpu_settings();

        assert_eq!(settings.backends, Some(Backends::VULKAN));
        assert!(settings.features.contains(WgpuFeatures::POLYGON_MODE_LINE));
    }
}



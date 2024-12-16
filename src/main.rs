mod manager;
mod entities;
mod environment;

use bevy::prelude::*;
use bevy::render::render_resource::WgpuFeatures;
use bevy::render::RenderPlugin;
use bevy::render::settings::{Backends, RenderCreation, WgpuSettings};
use bevy::window::WindowResolution;
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
                    title: "Mira Game - 0.15 bevy".to_string(),
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
    fn check_app_used_vulkan_backend() {
        let settings = create_gpu_settings();
        assert_eq!(settings.backends, Some(Backends::VULKAN));
        assert!(settings.features.contains(WgpuFeatures::POLYGON_MODE_LINE));
    }
}
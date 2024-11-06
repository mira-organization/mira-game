use crate::manager::{AppState, EnvironmentSets, InGameState};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct EnvironmentBase;

impl Plugin for  EnvironmentBase {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame(InGameState::Playing)), initialize_map.in_set(EnvironmentSets));
    }
}

/// Initialize default base [`Plane3d`] for ground detection.
/// This is deprecated because the chunk system will replace them.
fn initialize_map(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    info!("Initializing map");
    commands.spawn((
        Name::new("Terrain"),
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(10.0, 10.0)),
            material: materials.add(Color::srgb_u8(100, 100, 100)),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        RigidBody::Fixed,
        Collider::cuboid(5.0, 0.1, 5.0)
    ));
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::app::App;
    use bevy::ecs::world::EntityRef;
    use bevy::log::LogPlugin;

    #[test]
    fn test_initialize_map() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, LogPlugin::default()));

        app.init_resource::<Assets<Mesh>>();
        app.init_resource::<Assets<StandardMaterial>>();

        app.add_systems(Startup, initialize_map);
        app.update();

        let terrain_entity = app.world_mut().query::<EntityRef>()
            .iter(&app.world())
            .find(|entity| entity.get::<Name>().map_or(false, |name| name.as_str() == "Terrain"))
            .expect("Terrain entity not found");

        assert!(terrain_entity.get::<Name>().is_some());
        assert!(terrain_entity.get::<RigidBody>().is_some());
        assert!(terrain_entity.get::<Collider>().is_some());
    }
}
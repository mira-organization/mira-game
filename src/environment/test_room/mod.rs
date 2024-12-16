use bevy::prelude::*;
use bevy_rapier3d::prelude::{Collider, RigidBody};

pub struct TestRoomPlugin;

impl Plugin for TestRoomPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_flat_test_area);
    }
}

fn create_flat_test_area(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    commands.spawn((
        Name::new("Test Flat"),
        Mesh3d(meshes.add(Plane3d::mesh(&Default::default()).size(200.0, 200.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.3, 0.3))),
        Transform::from_xyz(0.0, -40.0, 0.0),
        GlobalTransform::default(),
        RigidBody::Fixed,
        Collider::cuboid(100.0, 0.1, 100.0),
    ));
}
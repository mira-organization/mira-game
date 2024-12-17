use bevy::app::App;
use bevy::core_pipeline::bloom::{Bloom};
use bevy::prelude::*;
use bevy_atmosphere::prelude::{AtmosphereCamera, AtmospherePlugin};
use bevy_rapier3d::prelude::{Collider, Damping, LockedAxes, RigidBody, Velocity};

use bevy_third_person_camera::{Offset, ThirdPersonCamera, ThirdPersonCameraPlugin, ThirdPersonCameraTarget, Zoom};
use crate::entities::player::{Grounded, PlayerSkillAbleStats, PlayerStats};

pub struct PlayerBasePlugin;

impl Plugin for PlayerBasePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ThirdPersonCameraPlugin,
            AtmospherePlugin
        ));

        app.add_systems(Startup, (
            load_player_model,
            load_player_camera
        ));
    }
}

fn load_player_model(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Name::new("Player"),
        SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("entities/player.glb"))),
        Transform::from_xyz(0.0, 0.0, 0.0),
        GlobalTransform::default(),
        PlayerStats::default(),
        PlayerSkillAbleStats::default(),
        ThirdPersonCameraTarget,
        RigidBody::Dynamic,
        Collider::cuboid(0.5, 0.5, 0.5),
        Velocity::default(),
        Grounded(true),
        Damping {
            linear_damping: 0.2,
            angular_damping: 0.5
        },
        LockedAxes::ROTATION_LOCKED_X | LockedAxes::ROTATION_LOCKED_Z
    ));
}

fn load_player_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("PlayerCamera"),
        Camera3d::default(),
        Transform::from_xyz(0.0, 5.0, 10.0),
        GlobalTransform::default(),
        ThirdPersonCamera {
            sensitivity: Vec2::new(1.0, 1.0),
            zoom: Zoom::new(3.5, 12.75),
            cursor_lock_key: KeyCode::Escape,
            offset: Offset::new(0.0, 0.8),
            offset_enabled: true,
            ..default()
        },
        AtmosphereCamera::default(),
        Bloom::default(),
        DistanceFog {
            color: Color::srgb(0.3, 0.3, 0.3),
            falloff: FogFalloff::Linear {
                start: 600.0,
                end: 700.0
            },
            ..default()
        }
    ));
}

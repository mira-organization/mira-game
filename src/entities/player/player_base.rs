use bevy::core_pipeline::bloom::BloomSettings;
use bevy::core_pipeline::experimental::taa::TemporalAntiAliasBundle;
use bevy::prelude::*;
use bevy_atmosphere::plugin::{AtmosphereCamera, AtmospherePlugin};
use bevy_rapier3d::prelude::*;
use bevy_third_person_camera::*;
use crate::entities::player::{Player, PlayerSkillAbleStats};
use crate::entities::player::player_input::Grounded;
use crate::manager::PlayerSets;

pub struct PlayerBasePlugin;

impl Plugin for PlayerBasePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((ThirdPersonCameraPlugin, AtmospherePlugin));

        app.add_systems(Startup, (load_player_model, load_player_camera).in_set(PlayerSets));
    }
}

fn load_player_model(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    commands.spawn((
        Name::new("Player"),
        PbrBundle {
            mesh: meshes.add(Cuboid::from_size(Vec3::splat(1.0))),
            material: materials.add(Color::srgb_u8(200, 0, 0)),
            transform: Transform::from_xyz(0.0, 0.1, 0.0),
            ..default()
        },
        Player::default(),
        PlayerSkillAbleStats::default(),
        ThirdPersonCameraTarget,
        RigidBody::Dynamic,
        Collider::cuboid(0.5, 0.5, 0.5),
        Velocity::default(),
        Grounded(true),
        Damping {
            linear_damping: 0.2,
            angular_damping: 0.5, // stop random rotating.
        },
        LockedAxes::ROTATION_LOCKED_X | LockedAxes::ROTATION_LOCKED_Z,
    ));
}

fn load_player_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("PlayerCamera"),
        Camera3dBundle {
            transform: Transform::from_xyz(-7.1, 6.8, 22.2).looking_at(Vec3::ZERO, Vec3::Y),
            camera: Camera {
                hdr: true,
                ..default()
            },
            ..default()
        },
        ThirdPersonCamera {
            sensitivity: Vec2::new(2.8, 2.8),
            zoom: Zoom::new(4.5, 30.0),
            cursor_lock_key: KeyCode::Escape,
            offset: Offset::new(0.0, 0.8,),
            offset_enabled: true,
            ..default()
        },
        TemporalAntiAliasBundle::default(),
        BloomSettings::default(),
        AtmosphereCamera::default(),
/*        FogSettings {
            color: Color::srgb(0.25, 0.25, 0.30),
            falloff: FogFalloff::Linear {
                start: 25.0,
                end: 50.0,
            },
            ..default()
        },*/
    ));
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::ecs::world::EntityRef;
    use bevy::app::App;

    #[test]
    fn test_load_player_model() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);

        app.init_resource::<Assets<Mesh>>();
        app.init_resource::<Assets<StandardMaterial>>();

        app.add_systems(Startup, load_player_model);
        app.update();

        let player_entity = app.world_mut().query::<EntityRef>()
            .iter(&app.world())
            .find(|entity| entity.get::<Name>().map_or(false, |name| name.as_str() == "Player"))
            .expect("Player entity not found");

        assert!(player_entity.get::<Name>().is_some());
        assert!(player_entity.get::<Player>().is_some());
        assert!(player_entity.get::<PlayerSkillAbleStats>().is_some());
        assert!(player_entity.get::<ThirdPersonCameraTarget>().is_some());
        assert!(player_entity.get::<RigidBody>().is_some());
        assert!(player_entity.get::<Collider>().is_some());
        assert!(player_entity.get::<Velocity>().is_some());
        assert!(player_entity.get::<Grounded>().is_some());
        assert!(player_entity.get::<Damping>().is_some());
        assert!(player_entity.get::<LockedAxes>().is_some());

        let damping = player_entity.get::<Damping>().unwrap();
        assert_eq!(damping.linear_damping, 0.2);
        assert_eq!(damping.angular_damping, 0.5);

        let locked_axes = player_entity.get::<LockedAxes>().unwrap();
        assert!(locked_axes.contains(LockedAxes::ROTATION_LOCKED_X));
        assert!(locked_axes.contains(LockedAxes::ROTATION_LOCKED_Z));
    }

    #[test]
    fn test_load_player_camera() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);

        app.add_systems(Startup, load_player_camera);
        app.update();

        let camera_entity = app.world_mut().query::<EntityRef>()
            .iter(&app.world())
            .find(|entity| entity.get::<Name>().map_or(false, |name| name.as_str() == "PlayerCamera"))
            .expect("PlayerCamera entity not found");

        assert!(camera_entity.get::<Name>().is_some());
        assert!(camera_entity.get::<ThirdPersonCamera>().is_some());
        assert!(camera_entity.get::<BloomSettings>().is_some());
        assert!(camera_entity.get::<AtmosphereCamera>().is_some());
        assert!(camera_entity.get::<FogSettings>().is_some());

        let fog = camera_entity.get::<FogSettings>().unwrap();
        if let FogFalloff::Linear { start, end } = fog.falloff {
            assert_eq!(start, 25.0);
            assert_eq!(end, 50.0);
        } else {
            panic!("Unexpected FogFalloff type");
        }
    }
}
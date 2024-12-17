use bevy::prelude::*;
use bevy_rapier3d::prelude::{QueryFilter, RapierContext, Real, Velocity};
use crate::entities::player::{Grounded, PlayerState, PlayerStats};

#[derive(Event)]
pub enum InputAction {
    Idle,
    Move(Vec3),
    Sprinting(Vec3),
    Dodge,
    Jump
}

pub struct PlayerInputPlugin;

impl Plugin for PlayerInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<InputAction>();

        app.add_systems(Update, (
            fetch_keyboard_input,
            update_movement,
            ground_check
        ));
    }
}

fn fetch_keyboard_input(mut input_event_writer: EventWriter<InputAction>,
                        keyboard: Res<ButtonInput<KeyCode>>,
                        camera_query: Query<&Transform, (With<Camera>, Without<PlayerStats>)>,
                        mut player_query: Query<(&mut PlayerStats, &Grounded), With<PlayerStats>>,
                        time: Res<Time>,
) {
    for (mut player, grounded) in player_query.iter_mut() {
        if let Ok(cam_transform) = camera_query.get_single() {
            let forward_key = KeyCode::KeyW;
            let backward_key = KeyCode::KeyS;
            let left_key = KeyCode::KeyA;
            let right_key = KeyCode::KeyD;

            let sprinting_key = KeyCode::Space;
            let dodge_key = KeyCode::Space;
            let jump_key = KeyCode::KeyF;

            let mut dodge_alone_used = true;
            if dodge_key == sprinting_key {
                dodge_alone_used = false;
            }

            let mut direction = Vec3::ZERO;
            if keyboard.pressed(forward_key) {
                direction += Vec3::new(cam_transform.forward().x, direction.y, cam_transform.forward().z);
            }

            if keyboard.pressed(backward_key) {
                direction += Vec3::new(cam_transform.back().x, direction.y, cam_transform.back().z);
            }

            if keyboard.pressed(left_key) {
                direction += cam_transform.left().as_vec3();
            }
            if keyboard.pressed(right_key) {
                direction += cam_transform.right().as_vec3();
            }

            if direction.length_squared() > 0.0 {
                let normalized_direction = direction.normalize();
                if keyboard.pressed(left_key) || keyboard.pressed(right_key) ||
                    keyboard.pressed(forward_key) || keyboard.pressed(backward_key) {
                    if player.state != PlayerState::Dodging && player.state != PlayerState::Jumping {
                        input_event_writer.send(InputAction::Move(normalized_direction));
                    }
                } else {
                    if player.state != PlayerState::Dodging && player.state != PlayerState::Jumping {
                        input_event_writer.send(InputAction::Idle);
                    }
                }
            } else {
                if player.state != PlayerState::Dodging && player.state != PlayerState::Jumping {
                    input_event_writer.send(InputAction::Idle);
                }
            }

            if keyboard.just_pressed(sprinting_key) {
                if player.state == PlayerState::Dodging {
                    return;
                }
                player.timers.sprint_timer = 0.0;
            } else if keyboard.pressed(sprinting_key) {
                player.timers.sprint_timer += time.delta_secs();

                if player.timers.sprint_timer > 0.6 {
                    input_event_writer.send(InputAction::Sprinting(direction.normalize()));
                }
            } else if keyboard.just_released(sprinting_key) {
                if player.timers.sprint_timer <= 0.2 && !dodge_alone_used {
                    input_event_writer.send(InputAction::Dodge);
                }
                player.timers.sprint_timer = 0.0;
            }

            if dodge_alone_used {
                if keyboard.just_pressed(dodge_key) {
                    input_event_writer.send(InputAction::Dodge);
                    player.timers.sprint_timer = 0.0;
                }
            }

            if keyboard.just_pressed(jump_key) && player.state != PlayerState::Jumping && grounded.0 {
                input_event_writer.send(InputAction::Jump);
            }
        }
    }
}

fn update_movement(time: Res<Time>,
                   mut input_event_reader: EventReader<InputAction>,
                   mut player_query: Query<(&mut Transform, &mut Velocity, &mut PlayerStats, &mut Grounded)>
) {
    for event in input_event_reader.read() {
        for (mut transform, mut velocity, mut player, mut grounded) in player_query.iter_mut() {
            match event {
                InputAction::Move(direction) => {
                    if direction.length_squared() > 0.0 {
                        let flat_direction = Vec3::new(direction.x, 0.0, direction.z).normalize();
                        let target_rotation = Quat::from_rotation_arc(-Vec3::Z, flat_direction);
                        transform.rotation = transform.rotation.slerp(target_rotation, 0.1);
                        let movement_speed = (player.base.speed * 100.0) * time.delta_secs();
                        velocity.linvel = Vec3::new(flat_direction.x * movement_speed, velocity.linvel.y, flat_direction.z * movement_speed);
                        player.state = PlayerState::Moving;
                        velocity.angvel = Vec3::ZERO;
                    }
                }

                InputAction::Sprinting(direction) => {
                    if player.base.current_stats.stamina < 1.0 {
                        player.state = PlayerState::Moving;
                        player.timers.sprint_timer = 0.0;
                        return;
                    }
                    if direction.length_squared() > 0.0 {
                        let flat_direction = Vec3::new(direction.x, 0.0, direction.z).normalize();
                        let current_forward = transform.forward().as_vec3();

                        if current_forward.dot(flat_direction) < 0.99 {
                            let target_rotation = Quat::from_rotation_arc(-Vec3::Z, flat_direction);
                            transform.rotation = transform.rotation.slerp(target_rotation, 0.1);
                        }

                        let movement_speed = ((player.base.speed * player.speed_sprinting_multiplier) * 100.0) * time.delta_secs();
                        velocity.linvel = Vec3::new(flat_direction.x * movement_speed, velocity.linvel.y, flat_direction.z * movement_speed);
                        if player.state != PlayerState::Jumping {
                            player.state = PlayerState::Sprinting;
                        }
                        velocity.angvel = Vec3::ZERO;
                    }
                }

                InputAction::Dodge => {
                    if !grounded.0 {
                        return;
                    }
                    if player.base.current_stats.stamina < 1.0 {
                        return;
                    }
                    let dodge_distance = 2.0;
                    let dodge_duration = 0.2;
                    let dodge_direction = transform.forward().as_vec3().normalize();
                    let dodge_speed = dodge_distance / dodge_duration;

                    velocity.linvel = Vec3::new(dodge_direction.x * dodge_speed, velocity.linvel.y, dodge_direction.z * dodge_speed);
                    player.state = PlayerState::Dodging;
                    velocity.angvel = Vec3::ZERO;
                }

                InputAction::Jump => {
                    if player.base.current_stats.stamina < 1.0 {
                        return;
                    }

                    if grounded.0 && player.state != PlayerState::Dodging {
                        velocity.linvel.y = player.base.jump_height * 1.4;
                        player.state = PlayerState::Jumping;
                        grounded.0 = false;
                    }
                }

                InputAction::Idle => {
                    player.state = PlayerState::Idling;
                    velocity.linvel = Vec3::new(0.0, velocity.linvel.y, 0.0);
                    velocity.angvel = Vec3::ZERO;
                }
            }
        }
    }
}

fn ground_check(mut player_query: Query<(&mut Transform, &mut PlayerStats, &mut Grounded), With<PlayerStats>>,
                rapier_context: Query<&RapierContext>
) {
    for (transform, mut player, mut grounded) in player_query.iter_mut() {
        let ray_start = transform.translation + Vec3::new(0.0, 0.15, 0.0);
        if let Ok(context) = rapier_context.get_single() {
            if let Some((_entity, _toi)) = context.cast_ray(
                ray_start, Vec3::NEG_Y, 0.2 as Real, true, QueryFilter::only_fixed()
            ) {
                grounded.0 = true;
                if player.state == PlayerState::Jumping {
                    player.state = PlayerState::Idling;
                }
            } else {
                grounded.0 = false;
            }
        }
    }
}


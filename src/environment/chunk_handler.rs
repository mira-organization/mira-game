use std::collections::HashMap;
use bevy::asset::LoadState;
use bevy::gltf::{GltfMesh, GltfNode};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::entities::player::Player;
use crate::environment::{Chunk};

#[derive(Component, Resource, Debug, Default)]
pub struct ChunkManager {
    pub chunk_entries: HashMap<(i32, i32), Chunk>,
}

#[derive(Resource)]
pub struct SceneHandleResource {
    pub handle: Handle<Gltf>,
}

pub struct ChunkHandlerPlugin;

impl Plugin for ChunkHandlerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ChunkManager::default());
        app.add_systems(Startup,
            load_save_config_area_file);

        app.add_systems(Update, (extract_chunks_from_current_areas, load_chunks, unload_chunks).after(load_save_config_area_file));
    }
}

// Todo: include assets loading system
fn load_save_config_area_file(mut commands: Commands,
                                    asset_server: Res<AssetServer>
) {
    let scene_area_handle= asset_server.load("maps/debug.glb");
    commands.insert_resource(SceneHandleResource{handle: scene_area_handle.clone()});

    info!("Load scene config area from {:?}", scene_area_handle);
}

fn extract_chunks_from_current_areas(asset_server: Res<AssetServer>,
                                     scene_handle: Res<SceneHandleResource>,
                                     glb_handle: Res<Assets<Gltf>>,
                                     node_handle: Res<Assets<GltfNode>>,
                                     mut chunk_manager: ResMut<ChunkManager>,
) {

    let load_state = asset_server.get_load_state(&scene_handle.handle);
    if load_state != Option::from(LoadState::Loaded) {
        return;
    }

    if let Some(gltf) = glb_handle.get(&scene_handle.handle) {
        for (name, handle) in gltf.named_nodes.iter() {
            if name.contains("chunk") {
                if let Some(node) = node_handle.get(&*handle) {
                    for child in node.children.iter() {
                        if child.name.contains("terrain") {
                            let x = child.transform.translation.x as i32;
                            let z = child.transform.translation.z as i32;

                            if !chunk_manager.chunk_entries.contains_key(&(x, z)) {
                                chunk_manager.chunk_entries.insert((x, z), Chunk {
                                    id: None,
                                    node: handle.clone(),
                                    x,
                                    z,
                                    size: child.transform.scale.x as i32 * 2,
                                    loaded: false,
                                    area: "debug".to_string(),
                                    name: name.to_string(),
                                    player_inbound: false,
                                });

                                info!("Insert new Chunk - {:?} - {}", name, chunk_manager.chunk_entries.len());
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Visibility: 3 chunks = rendered, 2 = visibility ready, all others destroy. Todo: make function split.
fn load_chunks(mut commands: Commands,
               player_query: Query<&Transform, With<Player>>,
               node_handle: Res<Assets<GltfNode>>,
               mesh_handle: Res<Assets<GltfMesh>>,
               meshes: ResMut<Assets<Mesh>>,
               mut chunk_manager: ResMut<ChunkManager>,
               mut visibility_query: Query<(&mut Visibility, Option<&mut ColliderDisabled>)>,
) {
    if let Ok(transform) = player_query.get_single() {
        let visible_chunks = get_visible_chunks(&transform, 512);

        for chunk_key in visible_chunks.iter() {
            let key = (chunk_key.0, chunk_key.1);
            if let Some(chunk) = chunk_manager.chunk_entries.get_mut(&key) {
                if chunk.loaded {
                    continue;
                }

                if let Some(node) = node_handle.get(&chunk.node) {
                    for child in node.children.iter() {
                        if child.name.contains("terrain") {
                            if let Some(mesh_option) = &child.mesh {
                                if let Some(mesh) = mesh_handle.get(&*mesh_option) {
                                    load_single_chunk(&mut commands, chunk, &meshes, child, mesh, &mut visibility_query);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn unload_chunks(mut commands: Commands,
                 player_query: Query<&Transform, With<Player>>,
                 mut chunk_manager: ResMut<ChunkManager>,
                 mut visibility_query: Query<(&mut Visibility, Option<&mut ColliderDisabled>)>,
) {
    let unload_distance = 500.0;
        if let Ok(transform) = player_query.get_single() {
            let position = transform.translation;

            for chunk in chunk_manager.chunk_entries.values_mut() {
                let chunk_position = Vec3::new(chunk.x as f32, position.y, chunk.z as f32);
                let distance_to_chunk = position.distance(chunk_position);

                if distance_to_chunk > unload_distance && chunk.loaded {
                    if let Some(entity) = chunk.id {
                        if let Ok((mut visibility, collider_disable)) = visibility_query.get_mut(entity) {
                            *visibility = Visibility::Hidden;
                            if collider_disable.is_some() {
                                commands.entity(entity).insert(ColliderDisabled);
                            }
                        }
                    }
                    chunk.loaded = false;
                    info!("Unload {:?}", chunk.name);
                }
            }

            //Todo: Destroy system if to far.
        }
}

fn get_visible_chunks(player_transform: &Transform, size: i32) -> Vec<(i32, i32)> {
    let mut visible_chunks = Vec::new();

    let player_position = player_transform.translation;
    let chunk_size = size;
    let view_distance = 500.0; //Todo Change this

    let min_chunk_x = (player_position.x - view_distance).floor() / chunk_size as f32;
    let max_chunk_x = (player_position.x + view_distance).ceil() / chunk_size as f32;
    let min_chunk_z = (player_position.z - view_distance).floor() / chunk_size as f32;
    let max_chunk_z = (player_position.z + view_distance).ceil() / chunk_size as f32;

    for x in min_chunk_x as i32..=max_chunk_x as i32 {
        for z in min_chunk_z as i32..=max_chunk_z as i32 {
            let chunk_x = x * chunk_size;
            let chunk_z = z * chunk_size;

            let distance_to_chunk = player_position.distance(Vec3::new(chunk_x as f32, player_position.y, chunk_z as f32));
            if distance_to_chunk < view_distance {
                visible_chunks.push((chunk_x, chunk_z));
            }
        }
    }

    visible_chunks
}

fn load_single_chunk(commands: &mut Commands,
                     chunk: &mut Chunk,
                     meshes: &ResMut<Assets<Mesh>>,
                     child: &GltfNode,
                     mesh: &GltfMesh,
                     visibility_query: &mut Query<(&mut Visibility, Option<&mut ColliderDisabled>)>
) {
    if let Some(material) = &mesh.primitives[0].material {
        let bevy_mesh = mesh.primitives[0].mesh.clone();

        if let Some(col_mesh) = meshes.get(&bevy_mesh) {
            if let Some(collider) = Collider::from_bevy_mesh(col_mesh, &ComputedColliderShape::TriMesh) {
                if chunk.id.is_none() {
                    let entity_id = commands.spawn((
                        Name::new(chunk.name.clone()),
                        PbrBundle {
                            mesh: bevy_mesh,
                            transform: Transform {
                                translation: child.transform.translation,
                                scale: child.transform.scale,
                                ..default()
                            },
                            visibility: Visibility::Visible,
                            material: material.clone(),
                            ..default()
                        },
                        RigidBody::Fixed,
                        collider,
                    )).id();

                    chunk.id = Option::from(entity_id);
                    chunk.loaded = true;
                    info!("Loaded {:?}", chunk.name);
                } else {
                    if let Some(entity) = chunk.id {
                        if let Ok((mut visibility, collider_disable)) = visibility_query.get_mut(entity) {
                            *visibility = Visibility::Visible;
                            if collider_disable.is_some() {
                                commands.entity(entity).remove::<ColliderDisabled>();
                            }
                        }
                    }
                    chunk.loaded = true;
                    info!("Loaded {:?}", chunk.name);
                }
            }
        }
    }
}

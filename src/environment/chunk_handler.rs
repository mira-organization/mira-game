use std::collections::HashMap;
use bevy::asset::LoadState;
use bevy::gltf::{GltfMesh, GltfNode};
use bevy::prelude::*;
use bevy::tasks::{AsyncComputeTaskPool, Task};
use bevy::tasks::futures_lite::future;
use bevy_rapier3d::prelude::*;
use crate::entities::player::Player;
use crate::environment::{Chunk};
use crate::environment::chunk_builder::*;

#[derive(Component, Resource, Debug, Default)]
pub struct ChunkManager {
    pub chunk_entries: HashMap<(i32, i32), Chunk>,
    pub load_tasks: Vec<Task<HashMap<(i32, i32), Chunk>>>,
    pub need_update: bool,
}

#[derive(Resource)]
pub struct SceneHandleResource {
    pub handle: Handle<Gltf>,
}

struct ChildData {
    #[allow(dead_code)] // Only internal usage.
    name: String,
    translation: (i32, i32),
    scale: i32,
}

pub struct ChunkHandlerPlugin;

impl Plugin for ChunkHandlerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ChunkManager::default());
        app.add_systems(Startup,
            load_save_config_area_file);

        app.add_systems(Update, (create_chunk_loading_task, process_chunk_loading_task_data).after(load_save_config_area_file));

        app.add_systems(Update, (load_chunks, unload_chunks));
    }
}

fn load_save_config_area_file(mut commands: Commands,
                              asset_server: Res<AssetServer>,
                              mut chunk_manager: ResMut<ChunkManager>,
) {
    let scene_area_handle= asset_server.load("maps/debug.glb");
    commands.insert_resource(SceneHandleResource{handle: scene_area_handle.clone()});

    info!("Load scene config area from {:?}", scene_area_handle);
    chunk_manager.need_update = true;
}

fn create_chunk_loading_task(
    asset_server: Res<AssetServer>,
    scene_handle: Res<SceneHandleResource>,
    glb_handle: Res<Assets<Gltf>>,
    node_handle: Res<Assets<GltfNode>>,
    mut chunk_manager: ResMut<ChunkManager>,
) {
    let load_state = asset_server.get_load_state(&scene_handle.handle);
    if load_state != Option::from(LoadState::Loaded) {
        return;
    }

    if chunk_manager.need_update {
    let node_data: HashMap<String, (Handle<GltfNode>, Vec<ChildData>)> = if let Some(gltf) = glb_handle.get(&scene_handle.handle) {
        gltf.named_nodes.iter()
            .filter_map(|(name, handle)| {
                if let Some(node) = node_handle.get(handle) {
                    let children = node.children.iter()
                        .filter(|child| child.name.contains("terrain"))
                        .map(|child| ChildData {
                            name: child.name.clone(),
                            translation: (child.transform.translation.x as i32, child.transform.translation.z as i32),
                            scale: child.transform.scale.x as i32 * 2,
                        })
                        .collect::<Vec<_>>();

                    Some((name.clone().to_string(), (handle.clone(), children)))
                } else {
                    None
                }
            })
            .collect()
    } else {
        HashMap::new()
    };

    let task_pool = AsyncComputeTaskPool::get();
    let task = task_pool.spawn(async move {
        let mut loaded_chunks = HashMap::new();

        for (name, (handle, children)) in node_data.iter() {
            for child in children {
                let (x, z) = child.translation;

                if !loaded_chunks.contains_key(&(x, z)) {
                    loaded_chunks.insert(
                        (x, z),
                        Chunk {
                            id: None,
                            node: handle.clone(),
                            x,
                            z,
                            size: child.scale,
                            loaded: false,
                            area: "debug".to_string(),
                            name: name.clone(),
                            player_inbound: false,
                        },
                    );

                    info!("Create new Chunk Thread - {:?} - {}", name, loaded_chunks.len());
                }
            }
        }

        loaded_chunks
    });

        chunk_manager.load_tasks.push(task);
        chunk_manager.need_update = false;
    }
}

fn process_chunk_loading_task_data(
    mut chunk_manager: ResMut<ChunkManager>
) {
    let mut completed_tasks = Vec::new();

    for (i, load_task) in chunk_manager.load_tasks.iter_mut().enumerate() {
        if let Some(loaded_chunks) = future::block_on(future::poll_once(load_task)) {
            completed_tasks.push((i, loaded_chunks));
        }
    }

    for (i, loaded_chunks) in completed_tasks {
        let chunk_entries = &mut chunk_manager.chunk_entries;

        for (pos, chunk) in loaded_chunks {
            chunk_entries.insert(pos, chunk);
        }

        let _ = chunk_manager.load_tasks.remove(i);
    }
}

fn load_chunks(mut commands: Commands,
               player_query: Query<&Transform, With<Player>>,
               node_handle: Res<Assets<GltfNode>>,
               mesh_handle: Res<Assets<GltfMesh>>,
               meshes: ResMut<Assets<Mesh>>,
               mut materials: ResMut<Assets<StandardMaterial>>,
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
                                    load_terrain(&mut commands, chunk, &meshes, &mut materials, child, mesh, &mut visibility_query);
                                }
                            }
                        }

                        if child.name.contains("vegetation") {
                            if let Some(mesh_option) = &child.mesh {
                                if let Some(mesh) = mesh_handle.get(&*mesh_option) {
                                    load_vegetation(&mut commands, chunk, &meshes, &mut materials, child, mesh, &mut visibility_query);
                                }
                            }
                        }

                        if child.name.contains("structures") {
                            if let Some(mesh_option) = &child.mesh {
                                if let Some(mesh) = mesh_handle.get(&*mesh_option) {
                                    load_structures(&mut commands, chunk, &meshes, &mut materials, child, mesh, &mut visibility_query);
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
    let unload_distance = 800.0;
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
        }
}

fn get_visible_chunks(player_transform: &Transform, size: i32) -> Vec<(i32, i32)> {
    let mut visible_chunks = Vec::new();

    let player_position = player_transform.translation;
    let chunk_size = size;
    let view_distance = 800.0;

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

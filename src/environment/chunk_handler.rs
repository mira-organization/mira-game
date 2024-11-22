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

/// func load area files. Note this need to be change because the area file is hard coded.
fn load_save_config_area_file(mut commands: Commands,
                              asset_server: Res<AssetServer>,
                              mut chunk_manager: ResMut<ChunkManager>,
) {
    let scene_area_handle= asset_server.load("maps/debug.glb");
    commands.insert_resource(SceneHandleResource{handle: scene_area_handle.clone()});

    info!("Load scene config area from {:?}", scene_area_handle);
    chunk_manager.need_update = true;
}

/// func create a new async task for [`process_chunk_loading_task_data`].
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
                        },
                    );

                    debug!("Create new Chunk Thread - {:?} - {}", name, loaded_chunks.len());
                }
            }
        }

        loaded_chunks
    });

        chunk_manager.load_tasks.push(task);
        chunk_manager.need_update = false;
    }
}

/// create async task for handle [`Chunk`] by [`Chunk`].
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

/// load chunks if the player near. Used the [``get_visible_chunks`] func internal.
fn load_chunks(
    mut commands: Commands,
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

                process_chunk(
                    &mut commands,
                    chunk,
                    &node_handle,
                    &mesh_handle,
                    &meshes,
                    &mut materials,
                    &mut visibility_query,
                );
            }
        }
    }
}

/// func for process [`Chunk`] building. Func called [`handle_terrain`], [`handle_vegetation`] and [`handle_structures`].
fn process_chunk(
    commands: &mut Commands,
    chunk: &mut Chunk,
    node_handle: &Res<Assets<GltfNode>>,
    mesh_handle: &Res<Assets<GltfMesh>>,
    meshes: &ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    visibility_query: &mut Query<(&mut Visibility, Option<&mut ColliderDisabled>)>,
) {
    if let Some(node) = node_handle.get(&chunk.node) {
        for child in node.children.iter() {
            if child.name.contains("terrain") {
                handle_terrain(commands, chunk, meshes, materials, child, mesh_handle, visibility_query);
            } else if child.name.contains("vegetation") {
                handle_vegetation(commands, chunk, meshes, materials, child, mesh_handle, visibility_query);
            } else if child.name.contains("structures") {
                handle_structures(commands, chunk, meshes, materials, child, mesh_handle, visibility_query);
            }
        }
    }
}

/// internal func for handle terrain which found by [`Chunk`].
fn handle_terrain(
    commands: &mut Commands,
    chunk: &mut Chunk,
    meshes: &ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    child: &GltfNode,
    mesh_handle: &Res<Assets<GltfMesh>>,
    visibility_query: &mut Query<(&mut Visibility, Option<&mut ColliderDisabled>)>,
) {
    if let Some(mesh_option) = &child.mesh {
        if let Some(mesh) = mesh_handle.get(&*mesh_option) {
            load_terrain(commands, chunk, meshes, materials, child, mesh, visibility_query);
        }
    }
}

/// internal func for handle vegetations which found by [`Chunk`].
fn handle_vegetation(
    commands: &mut Commands,
    chunk: &mut Chunk,
    meshes: &ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    child: &GltfNode,
    mesh_handle: &Res<Assets<GltfMesh>>,
    visibility_query: &mut Query<(&mut Visibility, Option<&mut ColliderDisabled>)>,
) {
    if let Some(mesh_option) = &child.mesh {
        if let Some(mesh) = mesh_handle.get(&*mesh_option) {
            load_vegetation(commands, chunk, meshes, materials, child, mesh, visibility_query);
        }
    } else {
        if child.children.is_empty() {
            return;
        }
        process_node_recursively("vegetation", commands, chunk, meshes, materials, child, mesh_handle, visibility_query);
    }
}

/// internal func for handle structures which found by [`Chunk`].
fn handle_structures(
    commands: &mut Commands,
    chunk: &mut Chunk,
    meshes: &ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    child: &GltfNode,
    mesh_handle: &Res<Assets<GltfMesh>>,
    visibility_query: &mut Query<(&mut Visibility, Option<&mut ColliderDisabled>)>,
) {
    if let Some(mesh_option) = &child.mesh {
        if let Some(mesh) = mesh_handle.get(&*mesh_option) {
            load_structures(commands, chunk, meshes, materials, child, mesh, visibility_query);
        }
    } else {
        if child.children.is_empty() {
            return;
        }
        process_node_recursively("structures", commands, chunk, meshes, materials, child, mesh_handle, visibility_query);
    }
}

/// unload chunks if the player to fdr away.
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
                    debug!("Unload {:?}", chunk.name);
                }
            }
        }
}

/// checks chunk visibility by player position. This is needed for unload and load chunks.
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

// ToDo: EXPERIMENTAL TEST IS NEEDED!
fn process_node_recursively(
    category: &str,
    commands: &mut Commands,
    chunk: &mut Chunk,
    meshes: &ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    node: &GltfNode,
    mesh_handle: &Res<Assets<GltfMesh>>,
    visibility_query: &mut Query<(&mut Visibility, Option<&mut ColliderDisabled>)>,
) {
    for child in node.children.iter() {
        if let Some(mesh_option) = &child.mesh {
            if let Some(mesh) = mesh_handle.get(&*mesh_option) {
                if category.eq_ignore_ascii_case("vegetation") {
                    load_vegetation(
                        commands,
                        chunk,
                        meshes,
                        materials,
                        child,
                        mesh,
                        visibility_query,
                    );
                    continue;
                }
                if category.eq_ignore_ascii_case("structures") {
                    load_structures(
                        commands,
                        chunk,
                        meshes,
                        materials,
                        child,
                        mesh,
                        visibility_query,
                    );
                    continue;
                }
            }
        } else {
            if child.children.is_empty() {
                continue;
            }
            process_node_recursively(
                category,
                commands,
                chunk,
                meshes,
                materials,
                child,
                mesh_handle,
                visibility_query
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chunk_manager_initialization() {
        let mut app = App::new();
        app.insert_resource(ChunkManager::default());

        let chunk_manager = app.world().resource::<ChunkManager>();
        assert!(chunk_manager.chunk_entries.is_empty());
        assert!(chunk_manager.load_tasks.is_empty());
        assert_eq!(chunk_manager.need_update, false);
    }

}


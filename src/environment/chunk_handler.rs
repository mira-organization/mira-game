use std::collections::HashMap;
use bevy::asset::LoadState;
use bevy::gltf::GltfNode;
use bevy::prelude::*;
use crate::environment::Chunk;

#[derive(Component, Resource, Debug, Default)]
pub struct ChunkManager {
    pub chunk_entries: HashMap<(i32, i32), Entity>,
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

        app.add_systems(Update, extract_chunks_from_current_areas.after(load_save_config_area_file));
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

fn extract_chunks_from_current_areas(mut commands: Commands,
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

    if let Some(gltf) = glb_handle.get(&scene_handle.handle) {
        for (name, handle) in gltf.named_nodes.iter() {
            if name.contains("chunk") {
                if let Some(node) = node_handle.get(&*handle) {
                    let x = node.transform.translation.x as i32;
                    let z = node.transform.translation.z as i32;

                    if !chunk_manager.chunk_entries.contains_key(&(x, z)) {
                        let entity = commands.spawn_empty().id();
                        chunk_manager.chunk_entries.insert((x, z), entity);

                        commands.entity(entity).insert(Chunk {
                            node: handle.clone(),
                            x,
                            z,
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

/// Visibility: 3 chunks = rendered, 2 = visibility ready, all others destroy.
fn load_chunks() {}

fn unload_chunks() {}

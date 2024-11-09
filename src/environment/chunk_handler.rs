use std::collections::HashMap;
use bevy::prelude::*;

#[derive(Component, Resource, Debug, Default)]
pub struct ChunkManager {
    pub chunk_entries: HashMap<(i32, i32), Entity>,
    pub loaded_chunks: HashMap<(i32, i32), Handle<Scene>>,
}

pub struct ChunkHandlerPlugin;

impl Plugin for ChunkHandlerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ChunkManager::default());
        app.add_systems(Startup, load_area_file_convert_to_chunks);
    }
}

fn load_area_file_convert_to_chunks(mut commands: Commands,
                                    asset_server: Res<AssetServer>,
                                    mut chunk_manager: ResMut<ChunkManager>
) {
    let scene_area_handle = asset_server.load(GltfAssetLabel::Scene(0).from_asset("maps/debug1_area.glb"));
    for x in -1..=1 {
        for z in -1..=1 {
            let chunk_entity = commands.spawn((
                SceneBundle {
                    scene: scene_area_handle.clone(),
                    transform: Transform::from_xyz(x as f32 * 256.0, 0.0, z as f32 * 256.0),
                    visibility: Visibility::Hidden,
                    ..default()
                }
            )).id();

            chunk_manager.chunk_entries.insert((x, z), chunk_entity);
        }
    }

    info!("Chunks loaded: {}", chunk_manager.chunk_entries.len());
}

fn load_chunks() {}

fn unload_chunks() {}

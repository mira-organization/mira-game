use bevy::asset::RecursiveDependencyLoadState;
use bevy::prelude::*;
use crate::logic::loading_handler::pipeline_check::{PipelineCheckPlugin, PipelinesReady};

#[derive(Resource, Default)]
pub enum LoadingState {
    Loading,
    #[default]
    Ready
}

#[derive(Resource, Default, Debug)]
pub struct LoadingData {
    pub assets: Vec<UntypedHandle>,
    pub confirmation_frames_target: usize,
    pub confirmation_frames_count: usize,
}

impl LoadingData {
    pub fn new(confirmation_frames_target: usize) -> Self {
        Self {
            assets: Vec::new(),
            confirmation_frames_target,
            confirmation_frames_count: 0,
        }
    }
}

pub struct LoadingHandlerPlugin;

impl Plugin for LoadingHandlerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LoadingData::new(5))
            .insert_resource(LoadingState::default());

        app.add_systems(Update, update_pipeline_loading_state);

        app.add_plugins(PipelineCheckPlugin);
    }
}

fn update_pipeline_loading_state(mut loading_data: ResMut<LoadingData>,
                                 mut loading_state: ResMut<LoadingState>,
                                 asset_server: Res<AssetServer>,
                                 pipelines_ready: Res<PipelinesReady>) {
    if !loading_data.assets.is_empty() || !pipelines_ready.0 {
        loading_data.confirmation_frames_count = 0;

        let mut pop_list: Vec<usize> = Vec::new();
        for (index, asset) in loading_data.assets.iter().enumerate() {
            if let Some(state) = asset_server.get_recursive_dependency_load_state(asset) {
                if let RecursiveDependencyLoadState::Loaded = state {
                    pop_list.push(index);
                }
            }
        }

        for i in pop_list.iter() {
            loading_data.assets.remove(*i);
        }
    } else {
        loading_data.confirmation_frames_count += 1;
        if loading_data.confirmation_frames_count == loading_data.confirmation_frames_target {
            *loading_state = LoadingState::Ready;
            info!("Loading asset: {}", loading_data.assets.len());
        } else {
            *loading_state = LoadingState::Loading;
        }
    }
}

mod pipeline_check {
    use bevy::{prelude::*, render::render_resource::*, render::*};

    #[derive(Resource, Default, Debug)]
    pub struct PipelinesReady(pub bool);

    pub struct PipelineCheckPlugin;

    impl Plugin for PipelineCheckPlugin {
        fn build(&self, app: &mut App) {
            app.insert_resource(PipelinesReady::default());
            app.sub_app_mut(RenderApp)
                .add_systems(ExtractSchedule, update_pipeline_checks);
        }
    }

    fn update_pipeline_checks(mut main_world: ResMut<MainWorld>, pipelines: Res<PipelineCache>) {
        if let Some(mut ready) = main_world.get_resource_mut::<PipelinesReady>() {
            ready.0 = pipelines.waiting_pipelines().count() == 0;
        }
    }
}
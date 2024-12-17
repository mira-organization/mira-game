use bevy::prelude::*;
use crate::logic::loading::{pipeline_check::{LoadingData, LoadingHandlerPlugin},
                            pipeline_check::checker::PipelinesReady};
use rand::seq::SliceRandom;

mod pipeline_check;

#[derive(Component)]
struct ProgressBar;

#[derive(Component)]
struct LoadingText;

#[derive(Component)]
struct BackgroundImage;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LoadingHandlerPlugin);

        app.add_systems(Startup, setup_loading_screen);
        app.add_systems(Update, update_loading_screen);
    }
}

fn setup_loading_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    let images = vec![
        "backgrounds/loading/wait_mira_ch_pro_01.jpg",
        "backgrounds/loading/wait_mira_ch_pro_02.png",
        "backgrounds/loading/wait_mira_ch_pro_03.png",
    ];

    let random_image = images.choose(&mut rand::thread_rng()).unwrap();

    commands.spawn((Name::new("LoadingScreenCam"), Camera2d::default()));

    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            position_type: PositionType::Absolute,
            ..default()
        },
        BackgroundColor(Color::NONE)
    )).with_children(|child| {

        child.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                ..default()
            },
            ImageNode {
                image: asset_server.load(random_image.as_str()),
                ..default()
            },
            BackgroundImage
        ));

    });
}

fn update_loading_screen(loading_data: Res<LoadingData>,
                         pipelines_ready: Res<PipelinesReady>,
                         mut query_bar: Query<&mut Node, With<ProgressBar>>,
                         mut query_text: Query<&mut Text, With<LoadingText>>
) {

}
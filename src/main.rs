use std::collections::HashMap;

use bevy::{core::FixedTimestep, prelude::*, text::Text2dBounds};

mod ending;
mod home;
mod hud;
mod info;
mod jobs_list;
mod modes;
mod start;
mod work;
mod loading;
mod minigames;

const WIDTH: f32 =  1600.0;
const HEIGHT: f32 = 1200.0;

const TIMESTEP: f64 = 30.0 / 60.0;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    Loading,
    Start,
    Home,
    Modes,
    JobsList,
    Work,
    Ending,
}



fn main() {
    App::new()
        .add_state(AppState::Loading)
        .insert_resource(WindowDescriptor {
            title: "BiO Career Man II".to_string(),
            width: WIDTH,
            height: HEIGHT,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_plugin(home::Home)
        .add_plugin(jobs_list::JobsList)
        .add_plugin(modes::Modes)
        .add_plugin(work::Work)
        .add_plugin(ending::Ending)
        .add_plugin(start::Start)
		.add_plugin(loading::Loading)
        .add_system(bevy::input::system::exit_on_esc_system)
        .insert_resource(loading::LoadedAssets(HashMap::new()))
        .insert_resource(loading::LoadedFonts(HashMap::new()))
        .insert_resource(loading::AssetsLoading(Vec::new()))
        .insert_resource(loading::LoadingIndex(0))
        .add_startup_system(spawn_cameras)
		.add_startup_system(info::create_library)
        .add_system(hud::update_hud)
        .run()
}

fn spawn_cameras(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}

use std::collections::HashMap;

use bevy::prelude::*;
use info::{Library, create_library};

mod ending;
mod home;
mod hud;
mod info;
mod jobs_list;
mod modes;
mod randomizer;
mod work;

const WIDTH: f32 = 1600.0;
const HEIGHT: f32 = 1200.0;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    Start,
    Home,
    Modes,
    JobsList,
    Work,
    Ending,
}

pub struct GameProgress {
    money: usize,
    humanness: i32,
    day: usize,
    max_days: usize,
    library: info::Library,
	modes: Vec<(info::Mode, bool)>,
}

#[derive(Component)]
pub struct StartMarker;

pub struct LoadedAssets(HashMap<String, Handle<Image>>);

fn main() {
    App::new()
        .add_state(AppState::Start)
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
        .add_system(bevy::input::system::exit_on_esc_system)
        .insert_resource(GameProgress {
            money: 0,
            humanness: 100,
            day: 1,
            max_days: 15,
            library: info::Library {
                letters: Vec::new(),
                min_len: Vec::new(),
                max_len: Vec::new(),
				news: Vec::new(),
            },
			modes: Vec::new(),
        })
		.insert_resource(LoadedAssets(HashMap::new()))
		.add_startup_system(load_assets)
        .add_system_set(SystemSet::on_enter(AppState::Start).with_system(spawn_start))
        .add_system_set(SystemSet::on_update(AppState::Start).with_system(start_input))
        .add_system_set(SystemSet::on_exit(AppState::Start).with_system(cleanup_start))
        .add_system(change_state)
        .add_system(hud::update_hud)
        .run()
}

fn spawn_start(mut commands: Commands, assets: Res<AssetServer>, mut game_progress: ResMut<GameProgress>) {
	commands.spawn_bundle(OrthographicCameraBundle::new_2d());
	info::create_library(game_progress);

    commands
        .spawn_bundle(SpriteBundle {
            texture: assets.load("start.png"),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                ..Default::default()
            },
            sprite: Sprite {
                custom_size: Some(Vec2::new(800.0, 600.0)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(StartMarker);
}

fn cleanup_start(mut commands: Commands, query: Query<Entity, With<StartMarker>>) {
    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}

fn start_input(keys: Res<Input<KeyCode>>, mut app_state: ResMut<State<AppState>>) {
    if keys.just_pressed(KeyCode::S) {
        app_state.set(AppState::Home).unwrap();
    }
}

fn change_state(keys: Res<Input<KeyCode>>, mut app_state: ResMut<State<AppState>>) {
    if keys.just_pressed(KeyCode::Space) {
        match app_state.current() {
            AppState::Start => app_state.set(AppState::Home).unwrap(),
            AppState::Home => app_state.set(AppState::Modes).unwrap(),
            AppState::Modes => app_state.set(AppState::JobsList).unwrap(),
            AppState::JobsList => app_state.set(AppState::Work).unwrap(),
            AppState::Work => app_state.set(AppState::Ending).unwrap(),
            AppState::Ending => app_state.set(AppState::Start).unwrap(),
        }
    }
}

fn load_assets(mut assets: ResMut<LoadedAssets>, asset_server: Res<AssetServer>) {
    let names = [
        "home_new.png",
        "work_new.png",
        "customer_bubble.png",
        "bcman_bubble.png",
    ];

    for name in names {
        assets.0.insert(name.to_string(), asset_server.load(name));
    }
}

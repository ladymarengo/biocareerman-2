use std::collections::HashMap;

use bevy::{prelude::*, text::Text2dBounds};

mod ending;
mod home;
mod hud;
mod info;
mod jobs_list;
mod modes;
mod randomizer;
mod work;
mod start;

const WIDTH: f32 = 1600.0;
const HEIGHT: f32 = 1200.0;

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

pub struct GameProgress {
    money: usize,
    humanness: i32,
    day: usize,
    max_days: usize,
    library: info::Library,
    modes: Vec<(info::Mode, bool)>,
    customers: Vec<info::CallCenterTask>,
}

pub struct LoadedAssets(HashMap<String, Handle<Image>>);

pub struct LoadedFonts(HashMap<String, Handle<Font>>);

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
            customers: Vec::new(),
        })
        .insert_resource(LoadedAssets(HashMap::new()))
        // .add_startup_system(load_assets)
        .add_system(hud::update_hud)
		.add_system_set(
            SystemSet::on_enter(AppState::Loading)
                .with_system(load_assets))
		.add_system_set(
			SystemSet::on_update(AppState::Loading)
				.with_system(change_state)
        )
        .run()
}

fn load_assets(
    mut assets: ResMut<LoadedAssets>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    let names = [
        "logo.png",
        "home_new.png",
        "work_new.png",
        "customer_bubble.png",
        "bcman_bubble.png",
        "customer_color.png",
        "customer_face_1.png",
        "customer_mask.png",
        "customer_redness.png",
        "eye_mod_work.png",
        "eye_mod_home.png",
        "smilemod_work.png",
        "smilemod_home.png",
        "Bahamas.png",
        "dumpster.png",
        "newfarm.png",
    ];

    for name in names {
        assets.0.insert(name.to_string(), asset_server.load(name));
    }
    let music = asset_server.load("sounds/100_humanlong.ogg");
    audio.play(music);

}

fn change_state(mut app_state: ResMut<State<AppState>>) {
	app_state.set(AppState::Start).unwrap();
}

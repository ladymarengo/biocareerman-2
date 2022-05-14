use bevy::prelude::*;

mod home;
mod jobs_list;
mod modes;
mod work;
mod ending;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    Start,
	Home,
	Modes,
	JobsList,
	Work,
	Ending
}

#[derive(Component)]
pub struct StartMarker;

fn main() {
    App::new()
        .add_state(AppState::Start)
        .insert_resource(WindowDescriptor {
            title: "BiO Career Man II".to_string(),
            width: 800.0,
            height: 600.0,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::BLUE))
		.add_plugins(DefaultPlugins)
		.add_plugin(home::Home)
		.add_plugin(jobs_list::JobsList)
		.add_plugin(modes::Modes)
		.add_plugin(work::Work)
		.add_plugin(ending::Ending)
		.add_system(bevy::input::system::exit_on_esc_system)
		.add_system_set(
            SystemSet::on_enter(AppState::Start)
                .with_system(spawn_start)
        )
		.add_system_set(
            SystemSet::on_update(AppState::Start)
                .with_system(start_input)
        )
		.add_system_set(
            SystemSet::on_exit(AppState::Start)
                .with_system(cleanup_start)
        )
		.add_system(change_state)
		.run()
}

fn spawn_start(mut commands: Commands, assets: Res<AssetServer>) {
	commands.spawn_bundle(OrthographicCameraBundle::new_2d());

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

fn start_input(keys: Res<Input<KeyCode>>, mut app_state: ResMut<State<AppState>>)
{
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
			AppState::Ending => app_state.set(AppState::Start).unwrap()
		}
    }
}

use bevy::prelude::*;
use super::*;

pub struct Home;

#[derive(Component)]
pub struct HomeMarker;

impl Plugin for Home {
    fn build(&self, app: &mut App) {
        app
        .add_system_set(
            SystemSet::on_enter(AppState::Home)
                .with_system(spawn_home)
        )
		.add_system_set(
            SystemSet::on_update(AppState::Home)
                .with_system(home_input)
        )
		.add_system_set(
			SystemSet::on_exit(AppState::Home)
				.with_system(cleanup_home));
		}
	}

fn spawn_home(mut commands: Commands, assets: Res<AssetServer>) {
	commands
        .spawn_bundle(SpriteBundle {
            texture: assets.load("home.png"),
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
		.insert(HomeMarker);
}

fn home_input(keys: Res<Input<KeyCode>>, mut app_state: ResMut<State<AppState>>)
{
	if keys.just_pressed(KeyCode::W) {
		app_state.set(AppState::JobsList).unwrap();
	} else if keys.just_pressed(KeyCode::M) {
		app_state.set(AppState::Modes).unwrap();
	}
}

fn cleanup_home(mut commands: Commands, query: Query<Entity, With<HomeMarker>>) {
    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}
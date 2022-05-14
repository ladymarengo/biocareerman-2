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
			SystemSet::on_exit(AppState::Home)
				.with_system(cleanup_home));
		}
	}

fn spawn_home(mut commands: Commands) {
	commands.spawn_bundle(OrthographicCameraBundle::new_2d());

	println!("home");

    commands.spawn_bundle(SpriteBundle {
		transform: Transform {
			translation: Vec3::new(0.0, 0.0, 2.0),
			..Default::default()
		},
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(50.0, 100.0)),
            ..default()
        },
        ..default()
    })
		.insert(HomeMarker);
}

fn cleanup_home(mut commands: Commands, query: Query<Entity, With<HomeMarker>>) {
    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}
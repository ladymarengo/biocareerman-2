use bevy::prelude::*;
use super::*;

pub struct Ending;

#[derive(Component)]
pub struct EndingMarker;

impl Plugin for Ending {
    fn build(&self, app: &mut App) {
        app
        .add_system_set(
            SystemSet::on_enter(AppState::Ending)
                .with_system(spawn_ending)
        )
		.add_system_set(
			SystemSet::on_exit(AppState::Ending)
				.with_system(cleanup_ending));
		}
	}

fn spawn_ending(mut commands: Commands) {
	commands.spawn_bundle(OrthographicCameraBundle::new_2d());

	println!("Ending");

    commands.spawn_bundle(SpriteBundle {
		transform: Transform {
			translation: Vec3::new(0.0, 0.0, 2.0),
			..Default::default()
		},
        sprite: Sprite {
            color: Color::rgb(0.0, 0.25, 1.0),
            custom_size: Some(Vec2::new(50.0, 100.0)),
            ..default()
        },
        ..default()
    })
		.insert(EndingMarker);
}

fn cleanup_ending(mut commands: Commands, query: Query<Entity, With<EndingMarker>>) {
    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}
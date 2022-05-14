use bevy::prelude::*;
use super::*;

pub struct Work;

#[derive(Component)]
pub struct WorkMarker;

impl Plugin for Work {
    fn build(&self, app: &mut App) {
        app
        .add_system_set(
            SystemSet::on_enter(AppState::Work)
                .with_system(spawn_work)
        )
		.add_system_set(
			SystemSet::on_exit(AppState::Work)
				.with_system(cleanup_work));
		}
	}

fn spawn_work(mut commands: Commands) {
	commands.spawn_bundle(OrthographicCameraBundle::new_2d());

	println!("Work");

    commands.spawn_bundle(SpriteBundle {
		transform: Transform {
			translation: Vec3::new(0.0, 0.0, 2.0),
			..Default::default()
		},
        sprite: Sprite {
            color: Color::rgb(0.25, 0.0, 0.75),
            custom_size: Some(Vec2::new(50.0, 100.0)),
            ..default()
        },
        ..default()
    })
		.insert(WorkMarker);
}

fn cleanup_work(mut commands: Commands, query: Query<Entity, With<WorkMarker>>) {
    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}
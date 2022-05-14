use bevy::prelude::*;
use super::*;

pub struct JobsList;

#[derive(Component)]
pub struct JobsListMarker;

impl Plugin for JobsList {
    fn build(&self, app: &mut App) {
        app
        .add_system_set(
            SystemSet::on_enter(AppState::JobsList)
                .with_system(spawn_jobslist)
        )
		.add_system_set(
			SystemSet::on_exit(AppState::JobsList)
				.with_system(cleanup_jobslist));
		}
	}

fn spawn_jobslist(mut commands: Commands) {
	commands.spawn_bundle(OrthographicCameraBundle::new_2d());

	println!("JobsList");

    commands.spawn_bundle(SpriteBundle {
		transform: Transform {
			translation: Vec3::new(0.0, 0.0, 2.0),
			..Default::default()
		},
        sprite: Sprite {
            color: Color::rgb(0.75, 0.75, 0.75),
            custom_size: Some(Vec2::new(50.0, 100.0)),
            ..default()
        },
        ..default()
    })
		.insert(JobsListMarker);
}

fn cleanup_jobslist(mut commands: Commands, query: Query<Entity, With<JobsListMarker>>) {
    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}
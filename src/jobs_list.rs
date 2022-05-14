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
            SystemSet::on_update(AppState::JobsList)
                .with_system(jobslist_input)
        )
		.add_system_set(
			SystemSet::on_exit(AppState::JobsList)
				.with_system(cleanup_jobslist));
		}
	}

fn spawn_jobslist(mut commands: Commands, assets: Res<AssetServer>) {
	commands
        .spawn_bundle(SpriteBundle {
            texture: assets.load("jobslist.png"),
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
		.insert(JobsListMarker);
}

fn jobslist_input(keys: Res<Input<KeyCode>>, mut app_state: ResMut<State<AppState>>)
{
	if keys.just_pressed(KeyCode::H) {
		app_state.set(AppState::Home).unwrap();
	} else if keys.just_pressed(KeyCode::W) {
		app_state.set(AppState::Work).unwrap();
	}
}

fn cleanup_jobslist(mut commands: Commands, query: Query<Entity, With<JobsListMarker>>) {
    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}
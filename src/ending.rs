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
            SystemSet::on_update(AppState::Ending)
                .with_system(ending_input)
        )
		.add_system_set(
			SystemSet::on_exit(AppState::Ending)
				.with_system(cleanup_ending));
		}
	}

fn spawn_ending(mut commands: Commands, assets: Res<AssetServer>) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: assets.load("ending.png"),
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
		.insert(EndingMarker);
}

fn ending_input(keys: Res<Input<KeyCode>>, mut app_state: ResMut<State<AppState>>)
{
	if keys.just_pressed(KeyCode::S) {
		app_state.set(AppState::Start).unwrap();
	}
}

fn cleanup_ending(mut commands: Commands, query: Query<Entity, With<EndingMarker>>) {
    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}
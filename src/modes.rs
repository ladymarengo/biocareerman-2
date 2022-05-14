use bevy::prelude::*;
use super::*;

pub struct Modes;

#[derive(Component)]
pub struct ModesMarker;

impl Plugin for Modes {
    fn build(&self, app: &mut App) {
        app
        .add_system_set(
            SystemSet::on_enter(AppState::Modes)
                .with_system(spawn_modes)
        )
		.add_system_set(
			SystemSet::on_exit(AppState::Modes)
				.with_system(cleanup_modes));
		}
	}

fn spawn_modes(mut commands: Commands) {
	commands.spawn_bundle(OrthographicCameraBundle::new_2d());

	println!("Modes");

    commands.spawn_bundle(SpriteBundle {
		transform: Transform {
			translation: Vec3::new(0.0, 0.0, 2.0),
			..Default::default()
		},
        sprite: Sprite {
            color: Color::rgb(0.0, 1.0, 1.0),
            custom_size: Some(Vec2::new(50.0, 100.0)),
            ..default()
        },
        ..default()
    })
		.insert(ModesMarker);
}

fn cleanup_modes(mut commands: Commands, query: Query<Entity, With<ModesMarker>>) {
    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}
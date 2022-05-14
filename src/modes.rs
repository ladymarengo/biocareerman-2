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
            SystemSet::on_update(AppState::Modes)
                .with_system(modes_input)
        )
		.add_system_set(
			SystemSet::on_exit(AppState::Modes)
				.with_system(cleanup_modes));
		}
	}

fn spawn_modes(mut commands: Commands, assets: Res<AssetServer>) {
	commands
        .spawn_bundle(SpriteBundle {
            texture: assets.load("modes.png"),
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
		.insert(ModesMarker);
}

fn modes_input(keys: Res<Input<KeyCode>>, mut app_state: ResMut<State<AppState>>)
{
	if keys.just_pressed(KeyCode::H) {
		app_state.set(AppState::Home).unwrap();
	}
}

fn cleanup_modes(mut commands: Commands, query: Query<Entity, With<ModesMarker>>) {
    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}
use bevy::prelude::*;
use super::*;
use crate::hud::*;

pub struct Ending;

#[derive(Component)]
pub struct EndingMarker;

impl Plugin for Ending {
    fn build(&self, app: &mut App) {
        app
        .add_system_set(
            SystemSet::on_enter(AppState::Ending)
                .with_system(spawn_ending)
				.with_system(hud::cleanup_hud)
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

fn spawn_ending(mut commands: Commands, assets: Res<AssetServer>, load_assets: Res<LoadedAssets>, game_progress: Res<GameProgress>) {
	if game_progress.money >= 1000 {
		commands
        .spawn_bundle(SpriteBundle {
            texture: load_assets.0.get("Bahamas.png").unwrap().clone(),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                ..Default::default()
            },
            sprite: Sprite {
                custom_size: Some(Vec2::new(WIDTH, HEIGHT)),
                ..Default::default()
            },
            ..Default::default()
        })
		.insert(EndingMarker);
	} else if game_progress.money >= 500 {
		commands
        .spawn_bundle(SpriteBundle {
            texture: load_assets.0.get("newfarm.png").unwrap().clone(),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                ..Default::default()
            },
            sprite: Sprite {
                custom_size: Some(Vec2::new(WIDTH, HEIGHT)),
                ..Default::default()
            },
            ..Default::default()
        })
		.insert(EndingMarker);
	} else {
		commands
        .spawn_bundle(SpriteBundle {
            texture: load_assets.0.get("dumpster.png").unwrap().clone(),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                ..Default::default()
            },
            sprite: Sprite {
                custom_size: Some(Vec2::new(WIDTH, HEIGHT)),
                ..Default::default()
            },
            ..Default::default()
        })
		.insert(EndingMarker);
	}

	if game_progress.humanness < 90 {
		commands
		.spawn_bundle(SpriteBundle {
			texture: load_assets.0.get("eye_mod_work.png").unwrap().clone(),
			transform: Transform {
				translation: Vec3::new(0.0, 0.0, 5.0),
				..Default::default()
			},
			sprite: Sprite {
				custom_size: Some(Vec2::new(WIDTH, HEIGHT)),
				..Default::default()
			},
			..Default::default()
		})
		.insert(EndingMarker);
	}
	
	if game_progress.modes[1].1 {
		commands
		.spawn_bundle(SpriteBundle {
			texture: load_assets.0.get("smilemod_work.png").unwrap().clone(),
			transform: Transform {
				translation: Vec3::new(0.0, 0.0, 5.0),
				..Default::default()
			},
			sprite: Sprite {
				custom_size: Some(Vec2::new(WIDTH, HEIGHT)),
				..Default::default()
			},
			..Default::default()
		})
		.insert(EndingMarker);
	}

	commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::Auto,
                position_type: PositionType::Absolute,
                position: Rect {
                    bottom: Val::Px(50.0),
                    left: Val::Px(WIDTH / 2.0 - 200.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text::with_section(
                "Press A to start again",
                TextStyle {
                    font: assets.load("FiraMono-Medium.ttf"),
                    font_size: 40.0,
                    color: Color::WHITE,
                },
                TextAlignment {
                    horizontal: HorizontalAlign::Center,
                    vertical: VerticalAlign::Center,
                    ..Default::default()
                },
            ),
            ..Default::default()
        })
		.insert(EndingMarker);
    
}

fn ending_input(keys: Res<Input<KeyCode>>, mut app_state: ResMut<State<AppState>>)
{
	if keys.just_pressed(KeyCode::A) {
		app_state.set(AppState::Start).unwrap();
	}
}

fn cleanup_ending(mut commands: Commands, query: Query<Entity, With<EndingMarker>>) {
    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}
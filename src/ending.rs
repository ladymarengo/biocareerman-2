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

	let endings = vec![
		"You're on a beach and you're the\nhappiest human on the planet.".to_string(),
		"You're on a beach. Everything is wonderful\nbut sometimes your eye-mode is burning\nhot from the Sun.".to_string(),
		"You're on a beach. You notice some\nrustiness from salty water and hear\nsand cracking in your metal joints.".to_string(),
		"You're on a farm and you're one with it.".to_string(),
		"You're on a farm and you feel pretty good.".to_string(),
		"You're on a farm and smelling feces\ndon't bother you at all.".to_string(),
		"You're in a dumpster but you're a human\nso you will find reasons to be happy\neven in this place.".to_string(),
		"You're in a dumpster. You're not too\nhappy about it but it's okay.".to_string(),
		"You're in a dumpster and you're a robot.\nYou feel weird connection with it.".to_string(),
	];

	let ending;

	if game_progress.money >= 1000 {
		if game_progress.humanness >= 75 {
			ending = endings[0].clone();
		} else if game_progress.humanness >= 40 {
			ending = endings[1].clone();
		} else {
			ending = endings[2].clone();
		}
	} else if game_progress.money >= 500 {
		if game_progress.humanness >= 75 {
			ending = endings[3].clone();
		} else if game_progress.humanness >= 40 {
			ending = endings[4].clone();
		} else {
			ending = endings[5].clone();
		}
	} else {
		if game_progress.humanness >= 75 {
			ending = endings[6].clone();
		} else if game_progress.humanness >= 40 {
			ending = endings[7].clone();
		} else {
			ending = endings[8].clone();
		}
	}

	commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::Auto,
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(50.0),
                    left: Val::Px(WIDTH / 2.0 - 200.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text::with_section(
                ending,
                TextStyle {
                    font: assets.load("FiraMono-Medium.ttf"),
                    font_size: 40.0,
                    color: Color::GRAY,
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
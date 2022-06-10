use std::cmp::min;

use super::*;
use crate::randomizer::get_random_word;
use bevy::math::Rect;

use instant::Instant;
use rand::Rng;

pub struct Work;

#[derive(Component)]
pub struct WorkMarker;

#[derive(Component)]
pub struct Bubble;

#[derive(Component)]
pub struct Phrase;

#[derive(Component)]
pub struct DelayTimer(Instant);

#[derive(Component)]
pub struct WorkDayTimer(Instant);


#[derive(Component)]
struct Word {
    word: String,
    index: usize,
    errors: usize,
    timer: Instant,
    started: bool,
	marked_to_despawn: bool,
	despawn_timer: Instant,
	minigame: MiniGame,
}

enum MiniGame {
	RANDOM_WORD,
	RANDOM_LETTERS,
	COUNT,
}

#[derive(Component)]
pub struct Redness;

impl Plugin for Work {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(AppState::Work)
				.with_system(spawn_minigame)
                .with_system(spawn_work)
                .with_system(spawn_word),
        )
        .insert_resource(DelayTimer(Instant::now()))
        .insert_resource(WorkDayTimer(Instant::now()))
        .add_system_set(
            SystemSet::on_update(AppState::Work)
                .with_system(text_input.label("print"))
                .with_system(spawn_word.after("print"))
                .with_system(correct_redness),
        )
        .add_system_set(SystemSet::on_exit(AppState::Work).with_system(cleanup_work));
    }
}

fn spawn_minigame(
	mut commands: Commands,
    query: Query<Entity, With<Word>>,
    asset_server: Res<AssetServer>,
    game_progress: ResMut<GameProgress>,
    redness: Query<Entity, With<Redness>>,
) {
	let mut rng = ::rand::thread_rng();
    let minigame: i32 = rng.gen_range(0..MiniGame::COUNT as i32);
	// println!("{} {}", MiniGame::COUNT as i32, minigame);

	if query.iter().collect::<Vec<Entity>>().len() == 0 {
		match minigame {
			0 => spawn_word(commands, query, asset_server, game_progress, redness),
			1 => spawn_letters(commands, query, asset_server, game_progress),
			_ => (),
		}
	}
}

fn spawn_work(
    mut commands: Commands,
    mut timer: ResMut<WorkDayTimer>,
    assets: Res<AssetServer>,
    load_assets: Res<LoadedAssets>,
    game_progress: Res<GameProgress>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
    timer.0 = Instant::now();

    // println!("Work");

    commands
        .spawn_bundle(SpriteBundle {
            texture: load_assets.0.get("work_new.png").unwrap().clone(),
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
        .insert(WorkMarker);

    commands
        .spawn_bundle(SpriteBundle {
            texture: load_assets.0.get("customer_bubble.png").unwrap().clone(),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.5),
                ..Default::default()
            },
            sprite: Sprite {
                custom_size: Some(Vec2::new(WIDTH, HEIGHT)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Bubble)
        .insert(WorkMarker);

    commands
        .spawn_bundle(SpriteBundle {
            texture: load_assets.0.get("customer_color.png").unwrap().clone(),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 1.0),
                ..Default::default()
            },
            sprite: Sprite {
                custom_size: Some(Vec2::new(WIDTH, HEIGHT)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(WorkMarker);

    commands
        .spawn_bundle(SpriteBundle {
            texture: load_assets.0.get("customer_face_1.png").unwrap().clone(),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 3.0),
                ..Default::default()
            },
            sprite: Sprite {
                custom_size: Some(Vec2::new(WIDTH, HEIGHT)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(WorkMarker);

    commands
        .spawn_bundle(SpriteBundle {
            texture: load_assets.0.get("customer_mask.png").unwrap().clone(),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 4.0),
                ..Default::default()
            },
            sprite: Sprite {
                custom_size: Some(Vec2::new(WIDTH, HEIGHT)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(WorkMarker);

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
            .insert(WorkMarker);
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
            .insert(WorkMarker);
    }

    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::Auto,
                position_type: PositionType::Absolute,
                position: Rect {
                    bottom: Val::Px(20.0),
                    left: Val::Px(WIDTH / 2.0 - 300.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text::with_section(
                "Type gray letters to complete tasks",
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
        .insert(WorkMarker);
}

fn cleanup_work(mut commands: Commands, query: Query<Entity, With<WorkMarker>>) {
    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}

fn spawn_word(
    mut commands: Commands,
    query: Query<Entity, With<Word>>,
    asset_server: Res<AssetServer>,
    game_progress: ResMut<GameProgress>,
    redness: Query<Entity, With<Redness>>,
) {
    if game_progress.day >= 16 {
        return;
    }
    let min_len = if game_progress.modes[3].1 {
        min(4, game_progress.library.min_len[game_progress.day - 1])
    } else {
        game_progress.library.min_len[game_progress.day - 1]
    };
    let max_len = if game_progress.modes[3].1 {
        min(4, game_progress.library.max_len[game_progress.day - 1])
    } else {
        game_progress.library.max_len[game_progress.day - 1]
    };

    if query.iter().collect::<Vec<Entity>>().len() == 0 {

        for e in redness.iter() {
            commands.entity(e).despawn_recursive();
        }

        let word = get_random_word(
            &game_progress.library.letters[game_progress.day - 1],
            min_len,
            max_len,
        );

		commands
            .spawn_bundle(Text2dBundle {
                text: Text {
                    alignment: TextAlignment {
                        vertical: VerticalAlign::Center,
                        horizontal: HorizontalAlign::Left,
                        ..Default::default()
                    },
                    sections: create_phrase_sections(&word, asset_server, &game_progress),
                    ..default()
                },
                text_2d_bounds: Text2dBounds {
                    size: Size {
                        width: WIDTH * 0.6,
                        height: HEIGHT * 0.1,
                    },
                },
                transform: Transform::from_xyz(
                    -(WIDTH * 0.2),
                    -(HEIGHT * 0.3),
                    1.0,
                ),
                ..default()
            })
			.insert(Word {
				        word: word,
				        index: 0,
				        errors: 0,
				        timer: Instant::now(),
				        started: false,
						marked_to_despawn: false,
						despawn_timer: Instant::now(),
						minigame: MiniGame::RANDOM_WORD,
				    })
			.insert(WorkMarker);
    }
}

fn spawn_letters(
    mut commands: Commands,
    query: Query<Entity, With<Word>>,
	asset_server: Res<AssetServer>,
	game_progress: ResMut<GameProgress>,
) {
	let mut sections = Vec::new();
	let phrase_index = ::rand::thread_rng().gen_range(0..game_progress.customers.random_letter.len());
	let phrase = game_progress.customers.random_letter[phrase_index].clone();

	for c in phrase.chars() {
		sections.push(TextSection {
			value: c.to_string(),
			style: TextStyle {
				font: asset_server.load("FiraMono-Medium.ttf"),
				font_size: 50.0,
				color: Color::GRAY,
			},
		});
	}
	commands
		.spawn_bundle(Text2dBundle {
			text: Text {
				alignment: TextAlignment {
					vertical: VerticalAlign::Center,
					horizontal: HorizontalAlign::Left,
					..Default::default()
				},
				sections,
				..default()
			},
			text_2d_bounds: Text2dBounds {
				size: Size {
					width: WIDTH * 0.6,
					height: HEIGHT * 0.1,
				},
			},
			transform: Transform::from_xyz(
				-(WIDTH * 0.2),
				-(HEIGHT * 0.3),
				1.0,
			),
			..default()
		})
		.insert(Word {
					word: phrase,
					index: 0,
					errors: 0,
					timer: Instant::now(),
					started: false,
					marked_to_despawn: false,
					despawn_timer: Instant::now(),
					minigame: MiniGame::RANDOM_LETTERS,
				})
		.insert(WorkMarker);
}

fn create_phrase_sections(word: &str, asset_server: Res<AssetServer>, game_progress: &ResMut<GameProgress>) -> Vec<TextSection> {
    let mut sections = Vec::new();
	let phrase_index = ::rand::thread_rng().gen_range(0..game_progress.customers.random_word.len());

	sections.push(TextSection {
		value: game_progress.customers.random_word[phrase_index].beginning.clone(),
		style: TextStyle {
			font: asset_server.load("FiraMono-Medium.ttf"),
			font_size: 50.0,
			color: Color::BLACK,
		},
	});

    for character in word.chars() {
        sections.push(TextSection {
            value: character.to_string(),
            style: TextStyle {
                font: asset_server.load("FiraMono-Medium.ttf"),
                font_size: 60.0,
                color: Color::GRAY,
            },
        })
    }

	sections.push(TextSection {
		value: game_progress.customers.random_word[phrase_index].end.clone(),
		style: TextStyle {
			font: asset_server.load("FiraMono-Medium.ttf"),
			font_size: 50.0,
			color: Color::BLACK,
		},
	});

	sections.push(TextSection {
		value: "".to_string(),
		style: TextStyle {
			font: asset_server.load("FiraMono-Medium.ttf"),
			font_size: 50.0,
			color: Color::BLACK,
		},
	});

    sections
}

fn text_input(
    mut char_evr: EventReader<ReceivedCharacter>,
    mut string: Local<String>,
    mut query: Query<(Entity, &mut Word, &mut Text)>,
    mut commands: Commands,
    mut game_progress: ResMut<GameProgress>,
    app_state: ResMut<State<AppState>>,
    workdaytimer: Res<WorkDayTimer>,
) {
    if !query.is_empty() {
        let (id, mut word, mut text) = query.single_mut();

		if word.marked_to_despawn {
			if word.despawn_timer.elapsed().as_millis() > 1000 {
				commands.entity(id).despawn();
                finish_day(game_progress, workdaytimer, app_state);
			}
			return;
		}

        for ev in char_evr.iter() {
            string.push(ev.char);
            if word.index < word.word.len()
                && ev.char != ' '
                && !(ev.char == 'w' && word.index == 0)
            {
                if word.index == 0 {
                    word.timer = Instant::now();
                    word.started = true;
                }
                if word.word.as_bytes()[word.index] == ev.char as u8
                    || if_letter_locked(&game_progress, word.word.as_bytes()[word.index] as char)
                {
                    text.sections[word.index + 1].style.color = Color::DARK_GREEN;
                } else {
                    word.errors += 1;
                    text.sections[word.index + 1].style.color = Color::RED;
                }
                word.index += 1;

                let mode_offset: u128 = if game_progress.modes[1].1 { 500 } else { 0 };

                if word.index == word.word.len() {
                    if word.errors <= 0 + game_progress.modes[0].1 as usize
                        && word.timer.elapsed().as_millis()
                            < word.word.len() as u128 * 300 - game_progress.day as u128 * 15
                                + mode_offset
                    {
						text.sections[word.index + 2].style.color = Color::DARK_GREEN;
						text.sections[word.index + 2].value = format!(" Perfect! Your reward is {} Botcoins.", word.index * word.word.len());
                        game_progress.money += word.index * word.word.len();
                    } else if word.errors > 1 + game_progress.modes[0].1 as usize
                        || word.timer.elapsed().as_millis()
                            > word.word.len() as u128 * 600 - game_progress.day as u128 * 30
                                + mode_offset
                    {
						text.sections[word.index + 2].style.color = Color::RED;
						text.sections[word.index + 2].value = format!(" Awful! Your reward is 0 Botcoins.");
                    } else {
						text.sections[word.index + 2].style.color = Color::YELLOW_GREEN;
						text.sections[word.index + 2].value = format!(" Fine. Your reward is {} Botcoins.", word.index);
                        game_progress.money += word.index;
                    }
					word.marked_to_despawn = true;
					word.despawn_timer = Instant::now();
                }
            }
        }
    }
}

fn correct_redness(
    mut commands: Commands,
    redness: Query<Entity, With<Redness>>,
    game_progress: Res<GameProgress>,
    word: Query<&Word>,
    load_assets: Res<LoadedAssets>,
) {
    let mode_offset: u128 = if game_progress.modes[1].1 { 500 } else { 0 };
    let redness: Vec<Entity> = redness.iter().collect();
    if !word.is_empty() {
        let word = word.single();
		if word.marked_to_despawn {
			return;
		}
        let redness_level = (word.timer.elapsed().as_millis()
            / ((word.word.len() as u128 * 600 - game_progress.day as u128 * 30 + mode_offset) / 10))
            as usize;
        // println!("{} {}", redness.len(), redness_level);
        if word.started && redness.len() < redness_level {
            commands
                .spawn_bundle(SpriteBundle {
                    texture: load_assets.0.get("customer_redness.png").unwrap().clone(),
                    transform: Transform {
                        translation: Vec3::new(0.0, 0.0, 2.0),
                        ..Default::default()
                    },
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(WIDTH, HEIGHT)),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Redness)
                .insert(WorkMarker);
        }
    }

    // println!("redness level {}", redness_level);
}

fn finish_day(
    mut game_progress: ResMut<GameProgress>,
    timer: Res<WorkDayTimer>,
    mut app_state: ResMut<State<AppState>>,
) {
    if timer.0.elapsed().as_secs() > 15 {
        game_progress.day += 1;
        match game_progress.day {
            16 => app_state.set(AppState::Ending).unwrap(),
            _ => app_state.set(AppState::Home).unwrap(),
        }
    }
}

fn if_letter_locked(game_progress: &ResMut<GameProgress>, c: char) -> bool {
    (game_progress.modes[2].1 && ("aeuioy".contains(c)))
        || (game_progress.modes[4].1 && ("qe".contains(c)))
}

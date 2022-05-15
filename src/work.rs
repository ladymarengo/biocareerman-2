use std::cmp::{min, max};

use super::*;
use crate::randomizer::get_random_word;
use bevy::math::Rect;
use bevy::prelude::*;
use instant::Instant;

pub struct Work;

#[derive(Component)]
pub struct WorkMarker;

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
}

impl Plugin for Work {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(AppState::Work)
                .with_system(spawn_work)
                .with_system(spawn_word),
        )
        .insert_resource(DelayTimer(Instant::now()))
        .insert_resource(WorkDayTimer(Instant::now()))
        .add_system_set(
            SystemSet::on_update(AppState::Work)
                .with_system(text_input.label("print"))
                .with_system(spawn_word.after("print")),
        )
        .add_system_set(SystemSet::on_exit(AppState::Work).with_system(cleanup_work));
    }
}

fn spawn_work(mut commands: Commands, mut timer: ResMut<WorkDayTimer>, assets: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
    timer.0 = Instant::now();

    // println!("Work");

    commands
        .spawn_bundle(SpriteBundle {
            texture: assets.load("work_new.png"),
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
    timer: Res<DelayTimer>,
    mut game_progress: ResMut<GameProgress>,
) {

	let min_len = if game_progress.modes[3].1 {min(4, game_progress.library.min_len[game_progress.day - 1])} else {game_progress.library.min_len[game_progress.day - 1]};
	let max_len = if game_progress.modes[3].1 {min(4, game_progress.library.max_len[game_progress.day - 1])} else {game_progress.library.max_len[game_progress.day - 1]};

    if query.iter().collect::<Vec<Entity>>().len() == 0 && timer.0.elapsed().as_millis() > 500 {
        let word = get_random_word(
            &game_progress.library.letters[game_progress.day - 1],
            min_len,
            max_len,
        );
        commands
            .spawn_bundle(TextBundle {
                style: Style {
                    align_self: AlignSelf::Center,
                    margin: Rect {
                        left: (Val::Auto),
                        right: (Val::Auto),
                        top: (Val::Auto),
                        bottom: (Val::Auto),
                    },
                    ..default()
                },
                text: Text {
                    sections: vectorize_word(&word, asset_server),
                    ..default()
                },
                ..default()
            })
            .insert(Word {
                word: word,
                index: 0,
                errors: 0,
                timer: Instant::now(),
            })
            .insert(WorkMarker);
    }
}

fn vectorize_word(word: &str, asset_server: Res<AssetServer>) -> Vec<TextSection> {
    let mut sections = Vec::new();

    for character in word.chars() {
        sections.push(TextSection {
            value: character.to_string(),
            style: TextStyle {
                font: asset_server.load("FiraMono-Medium.ttf"),
                font_size: 60.0,
                color: Color::GOLD,
            },
        })
    }
    sections
}

fn text_input(
    mut char_evr: EventReader<ReceivedCharacter>,
    // keys: Res<Input<KeyCode>>,
    mut string: Local<String>,
    mut query: Query<(Entity, &mut Word, &mut Text)>,
    mut commands: Commands,
    mut game_progress: ResMut<GameProgress>,
    mut timer: ResMut<DelayTimer>,
    mut app_state: ResMut<State<AppState>>,
    workdaytimer: Res<WorkDayTimer>,
) {
    if !query.is_empty() {
        let (id, mut word, mut text) = query.single_mut();

        for ev in char_evr.iter() {
            // println!("Got char: '{}'", ev.char);
            string.push(ev.char);
            if word.index < word.word.len()
                && ev.char != ' '
                && !(ev.char == 'w' && word.index == 0)
            {
                if word.index == 0 {
                    word.timer = Instant::now();
                }
                if word.word.as_bytes()[word.index] == ev.char as u8 || if_letter_locked(&game_progress, word.word.as_bytes()[word.index] as char) {
                    // println!("Yes!");
                    text.sections[word.index].style.color = Color::GREEN;
                } else {
                    // println!("No.");
                    word.errors += 1;
                    text.sections[word.index].style.color = Color::RED;
                }
                word.index += 1;

				let mode_offset: u128 = if game_progress.modes[1].1 {500} else {0};

                if word.index == word.word.len() {
					// println!("{}, {}, {}, {}", word.timer.elapsed().as_millis(), mode_offset, word.errors, game_progress.modes[0].1 as usize);

                    if word.errors <= 0 + game_progress.modes[0].1 as usize
                        && word.timer.elapsed().as_millis() < word.word.len() as u128 * 300 - game_progress.day as u128 * 15 + mode_offset
                    {
                        println!("Perfect!");
                        game_progress.money += word.index;
                    } else if word.errors > 1 + game_progress.modes[0].1 as usize
                        || word.timer.elapsed().as_millis() > word.word.len() as u128 * 600 - game_progress.day as u128 * 30 + mode_offset
                    {
						println!("Unsatisfying!");
                        
                    } else {
                        println!("Imperfect.");
                        game_progress.money += word.index / 2;
                        // println!("0");
                    }
                    commands.entity(id).despawn();
                    finish_day(game_progress, workdaytimer, app_state);
                    timer.0 = Instant::now();
                    return;
                }
            }
        }
    }

    // if keys.just_pressed(KeyCode::Return) {
    //     println!("Text input: {}", *string);
    //     string.clear();
    // }
}

fn finish_day(
    mut game_progress: ResMut<GameProgress>,
    timer: Res<WorkDayTimer>,
    mut app_state: ResMut<State<AppState>>,
) {
    if timer.0.elapsed().as_secs() > 10 {
        game_progress.day += 1;
        match game_progress.day {
            16 => app_state.set(AppState::Ending).unwrap(),
            _ => app_state.set(AppState::Home).unwrap(),
        }
    }
}

fn if_letter_locked(mut game_progress: &ResMut<GameProgress>, c: char) -> bool {
	(game_progress.modes[2].1 && ("aeuioy".contains(c))) || (game_progress.modes[4].1 && ("qe".contains(c)))
}

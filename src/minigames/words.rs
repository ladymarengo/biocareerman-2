use ::rand::Rng;
use bevy::prelude::*;
use instant::Instant;
use std::cmp::min;

use crate::info::*;
use crate::work::*;
use crate::*;

pub fn spawn_word(
    mut commands: Commands,
    query: Query<Entity, With<Word>>,
    asset_server: Res<AssetServer>,
    game_progress: ResMut<GameProgress>,
    redness: Query<Entity, With<Redness>>,
    disappear: bool,
) {
    if game_progress.day >= 16 {
        return;
    }
    let min_len = if game_progress.modes[3].1 {
        min(4, game_progress.library.len[game_progress.day - 1].min)
    } else {
        game_progress.library.len[game_progress.day - 1].min
    };
    let max_len = if game_progress.modes[3].1 {
        min(4, game_progress.library.len[game_progress.day - 1].max)
    } else {
        game_progress.library.len[game_progress.day - 1].max
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
                transform: Transform::from_xyz(-(WIDTH * 0.2), -(HEIGHT * 0.3), 1.0),
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
                minigame: if !disappear {
                    MiniGame::RandomWord
                } else {
                    MiniGame::RandomWordDisappear
                },
                letters: 0,
                typed: 0,
            })
            .insert(WorkMarker);
    }
}

fn create_phrase_sections(
    word: &str,
    asset_server: Res<AssetServer>,
    game_progress: &ResMut<GameProgress>,
) -> Vec<TextSection> {
    let mut sections = Vec::new();
    let phrase_index = ::rand::thread_rng().gen_range(0..game_progress.customers.random_word.len());

    sections.push(TextSection {
        value: game_progress.customers.random_word[phrase_index].0.clone(),
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
        value: game_progress.customers.random_word[phrase_index].1.clone(),
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

pub fn get_random_word(valid_letters: &[char], min_length: usize, max_length: usize) -> String {
    let mut rng = ::rand::thread_rng();
    let length: i32 = rng.gen_range(min_length as i32..=max_length as i32);
    let mut letter: usize;

    let mut word: Vec<char> = Vec::new();
    for _i in 0..length {
        letter = rng.gen_range(0..valid_letters.len());
        word.push(valid_letters[letter]);
    }
    word.into_iter().collect()
}

pub fn input_words(
    mut char_evr: EventReader<ReceivedCharacter>,
    mut query: Query<(Entity, &mut Word, &mut Text)>,
    mut game_progress: ResMut<GameProgress>,
) {
    let (_id, mut word, mut text) = query.single_mut();

    for ev in char_evr.iter() {
        if word.index < word.word.len() && ev.char != ' ' && !(ev.char == 'w' && word.index == 0) {
            if word.index == 0 {
                word.timer = Instant::now();
                word.started = true;
                if let MiniGame::RandomWordDisappear = word.minigame {
                    disappear_letters(&word, &mut text, &game_progress);
                }
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
                    text.sections[word.index + 2].value = format!(
                        " Perfect! Your reward is {} Botcoins.",
                        word.index
                            * 2
                            * if let MiniGame::RandomWordDisappear = word.minigame {
                                word.index
                            } else {
                                1
                            }
                    );
                    game_progress.money += word.index
                        * 2
                        * if let MiniGame::RandomWordDisappear = word.minigame {
                            word.index
                        } else {
                            1
                        };
                } else if word.errors > 1 + game_progress.modes[0].1 as usize
                    || word.timer.elapsed().as_millis()
                        > word.word.len() as u128 * 600 - game_progress.day as u128 * 30
                            + mode_offset
                {
                    text.sections[word.index + 2].style.color = Color::RED;
                    text.sections[word.index + 2].value =
                        format!(" Awful! Your reward is 0 Botcoins.");
                } else {
                    text.sections[word.index + 2].style.color = Color::YELLOW_GREEN;
                    text.sections[word.index + 2].value = format!(
                        " Fine. Your reward is {} Botcoins.",
                        word.index
                            * if let MiniGame::RandomWordDisappear = word.minigame {
                                2
                            } else {
                                1
                            }
                    );
                    game_progress.money += word.index
                        * if let MiniGame::RandomWordDisappear = word.minigame {
                            2
                        } else {
                            1
                        };
                }
                word.marked_to_despawn = true;
                word.despawn_timer = Instant::now();
                word.started = false;
            }
        }
    }
}

fn disappear_letters(
    word: &Mut<Word>,
    text: &mut Mut<Text>,
    _game_progress: &ResMut<GameProgress>,
) {
    for index in word.index + 1..=word.word.len() {
        text.sections[index].style.color = Color::rgb(0.9, 0.9, 0.9);
    }
}

use crate::info::*;
use crate::work::*;
use crate::*;
use ::rand::Rng;
use instant::Instant;

pub fn spawn_letters(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_progress: ResMut<GameProgress>,
    redness: Query<Entity, With<Redness>>,
) {
    let mut sections = Vec::new();
    let phrase_index =
        ::rand::thread_rng().gen_range(0..game_progress.customers.random_letter.len());
    let phrase = game_progress.customers.random_letter[phrase_index].clone();
    let index = get_target_letter(&phrase);

    for e in redness.iter() {
        commands.entity(e).despawn_recursive();
    }

    for (i, c) in phrase.chars().enumerate() {
        sections.push(TextSection {
            value: c.to_string(),
            style: TextStyle {
                font: asset_server.load("FiraMono-Medium.ttf"),
                font_size: if i != index { 50.0 } else { 70.0 },
                color: if i != index { Color::GRAY } else { Color::BLUE },
            },
        });
    }
    sections.push(TextSection {
        value: "".to_string(),
        style: TextStyle {
            font: asset_server.load("FiraMono-Medium.ttf"),
            font_size: 50.0,
            color: Color::BLACK,
        },
    });
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
            transform: Transform::from_xyz(-(WIDTH * 0.2), -(HEIGHT * 0.3), 1.0),
            ..default()
        })
        .insert(Word {
            word: phrase.clone(),
            index,
            errors: 0,
            timer: Instant::now(),
            started: false,
            marked_to_despawn: false,
            despawn_timer: Instant::now(),
            minigame: MiniGame::RandomLetters,
            letters: ::rand::thread_rng().gen_range(
                game_progress.library.len[game_progress.day - 1].min
                    ..=game_progress.library.len[game_progress.day - 1].max,
            ),
            typed: 0,
        })
        .insert(WorkMarker);
}

pub fn get_target_letter(phrase: &str) -> usize {
    let mut index: usize = ::rand::thread_rng().gen_range(0..phrase.len());

    while !phrase.chars().collect::<Vec<char>>()[index].is_alphanumeric() {
        index = ::rand::thread_rng().gen_range(0..phrase.len());
    }

    index
}

pub fn input_letters(
    mut char_evr: EventReader<ReceivedCharacter>,
    mut query: Query<(Entity, &mut Word, &mut Text)>,
    mut game_progress: ResMut<GameProgress>,
) {
    let (_id, mut word, mut text) = query.single_mut();

    for ev in char_evr.iter() {
        if ev.char as u8 == word.word.as_bytes()[word.index].to_ascii_lowercase() {
            text.sections[word.index].style.color = Color::DARK_GREEN;
        } else {
            text.sections[word.index].style.color = Color::RED;
            word.errors += 1;
        }

        if word.typed == 0 {
            word.timer = Instant::now();
            word.started = true;
        }

        text.sections[word.index].style.font_size = 50.0;
        word.index = get_target_letter(&word.word);
        text.sections[word.index].style.font_size = 70.0;
        word.typed += 1;
        text.sections[word.index].style.color = if word.letters == word.typed {
            Color::GRAY
        } else {
            Color::BLUE
        };

        let mode_offset: u128 = if game_progress.modes[1].1 { 500 } else { 0 };

        if word.letters == word.typed {
            if word.errors <= 0 + game_progress.modes[0].1 as usize
                && word.timer.elapsed().as_millis()
                    < word.letters as u128 * 600 - game_progress.day as u128 * 15 + mode_offset
            {
                text.sections[word.word.len()].style.color = Color::DARK_GREEN;
                text.sections[word.word.len()].value = format!(
                    " Perfect! Your reward is {} Botcoins.",
                    word.letters * word.letters * 2
                );
                game_progress.money += word.letters * word.letters;
            } else if word.errors > 1 + game_progress.modes[0].1 as usize
                || word.timer.elapsed().as_millis()
                    > word.letters as u128 * 1200 - game_progress.day as u128 * 30 + mode_offset
            {
                text.sections[word.word.len()].style.color = Color::RED;
                text.sections[word.word.len()].value =
                    format!(" Awful! Your reward is 0 Botcoins.");
            } else {
                text.sections[word.word.len()].style.color = Color::YELLOW_GREEN;
                text.sections[word.word.len()].value =
                    format!(" Fine. Your reward is {} Botcoins.", word.letters);
                game_progress.money += word.letters;
            }
            word.marked_to_despawn = true;
            word.started = false;
            word.despawn_timer = Instant::now();
        }
    }
}

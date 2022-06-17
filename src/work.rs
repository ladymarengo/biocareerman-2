use super::*;
use crate::{minigames::{letters::*, words::*}, logging::LogBook};
use bevy::math::Rect;
use info::*;
use loading::*;

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
pub struct Word {
    pub word: String,
    pub index: usize,
    pub errors: usize,
    pub timer: Instant,
    pub started: bool,
    pub marked_to_despawn: bool,
    pub despawn_timer: Instant,
    pub minigame: MiniGame,
    pub letters: usize,
    pub typed: usize,
}

#[derive(Copy, Clone)]
pub enum MiniGame {
    RandomWord,
    RandomLetters,
    RandomWordDisappear,
    Count,
}

#[derive(Component)]
pub struct Redness;

impl Plugin for Work {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(AppState::Work)
                .with_system(spawn_work),
        )
        .insert_resource(DelayTimer(Instant::now()))
        .insert_resource(WorkDayTimer(Instant::now()))
        .add_system_set(
            SystemSet::on_update(AppState::Work)
                .with_system(text_input.label("print"))
                .with_system(spawn_minigame.after("print"))
                .with_system(letter_interpolation)
                .with_system(correct_redness),
        )
        .add_system_set(SystemSet::on_exit(AppState::Work).with_system(cleanup_work));
    }
}

fn spawn_minigame(
    commands: Commands,
    query: Query<Entity, With<Word>>,
    asset_server: Res<AssetServer>,
    game_progress: ResMut<GameProgress>,
    redness: Query<Entity, With<Redness>>,
) {
    let mut rng = ::rand::thread_rng();
    let minigame: i32 = rng.gen_range(0..MiniGame::Count as i32);

    if query.iter().collect::<Vec<Entity>>().len() == 0 {
        match minigame {
            0 => spawn_word(commands, query, asset_server, game_progress, redness, false),
            1 => spawn_letters(commands, asset_server, game_progress, redness),
            2 => spawn_word(commands, query, asset_server, game_progress, redness, true),
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

fn text_input(
    char_evr: EventReader<ReceivedCharacter>,
    mut query: Query<(Entity, &mut Word, &mut Text)>,
    mut commands: Commands,
    game_progress: ResMut<GameProgress>,
    app_state: ResMut<State<AppState>>,
    workdaytimer: Res<WorkDayTimer>,
	logbook: ResMut<LogBook>,
) {
    if !query.is_empty() {
        let (id, word, _text) = query.single_mut();

        if word.marked_to_despawn {
            if word.despawn_timer.elapsed().as_millis() > 1000 {
                commands.entity(id).despawn();
                finish_day(game_progress, workdaytimer, app_state);
            }
            return;
        }

        match word.minigame {
            MiniGame::RandomWord | MiniGame::RandomWordDisappear => {
                input_words(char_evr, query, game_progress, logbook)
            }
            MiniGame::RandomLetters => input_letters(char_evr, query, game_progress, logbook),
            _ => (),
        }
    }
}

fn letter_interpolation(mut query: Query<(&mut Text, &Word)>, _game_progress: Res<GameProgress>) {
    if !query.is_empty() {
        let (mut text, word) = query.single_mut();
        if let MiniGame::RandomLetters = word.minigame {
            if word.started && text.sections[word.index].style.font_size > 50.0 {
                text.sections[word.index].style.font_size -= 0.1;
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

pub fn if_letter_locked(game_progress: &ResMut<GameProgress>, c: char) -> bool {
    (game_progress.modes[2].1 && ("aeuioy".contains(c)))
        || (game_progress.modes[4].1 && ("qe".contains(c)))
}

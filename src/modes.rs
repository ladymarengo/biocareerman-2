use super::*;
use info::*;

pub struct Modes;

#[derive(Component)]
pub struct ModesMarker;

#[derive(Component)]
pub struct ModeIndex(usize);

impl Plugin for Modes {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Modes).with_system(spawn_modes))
            .add_system_set(SystemSet::on_update(AppState::Modes).with_system(modes_input))
            .add_system_set(SystemSet::on_exit(AppState::Modes).with_system(cleanup_modes));
    }
}

fn spawn_modes(
    mut commands: Commands,
    assets: Res<AssetServer>,
    game_progress: ResMut<GameProgress>,
) {
    let boxwidth = WIDTH * 0.3;
    let offsets = vec![
        (-(WIDTH * 0.05 + boxwidth), HEIGHT * 0.25),
        (-(WIDTH * 0.05 + boxwidth), 0.0),
        (-(WIDTH * 0.05 + boxwidth), -HEIGHT * 0.25),
        (WIDTH * 0.05, HEIGHT * 0.25),
        (WIDTH * 0.05, 0.0),
        (WIDTH * 0.05, -HEIGHT * 0.25),
    ];

    for (index, (mode, purchased)) in game_progress.modes.iter().enumerate() {
        let purchase_prompt = format!("Press {} to buy this mode.", index + 1);

        commands
            .spawn_bundle(Text2dBundle {
                text: Text {
                    alignment: TextAlignment {
                        vertical: VerticalAlign::Center,
                        horizontal: HorizontalAlign::Left,
                        ..Default::default()
                    },
                    sections: vec![
                        TextSection {
                            value: format!(
                                "\"{}\"\n{}\nPrice: {}\nHumanness: {}",
                                mode.name, mode.desc, mode.price, mode.humanness_impact
                            ),
                            style: TextStyle {
                                font: assets.load("FiraMono-Medium.ttf"),
                                font_size: 30.0,
                                color: Color::WHITE,
                            },
                        },
                        TextSection {
                            value: format!(
                                "\n{}",
                                if *purchased {
                                    "You already have this mode."
                                } else if mode.price <= game_progress.money {
                                    &purchase_prompt
                                } else {
                                    "You don't have enough money."
                                }
                            ),
                            style: TextStyle {
                                font: assets.load("FiraMono-Medium.ttf"),
                                font_size: 30.0,
                                color: if *purchased {
                                    Color::GREEN
                                } else if mode.price <= game_progress.money {
                                    Color::BLUE
                                } else {
                                    Color::RED
                                },
                            },
                        },
                    ],
                    ..default()
                },
                text_2d_bounds: Text2dBounds {
                    // Wrap text in the rectangle
                    size: Size {
                        width: boxwidth,
                        height: HEIGHT * 0.1,
                    },
                },
                transform: Transform::from_xyz(
                    offsets[index].0 as f32,
                    offsets[index].1 as f32,
                    1.0,
                ),
                ..default()
            })
            .insert(ModeIndex(index))
            .insert(ModesMarker);
    }

    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::Auto,
                position_type: PositionType::Absolute,
                position: Rect {
                    bottom: Val::Px(50.0),
                    left: Val::Px(WIDTH / 2.0 - 230.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text::with_section(
                "Press H to go back home",
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
        .insert(ModesMarker);
}

fn modes_input(
    keys: Res<Input<KeyCode>>,
    mut app_state: ResMut<State<AppState>>,
    mut game_progress: ResMut<GameProgress>,
    mut texts: Query<(&mut Text, &ModeIndex), With<ModesMarker>>,
) {
    if keys.just_pressed(KeyCode::H) {
        app_state.set(AppState::Home).unwrap();
    }
    if keys.just_pressed(KeyCode::Key1)
        && game_progress.modes[0].1 == false
        && game_progress.money >= game_progress.modes[0].0.price
    {
        game_progress.modes[0].1 = true;
        game_progress.money -= game_progress.modes[0].0.price;
        game_progress.humanness += game_progress.modes[0].0.humanness_impact;
    }
    if keys.just_pressed(KeyCode::Key2)
        && game_progress.modes[1].1 == false
        && game_progress.money >= game_progress.modes[1].0.price
    {
        game_progress.modes[1].1 = true;
        game_progress.money -= game_progress.modes[1].0.price;
        game_progress.humanness += game_progress.modes[1].0.humanness_impact;
    }
    if keys.just_pressed(KeyCode::Key3)
        && game_progress.modes[2].1 == false
        && game_progress.money >= game_progress.modes[2].0.price
    {
        game_progress.modes[2].1 = true;
        game_progress.money -= game_progress.modes[2].0.price;
        game_progress.humanness += game_progress.modes[2].0.humanness_impact;
    }
    if keys.just_pressed(KeyCode::Key4)
        && game_progress.modes[3].1 == false
        && game_progress.money >= game_progress.modes[3].0.price
    {
        game_progress.modes[3].1 = true;
        game_progress.money -= game_progress.modes[3].0.price;
        game_progress.humanness += game_progress.modes[3].0.humanness_impact;
    }
    if keys.just_pressed(KeyCode::Key5)
        && game_progress.modes[4].1 == false
        && game_progress.money >= game_progress.modes[4].0.price
    {
        game_progress.modes[4].1 = true;
        game_progress.money -= game_progress.modes[4].0.price;
        game_progress.humanness += game_progress.modes[4].0.humanness_impact;
    }
    if keys.just_pressed(KeyCode::Key6)
        && game_progress.modes[5].1 == false
        && game_progress.money >= game_progress.modes[5].0.price
    {
        game_progress.modes[5].1 = true;
        game_progress.money -= game_progress.modes[5].0.price;
        game_progress.humanness += game_progress.modes[5].0.humanness_impact;
    }

    for (mut text, index) in texts.iter_mut() {
        let purchase_prompt = format!("Press {} to buy this mode.", index.0 + 1);
        text.sections[1].value = format!(
            "\n{}",
            if game_progress.modes[index.0].1 {
                "You already have this mode."
            } else if game_progress.modes[index.0].0.price <= game_progress.money {
                &purchase_prompt
            } else {
                "You don't have enough money."
            }
        );
        text.sections[1].style.color = if game_progress.modes[index.0].1 {
            Color::GREEN
        } else if game_progress.modes[index.0].0.price <= game_progress.money {
            Color::BLUE
        } else {
            Color::RED
        };
    }
}

fn cleanup_modes(mut commands: Commands, query: Query<Entity, With<ModesMarker>>) {
    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}

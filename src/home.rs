use super::*;

pub struct Home;

#[derive(Component)]
pub struct HomeMarker;

impl Plugin for Home {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(AppState::Home)
                .with_system(spawn_home)
                .with_system(hud::spawn_hud),
        )
        .add_system_set(SystemSet::on_update(AppState::Home).with_system(home_input))
        .add_system_set(SystemSet::on_exit(AppState::Home).with_system(cleanup_home));
    }
}

fn spawn_home(
    mut commands: Commands,
    assets: Res<AssetServer>,
    game_progress: Res<GameProgress>,
    load_assets: Res<LoadedAssets>,
	load_fonts: Res<LoadedFonts>,
) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: load_assets.0.get("home_new.png").unwrap().clone(),
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
        .insert(HomeMarker);

    commands
        .spawn_bundle(SpriteBundle {
            texture: load_assets.0.get("bcman_bubble.png").unwrap().clone(),
            transform: Transform {
                translation: Vec3::new(-(WIDTH * 0.45), -(HEIGHT * 0.3), 0.0),
                ..Default::default()
            },
            sprite: Sprite {
                custom_size: Some(Vec2::new(WIDTH * 1.5, HEIGHT * 1.5)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(HomeMarker);

    // commands
    //     .spawn_bundle(TextBundle {
    //         style: Style {
    //             align_self: AlignSelf::Auto,
    //             position_type: PositionType::Absolute,
    //             position: Rect {
    //                 top: Val::Px(200.0),
    //                 left: Val::Px(100.0),
    //                 ..Default::default()
    //             },
    //             ..Default::default()
    //         },
    //         text: Text::with_section(
    //             game_progress.library.news[game_progress.day - 1].clone(),
    //             TextStyle {
    //                 font: assets.load("FiraMono-Medium.ttf"),
    //                 font_size: 30.0,
    //                 color: Color::BLACK,
    //             },
    //             TextAlignment {
    //                 horizontal: HorizontalAlign::Left,
    //                 vertical: VerticalAlign::Center,
    //                 ..Default::default()
    //             },
    //         ),
    //         ..Default::default()
    //     })

	commands.spawn_bundle(Text2dBundle {
		text: Text::with_section(
			game_progress.library.news[game_progress.day - 1].clone(),
			TextStyle {
				font: load_fonts.0.get("FiraMono-Medium.ttf").unwrap().clone(),
				font_size: 30.0,
				color: Color::BLACK,
			},
			TextAlignment {
				vertical: VerticalAlign::Center,
				horizontal: HorizontalAlign::Center,
			}
		),
		text_2d_bounds: Text2dBounds {
			// Wrap text in the rectangle
			size: Size{width: WIDTH * 0.46, height: HEIGHT * 0.15},
		},
		transform: Transform::from_xyz(
			-(WIDTH * 0.215),
			HEIGHT * 0.25,
			1.0,
		),
		..default()
	})
	.insert(HomeMarker);

    if game_progress.humanness < 90 {
        commands
            .spawn_bundle(SpriteBundle {
                texture: load_assets.0.get("eye_mod_home.png").unwrap().clone(),
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
            .insert(HomeMarker);
    }

    if game_progress.modes[1].1 {
        commands
            .spawn_bundle(SpriteBundle {
                texture: load_assets.0.get("smilemod_home.png").unwrap().clone(),
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
            .insert(HomeMarker);
    }

	commands.spawn_bundle(Text2dBundle {
		text: Text::with_section(
			"Press M to buy modes, W to go to work",
			TextStyle {
				font: load_fonts.0.get("FiraMono-Medium.ttf").unwrap().clone(),
				font_size: 40.0,
				color: Color::WHITE,
			},
			TextAlignment {
				vertical: VerticalAlign::Center,
				horizontal: HorizontalAlign::Center,
			}
		),
		text_2d_bounds: Text2dBounds {
			// Wrap text in the rectangle
			size: Size{width: WIDTH * 0.8, height: HEIGHT * 0.1},
		},
		transform: Transform::from_xyz(
			0.0,
			-(HEIGHT / 2.0 - HEIGHT * 0.05),
			1.0,
		),
		..default()
	})
	.insert(HomeMarker);
}

fn home_input(keys: Res<Input<KeyCode>>, mut app_state: ResMut<State<AppState>>) {
    if keys.just_pressed(KeyCode::W) {
        app_state.set(AppState::JobsList).unwrap();
    } else if keys.just_pressed(KeyCode::M) {
        app_state.set(AppState::Modes).unwrap();
    }
}

fn cleanup_home(mut commands: Commands, query: Query<Entity, With<HomeMarker>>) {
    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}

use super::*;
use loading::*;

#[derive(Component)]
pub struct StartMarker;

pub struct Start;

impl Plugin for Start {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(AppState::Start)
                .with_system(spawn_start)
        )
        .add_system_set(
            SystemSet::on_update(AppState::Start)
                .with_system(start_input)
        )
        .add_system_set(SystemSet::on_exit(AppState::Start).with_system(cleanup_start));
    }
}

fn spawn_start(
    mut commands: Commands,
    load_assets: Res<LoadedAssets>,
	load_fonts: Res<LoadedFonts>,
) {

    commands
        .spawn_bundle(SpriteBundle {
            texture: load_assets.0.get("logo.png").unwrap().clone(),
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
        .insert(StartMarker);

	commands.spawn_bundle(Text2dBundle {
        text: Text::with_section(
            "Press S to start",
            TextStyle {
				font: load_fonts.0.get("FiraMono-Medium.ttf").unwrap().clone(),
				font_size: 60.0,
				color: Color::WHITE,
			},
            TextAlignment {
				vertical: VerticalAlign::Center,
				horizontal: HorizontalAlign::Center,
			}
        ),
        text_2d_bounds: Text2dBounds {
            // Wrap text in the rectangle
            size: Size{width: WIDTH * 0.4, height: HEIGHT * 0.1},
        },
        transform: Transform::from_xyz(
            0.0,
            -(HEIGHT / 2.0 - HEIGHT * 0.07),
            1.0,
        ),
        ..default()
	})
        .insert(StartMarker);
}

fn cleanup_start(mut commands: Commands, query: Query<Entity, With<StartMarker>>) {
    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}

fn start_input(keys: Res<Input<KeyCode>>, mut app_state: ResMut<State<AppState>>) {
    if keys.just_pressed(KeyCode::S) {
        app_state.set(AppState::Home).unwrap();
    }
}
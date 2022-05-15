use bevy::prelude::*;
use super::*;
use crate::hud::*;

pub struct Home;

#[derive(Component)]
pub struct HomeMarker;

impl Plugin for Home {
    fn build(&self, app: &mut App) {
        app
        .add_system_set(
            SystemSet::on_enter(AppState::Home)
                .with_system(spawn_home)
				.with_system(hud::spawn_hud)
        )
		.add_system_set(
            SystemSet::on_update(AppState::Home)
                .with_system(home_input)
        )
		.add_system_set(
			SystemSet::on_exit(AppState::Home)
				.with_system(cleanup_home));
		}
	}

fn spawn_home(mut commands: Commands, assets: Res<AssetServer>, game_progress: Res<GameProgress>) {
	commands
        .spawn_bundle(SpriteBundle {
            texture: assets.load("home_new.png"),
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
            texture: assets.load("bcman_bubble.png"),
            transform: Transform {
                translation: Vec3::new(-700.0, -350.0, 0.0),
                ..Default::default()
            },
            sprite: Sprite {
                custom_size: Some(Vec2::new(WIDTH * 1.5, HEIGHT * 1.5)),
                ..Default::default()
            },
            ..Default::default()
        })
		.insert(HomeMarker);

	commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::Auto,
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(200.0),
                    left: Val::Px(100.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text::with_section(
                game_progress.library.news[game_progress.day - 1].clone(),
                TextStyle {
                    font: assets.load("FiraMono-Medium.ttf"),
                    font_size: 30.0,
                    color: Color::BLACK,
                },
                TextAlignment {
                    horizontal: HorizontalAlign::Left,
                    vertical: VerticalAlign::Center,
                    ..Default::default()
                },
            ),
            ..Default::default()
        })
		.insert(HomeMarker);
}

fn home_input(keys: Res<Input<KeyCode>>, mut app_state: ResMut<State<AppState>>)
{
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

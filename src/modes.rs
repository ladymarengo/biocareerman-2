use bevy::prelude::*;
use super::*;

pub struct Modes;

#[derive(Component)]
pub struct ModesMarker;

impl Plugin for Modes {
    fn build(&self, app: &mut App) {
        app
        .add_system_set(
            SystemSet::on_enter(AppState::Modes)
                .with_system(spawn_modes)
        )
		.add_system_set(
            SystemSet::on_update(AppState::Modes)
                .with_system(modes_input)
        )
		.add_system_set(
			SystemSet::on_exit(AppState::Modes)
				.with_system(cleanup_modes));
		}
	}

fn spawn_modes(mut commands: Commands, assets: Res<AssetServer>, mut game_progress: ResMut<GameProgress>) {

	let top: f32 = 100.0;
    let left: f32 = 200.0;

	commands
        .spawn_bundle(SpriteBundle {
            texture: assets.load("modes.png"),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                ..Default::default()
            },
            sprite: Sprite {
                custom_size: Some(Vec2::new(800.0, 600.0)),
                ..Default::default()
            },
            ..Default::default()
        })
		.insert(ModesMarker);

	for (index, (mode, purchased)) in game_progress.modes.iter().enumerate() {
		commands
			.spawn_bundle(TextBundle {
				style: Style {
					align_self: AlignSelf::Auto,
					position_type: PositionType::Absolute,
					position: Rect {
						top: Val::Px(top + index as f32 * 100.0),
						left: Val::Px(left),
						..Default::default()
					},
					..Default::default()
				},
				text: Text::with_section(
					format!("{}\n{}\nPrice: {}\nHumanness: {}", mode.name, mode.desc, mode.price, mode.humanness_impact),
					TextStyle {
						font: assets.load("FiraMono-Medium.ttf"),
						font_size: 30.0,
						color: if *purchased {Color::GREEN} else if mode.price < game_progress.money {Color::WHITE} else {Color::RED} ,
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
}

fn modes_input(keys: Res<Input<KeyCode>>, mut app_state: ResMut<State<AppState>>)
{
	if keys.just_pressed(KeyCode::H) {
		app_state.set(AppState::Home).unwrap();
	}
}

fn cleanup_modes(mut commands: Commands, query: Query<Entity, With<ModesMarker>>) {
    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}
use super::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct HudMarker;

#[derive(Component)]
pub struct Money;

#[derive(Component)]
pub struct Humanness;

#[derive(Component)]
pub struct Day;

pub fn spawn_hud(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(UiCameraBundle::default());

    let top: f32 = 20.0;
    let left: f32 = 200.0;

    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::Auto,
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(top),
                    left: Val::Px(left),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text::with_section(
                "Money ",
                TextStyle {
                    font: asset_server.load("FiraMono-Medium.ttf"),
                    font_size: 30.0,
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
        .insert(Money)
        .insert(HudMarker);

    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::Auto,
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(top),
                    left: Val::Px(left + 200.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text::with_section(
                "Humanness ",
                TextStyle {
                    font: asset_server.load("FiraMono-Medium.ttf"),
                    font_size: 30.0,
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
        .insert(Humanness)
        .insert(HudMarker);

    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::Auto,
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(top),
                    left: Val::Px(left + 450.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text::with_section(
                "Day 1/20",
                TextStyle {
                    font: asset_server.load("FiraMono-Medium.ttf"),
                    font_size: 30.0,
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
        .insert(Day)
        .insert(HudMarker);
}

pub fn update_hud(
    mut texts: Query<(
        &mut Text,
        Option<&Money>,
        Option<&Humanness>,
        Option<&Day>,
    ), With<HudMarker>>,
	game_progress: Res<GameProgress>
) {
	if !texts.is_empty() {
		for (mut text, if_money, if_humanness, if_day) in texts.iter_mut() {
			if let Some(_t) = if_money {
				text.sections[0].value = format!("Money: {}", game_progress.money);
			}
			if let Some(_t) = if_humanness {
				text.sections[0].value = format!("Humanness: {}", game_progress.humanness);
			}
			if let Some(_t) = if_day {
				text.sections[0].value = format!("Day {}/{}", game_progress.day, game_progress.max_days);
			}
		}
	}
}

pub fn cleanup_hud(mut commands: Commands, query: Query<Entity, With<HudMarker>>) {
    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}

use bevy::prelude::*;
use super::*;

pub struct Work;

#[derive(Component)]
pub struct WorkMarker;

#[derive(Component)]
struct Word {
	word: String,
	index: usize
}

impl Plugin for Work {
    fn build(&self, app: &mut App) {
        app
        .add_system_set(
            SystemSet::on_enter(AppState::Work)
                .with_system(spawn_work)
        )
		.add_system_set(
			SystemSet::on_update(AppState::Work)
                .with_system(spawn_word)
		)
		.add_system_set(
			SystemSet::on_exit(AppState::Work)
				.with_system(cleanup_work));
		}
	}

fn spawn_work(mut commands: Commands) {
	commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

	println!("Work");

    commands.spawn_bundle(SpriteBundle {
		transform: Transform {
			translation: Vec3::new(0.0, 0.0, 2.0),
			..Default::default()
		},
        sprite: Sprite {
            color: Color::rgb(0.25, 0.0, 0.75),
            custom_size: Some(Vec2::new(50.0, 100.0)),
            ..default()
        },
        ..default()
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
	asset_server: Res<AssetServer>
) {
	if query.is_empty() {
		commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                ..default()
            },
            // Use `Text` directly
            text: Text {
                // Construct a `Vec` of `TextSection`s
                sections: vectorize_word("hello".to_string(), asset_server),
                ..default()
            },
            ..default()
        })
        .insert(Word{word: "hello".to_string(), index: 0})
		.insert(WorkMarker);
	}
}

fn vectorize_word(word: String, asset_server: Res<AssetServer>) -> Vec<TextSection> {
	let mut sections = Vec::new();

	for character in word.chars() {
		sections.push(
			TextSection {
				value: character.to_string(),
				style: TextStyle {
					font: asset_server.load("FiraMono-Medium.ttf"),
					font_size: 60.0,
					color: Color::GOLD,
				},
			}
		)
	}
	sections
}

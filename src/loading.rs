use super::*;

pub struct Loading;

#[derive(Component)]
pub struct LoadingMarker;

impl Plugin for Loading {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(AppState::Loading)
                .with_system(spawn_loading_screen)
                .with_system(load_assets)
                .with_system(load_font),
        )
        .add_system_set(SystemSet::on_update(AppState::Loading).with_system(check_assets_ready))
        .add_system_set(
            SystemSet::on_update(AppState::Loading)
                .with_run_criteria(FixedTimestep::step(TIMESTEP))
                .with_system(update_loading_screen),
        )
        .add_system_set(SystemSet::on_exit(AppState::Loading).with_system(cleanup_loading));
    }
}

fn load_font(mut fonts: ResMut<LoadedFonts>, asset_server: Res<AssetServer>) {
    fonts.0.insert(
        "FiraMono-Medium.ttf".to_string(),
        asset_server.load(&"FiraMono-Medium.ttf".to_string()),
    );
}

fn load_assets(
    mut assets: ResMut<LoadedAssets>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    mut loading: ResMut<AssetsLoading>,
) {
    let names = [
        "logo.png",
        "home_new.png",
        "bcman_bubble.png",
        "work_new.png",
        "customer_bubble.png",
        "customer_color.png",
        "customer_face_1.png",
        "customer_mask.png",
        "customer_redness.png",
        "eye_mod_work.png",
        "eye_mod_home.png",
        "smilemod_work.png",
        "smilemod_home.png",
        "Bahamas.png",
        "dumpster.png",
        "newfarm.png",
    ];

    for name in names {
        let handle = asset_server.load(name);
        assets.0.insert(name.to_string(), handle.clone());
        loading.0.push(handle.clone_untyped());
    }
    let music = asset_server.load("sounds/100_humanlong.ogg");
    audio.play(music);
}

fn check_assets_ready(
    server: Res<AssetServer>,
    loading: Res<AssetsLoading>,
    mut app_state: ResMut<State<AppState>>,
) {
    use bevy::asset::LoadState;

    if server.get_group_load_state(loading.0.iter().map(|h| h.id)) == LoadState::Loaded {
        app_state.set(AppState::Start).unwrap();
    }
}

fn spawn_loading_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(Text2dBundle {
            text: Text::with_section(
                "Loading.",
                TextStyle {
                    font: asset_server.load("FiraMono-Medium.ttf"),
                    font_size: 70.0,
                    color: Color::WHITE,
                },
                TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Center,
                },
            ),
            text_2d_bounds: Text2dBounds {
                size: Size {
                    width: WIDTH * 0.4,
                    height: HEIGHT * 0.1,
                },
            },
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            ..default()
        })
        .insert(LoadingMarker);
}

fn update_loading_screen(
    mut text: Query<&mut Text, With<LoadingMarker>>,
    mut index: ResMut<LoadingIndex>,
) {
    let dots = vec!["", ".", "..", "..."];

    if !text.is_empty() {
        let mut text = text.single_mut();
        text.sections[0].value = format!("Loading{}", dots[index.0]);
        index.0 += 1;
        if index.0 > 3 {
            index.0 = 0;
        }
    }
}

fn cleanup_loading(mut commands: Commands, query: Query<Entity, With<LoadingMarker>>) {
    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}

pub struct LoadedAssets(pub HashMap<String, Handle<Image>>);

pub struct LoadedFonts(pub HashMap<String, Handle<Font>>);

pub struct AssetsLoading(pub Vec<HandleUntyped>);

#[derive(Component)]
pub struct LoadingIndex(pub usize);

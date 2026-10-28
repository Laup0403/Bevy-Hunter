use crate::enemy::{Enemy, MaxEnemies};
use bevy::{
    color::palettes::css::{BLACK, GOLD, GRAY},
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

pub struct Gamemanager;

impl Plugin for Gamemanager {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_systems(PreStartup, load_assets)
            .add_systems(Startup, load_ui)
            .add_systems(Update, (update_fps, update_birds, update_max))
            .add_systems(Update, switch_state.run_if(in_state(GameState::InGame)));
    }
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    Menu,
    InGame,
}

#[derive(Resource)]
pub struct GameAssets {
    pub player: Handle<Image>,
    pub gun: Handle<Image>,
    pub bullet: Handle<Image>,
    pub bird1: Handle<Image>,
    pub bird2: Handle<Image>,
    pub bird3: Handle<Image>,
    pub bird4: Handle<Image>,
}

fn load_assets(mut commands: Commands, assets: Res<AssetServer>) {
    let game_assets = GameAssets {
        player: assets.load("player.png"),
        gun: assets.load("gun.png"),
        bullet: assets.load("bullet.png"),
        bird1: assets.load("bird1.png"),
        bird2: assets.load("bird2.png"),
        bird3: assets.load("bird3.png"),
        bird4: assets.load("bird4.png"),
    };
    commands.insert_resource(game_assets);
}

const BG_COLOR: Srgba = GRAY;
const TEXT_COLOR: Srgba = GOLD;
const NUMBER_COLOR: Srgba = BLACK;

#[derive(Component)]
struct FPSCounter;

#[derive(Component)]
struct BirdCounter;

#[derive(Component)]
struct MaxBirds;

fn load_ui(mut commands: Commands) {
    commands
        .spawn((
            Text::new("FPS:"),
            TextColor(TEXT_COLOR.into()),
            TextBackgroundColor(BG_COLOR.into()),
        ))
        .with_child((
            TextSpan::default(),
            FPSCounter,
            TextColor(NUMBER_COLOR.into()),
            TextBackgroundColor(BG_COLOR.into()),
        ));
    commands
        .spawn((
            Text::new("\nBirds: "),
            TextColor(TEXT_COLOR.into()),
            TextBackgroundColor(BG_COLOR.into()),
        ))
        .with_child((
            TextSpan::default(),
            BirdCounter,
            TextColor(NUMBER_COLOR.into()),
            TextBackgroundColor(BG_COLOR.into()),
        ));
    commands
        .spawn((
            Text::new("\n\nMax Birds: "),
            TextColor(TEXT_COLOR.into()),
            TextBackgroundColor(BG_COLOR.into()),
        ))
        .with_child((
            MaxBirds,
            TextSpan::default(),
            TextColor(NUMBER_COLOR.into()),
            TextBackgroundColor(BG_COLOR.into()),
        ));
}

fn update_fps(
    diagnostic: Res<DiagnosticsStore>,
    mut query: Query<&mut TextSpan, With<FPSCounter>>,
) {
    for mut span in &mut query {
        if let Some(fps) = diagnostic.get(&FrameTimeDiagnosticsPlugin::FPS)
            && let Some(value) = fps.smoothed()
        {
            **span = format!("{value:.0}");
        }
    }
}

fn update_birds(birds: Query<&Enemy>, mut query: Query<&mut TextSpan, With<BirdCounter>>) {
    let mut num: usize = 0;
    for _ in birds {
        num += 1;
    }

    for mut span in &mut query {
        **span = format!("{num}");
    }
}

fn update_max(mut query: Query<&mut TextSpan, With<MaxBirds>>, max_enemies: Res<MaxEnemies>) {
    for mut span in &mut query {
        **span = format!("{}", max_enemies.0.to_string());
    }
}

fn switch_state(mut commands: Commands, kb: Res<ButtonInput<KeyCode>>) {
    if kb.just_pressed(KeyCode::Tab) {
        commands.set_state(GameState::Menu);
    }
}

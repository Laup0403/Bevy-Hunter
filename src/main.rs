use bevy::prelude::*;

mod player;
use player::PlayerPlugin;
mod shoot;
use shoot::ShootPlugin;
mod enemy;
use enemy::EnemyPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.45, 0.57, 0.2145)))
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(PreStartup, load_assets)
        .add_plugins((PlayerPlugin, ShootPlugin, EnemyPlugin))
        .run();
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

use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};

mod gamemanager;
use gamemanager::Gamemanager;
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
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins((Gamemanager, PlayerPlugin, ShootPlugin, EnemyPlugin))
        .run();
}

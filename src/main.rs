use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod player;
use player::PlayerPlugin;
mod shoot;
use shoot::ShootPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((PlayerPlugin, ShootPlugin))
        .add_systems(Startup, spawn_map)
        .run();
}

fn spawn_map(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn(Sprite::from_image(assets.load("Map.png")));
    commands.spawn(Collider::cuboid(5., 5.))
}

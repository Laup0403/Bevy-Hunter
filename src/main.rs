use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod player;
use player::PlayerPlugin;
mod shoot;
use shoot::ShootPlugin;
mod enemy;
use enemy::EnemyPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.45, 0.57, 0.2145)))
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin {
            mode: DebugRenderMode::COLLIDER_SHAPES,
            ..default()
        })
        .add_plugins((PlayerPlugin, ShootPlugin, EnemyPlugin))
        .run();
}

// Z Layers //
// 1 Zombies
// 2 Bullet
// 3 Player

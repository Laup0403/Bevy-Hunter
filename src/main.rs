use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod player;
use player::PlayerPlugin;
mod shoot;
use shoot::ShootPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin {
            mode: DebugRenderMode::COLLIDER_SHAPES,
            ..default()
        })
        .add_plugins((PlayerPlugin, ShootPlugin))
        .add_systems(Startup, spawn_map)
        .run();
}

// Collision groups
pub const WALLS: Group = Group::GROUP_1;
pub const PLAYER: Group = Group::GROUP_2;

fn spawn_map(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn(Sprite::from_image(assets.load("Map.png")));

    // Wall left
    commands.spawn((
        RigidBody::Fixed,
        Collider::cuboid(1., 640.),
        // I am group WALL i coolide with group PLAYER
        CollisionGroups::new(WALLS, PLAYER),
        Transform::from_xyz(-961., 0., 0.),
    ));
    // Wall right
    commands.spawn((
        RigidBody::Fixed,
        Collider::cuboid(1., 640.),
        CollisionGroups::new(WALLS, PLAYER),
        Transform::from_xyz(961., 0., 0.),
    ));
    // Wall top
    commands.spawn((
        RigidBody::Fixed,
        Collider::cuboid(960., 1.),
        CollisionGroups::new(WALLS, PLAYER),
        Transform::from_xyz(0., 641., 0.),
    ));
    // Wall bottom
    commands.spawn((
        RigidBody::Fixed,
        Collider::cuboid(960., 1.),
        CollisionGroups::new(WALLS, PLAYER),
        Transform::from_xyz(0., -641., 0.),
    ));
}

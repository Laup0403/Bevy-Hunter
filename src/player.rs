use bevy::{input::mouse::MouseWheel, prelude::*};
use std::f32::consts::FRAC_PI_2;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player)
            .add_systems(Update, (move_player, update_camera).chain());
    }
}

// Player //

#[derive(Component)]
pub struct Player;

const PLAYERSPEED: f32 = 150.;

fn setup_player(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn((
        Player,
        Transform::from_xyz(0., 0., 3.),
        Sprite::from_image(assets.load("player.png")),
    ));

    commands.spawn((Camera2d, Transform::from_scale(Vec3::splat(0.5))));
}

fn move_player(
    mut player: Single<&mut Transform, With<Player>>,
    time: Res<Time>,
    kb: Res<ButtonInput<KeyCode>>,

    window: Single<&Window>,
    camera_query: Single<(&Camera, &GlobalTransform)>,
) {
    let mut input = Vec2::ZERO;

    if kb.pressed(KeyCode::KeyW) {
        input.y += 1.;
    }
    if kb.pressed(KeyCode::KeyS) {
        input.y -= 1.;
    }
    if kb.pressed(KeyCode::KeyA) {
        input.x -= 1.;
    }
    if kb.pressed(KeyCode::KeyD) {
        input.x += 1.;
    }

    player.translation += (input.normalize_or_zero() * PLAYERSPEED * time.delta_secs()).extend(3.);

    // Rotation
    let (camera, camera_transform) = *camera_query;

    if let Some(cursor_position) = window.cursor_position()
        && let Ok(cursor_world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_position)
    {
        player.rotation = Quat::from_rotation_z(
            (cursor_world_pos - player.translation.xy()).to_angle() - FRAC_PI_2,
        );
    }
}

const CAMERA_DECAY_RATE: f32 = 4.;

fn update_camera(
    mut camera: Single<&mut Transform, (With<Camera2d>, Without<Player>)>,
    player: Single<&Transform, (With<Player>, Without<Camera2d>)>,
    time: Res<Time>,
    mut scroll_event: EventReader<MouseWheel>,
) {
    let Vec3 { x, y, .. } = player.translation;
    let direction = Vec3::new(x, y, camera.translation.z);

    camera
        .translation
        .smooth_nudge(&direction, CAMERA_DECAY_RATE, time.delta_secs());

    for event in scroll_event.read() {
        camera.scale += Vec3::splat(event.y * -0.2);
        camera.scale = Vec3::splat(camera.scale.x.clamp(0.5, 100.));
    }
}

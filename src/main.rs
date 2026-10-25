use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, setup_player)
        .add_systems(Update, move_player)
        .run();
}

#[derive(Component)]
pub struct Player;

const PLAYERSPEED: f32 = 50.;

fn setup_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Player,
        Transform::from_xyz(0., 0., 1.),
        Mesh2d(meshes.add(Rectangle::new(20., 20.))),
        MeshMaterial2d(materials.add(Color::srgb(0., 0., 1.))),
    ));

    commands.spawn(Camera2d);
}

fn move_player(
    mut player: Single<&mut Transform, With<Player>>,
    time: Res<Time>,
    kb: Res<ButtonInput<KeyCode>>,
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

    player.translation += (input.normalize_or_zero() * PLAYERSPEED * time.delta_secs()).extend(0.);
}

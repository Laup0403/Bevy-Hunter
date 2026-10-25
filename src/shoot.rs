use crate::player::Player;
use bevy::prelude::*;

pub struct ShootPlugin;

impl Plugin for ShootPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ShootEvent>()
            .add_systems(Update, (shoot, bullet_spawner, move_bullet));
    }
}

// Der component
#[derive(Component)]
pub struct Bullet {
    velocity: Vec2,
}

// Das event wenn gefeuert wurde
#[derive(Event)]
struct ShootEvent;

fn bullet_spawner(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,

    player_query: Query<&Transform, With<Player>>,
    mut shoot_event: EventReader<ShootEvent>,
) {
    // Wenn shoot event kommt
    for _ in shoot_event.read() {
        for transform in &player_query {
            let dir = transform.up().truncate().normalize();
            let spawn = transform.translation.truncate() + dir * 20.0;

            commands.spawn((
                Mesh2d(meshes.add(Circle::new(4.0))),
                MeshMaterial2d(materials.add(Color::srgb(1., 1., 0.))),
                Transform::from_xyz(spawn.x, spawn.y, 1.0),
                Bullet {
                    velocity: dir * 600.,
                },
            ));
        }
    }
}

fn shoot(mouse: Res<ButtonInput<MouseButton>>, mut shoot_event: EventWriter<ShootEvent>) {
    if mouse.just_pressed(MouseButton::Left) {
        shoot_event.send(ShootEvent);
    }
}

fn move_bullet(time: Res<Time>, mut query: Query<(&mut Transform, &Bullet)>) {
    for (mut transform, bullet) in &mut query {
        transform.translation.x += bullet.velocity.x * time.delta_secs();
        transform.translation.y += bullet.velocity.y * time.delta_secs();
    }
}

use crate::gamemanager::GameAssets;
use crate::{enemy::Enemy, player::Gun, player::Player};
use bevy::prelude::*;

pub struct ShootPlugin;

impl Plugin for ShootPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (bullet_spawner, move_bullet, check_bullet));
    }
}

// Der component
#[derive(Component)]
pub struct Bullet {
    velocity: Vec2,
}

fn bullet_spawner(
    mut commands: Commands,
    assets: Res<GameAssets>,
    gun_query: Query<&Transform, With<Gun>>,
    mouse: Res<ButtonInput<MouseButton>>,
) {
    if mouse.pressed(MouseButton::Left) {
        for transform in &gun_query {
            let dir = transform.up().truncate().normalize();
            let spawn = transform.translation.truncate() + dir * 2.0;

            commands.spawn((
                Sprite::from_image(assets.bullet.clone()),
                Transform::from_xyz(spawn.x + 1., spawn.y, 1.0),
                Bullet {
                    velocity: dir * 400.,
                },
            ));
        }
    }
}

fn move_bullet(time: Res<Time>, mut query: Query<(&mut Transform, &Bullet)>) {
    for (mut transform, bullet) in &mut query {
        transform.translation.x += bullet.velocity.x * time.delta_secs();
        transform.translation.y += bullet.velocity.y * time.delta_secs();
    }
}

pub fn check_bullet(
    bullet_query: Query<(Entity, &Transform), (With<Bullet>, Without<Player>)>,
    mut enemy_query: Query<
        (Entity, &mut Transform),
        (With<Enemy>, Without<Bullet>, Without<Player>),
    >,
    mut commands: Commands,
    player: Single<&Transform, With<Player>>,
) {
    // alle bullets sammeln in einem vec aus dem enitiy und dem transform
    let bullets: Vec<(Entity, Vec2)> = bullet_query
        .iter()
        .map(|(entity, tf)| (entity, tf.translation.xy()))
        .collect();

    // Wenn es keine bullets gibt abbrechen
    if bullets.is_empty() {
        return;
    }

    // benutze kugeln nicht mehrmals treffen lassen
    let mut used_bullets = std::collections::HashSet::<Entity>::new();

    for (enemy, enemy_trans) in &mut enemy_query {
        let enemy_pos = enemy_trans.translation.xy();
        let mut is_dead = false;

        for &(bullet, bullet_pos) in &bullets {
            //benutzte kugel überspringen.
            if used_bullets.contains(&bullet) {
                continue;
            }

            // checkt ob die distanz zwischen zwei punkten (bullet_pos, enemy_pos) kleiner als 20 ist
            if bullet_pos.distance_squared(enemy_pos) < 20. {
                // als tot kugel makieren
                used_bullets.insert(bullet);
                commands.entity(bullet).despawn();
                is_dead = true;
                break;
            }
        }

        // if is dead despawn
        if is_dead {
            commands.entity(enemy).despawn();
        }
    }

    for &(bullet, pos) in &bullets {
        if pos.x >= player.translation.x + 400.
            || pos.x <= player.translation.x - 400.
            || pos.y >= player.translation.y + 400.
            || pos.y <= player.translation.y - 400.
        {
            commands.entity(bullet).despawn();
        }
    }
}

use crate::{player::Player, shoot::Bullet};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EnemyEvent>()
            .add_systems(Update, (spawn_enemy, move_enemy, kill_enemy));
    }
}

// Siehe Shoot und Player

#[derive(Component)]
struct Enemy {
    health: f32,
}

#[derive(Event)]
struct EnemyEvent;

const ENEMYSPEED: f32 = 40.;

fn spawn_enemy(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut enemy_event: EventReader<EnemyEvent>,
) {
    for _ in enemy_event.read() {
        commands.spawn((
            Enemy { health: 20. },
            Transform::from_xyz(0., 0., 1.0),
            Sprite::from_image(assets.load("zombie.png")),
            RigidBody::KinematicPositionBased,
            KinematicCharacterController::default(),
            Collider::ball(20.),
            ActiveEvents::COLLISION_EVENTS,
            Sensor::default(),
        ));
    }
}

fn move_enemy(
    mut enemy_query: Query<
        (&mut Transform, &mut KinematicCharacterController),
        (With<Enemy>, Without<Player>),
    >,
    player_query: Single<&Transform, With<Player>>,
    kb: Res<ButtonInput<KeyCode>>,
    mut enemy_event: EventWriter<EnemyEvent>,
    time: Res<Time>,
) {
    if kb.just_pressed(KeyCode::Space) {
        enemy_event.send(EnemyEvent);
    }

    let player_translation = player_query.translation.xy();

    for (mut transform, mut controller) in &mut enemy_query {
        // Rotate to player
        let to_player = (player_translation - transform.translation.xy()).normalize();
        let rotate_to_player = Quat::from_rotation_arc(Vec3::Y, to_player.extend(0.));
        transform.rotation = rotate_to_player;

        // Move to player
        controller.translation = Some(to_player * ENEMYSPEED * time.delta_secs());
    }
}

fn kill_enemy(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    bullet_query: Query<&Bullet>,
    mut enemy_query: Query<&mut Enemy>,
) {
    for event in collision_events.read() {
        // Checken ob collision startet
        if let CollisionEvent::Started(entity1, entity2, _flags) = event {
            // Schauen ob bullet und enemy collidiert sind
            let (bullet, enemy) =
                if bullet_query.contains(*entity1) && enemy_query.contains(*entity2) {
                    (*entity1, *entity2)
                } else if bullet_query.contains(*entity2) && enemy_query.contains(*entity1) {
                    (*entity2, *entity1)
                } else {
                    continue;
                };

            if let Some(entity_commands) = commands.get_entity(bullet) {
                entity_commands.despawn_recursive();
            }
            if let Ok(mut enemy_obj) = enemy_query.get_mut(enemy) {
                enemy_obj.health -= 10.;

                if enemy_obj.health <= 0.
                    && let Some(entity_commands) = commands.get_entity(enemy)
                {
                    entity_commands.despawn_recursive();
                }
            }
        }
    }
}

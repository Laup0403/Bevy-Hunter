use crate::gamemanager::GameAssets;
use crate::player::Player;
use bevy::prelude::*;
use rand::random_range;
use std::f32::consts::TAU;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<SpawnEnemy>()
            .add_systems(Startup, spawn_all_enemies)
            .add_systems(Update, (spawn_enemy, move_enemy, enemy_respawn));
    }
}

// Siehe Shoot und Player

#[derive(Component)]
pub struct Enemy;

#[derive(Message)]
struct SpawnEnemy {
    position: Vec2,
}

const ENEMYSPEED: f32 = 10.;
pub const MAX_ENEMIES: usize = 100000;
// Spawnes per frame
const SPF: usize = 20;

fn spawn_enemy(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut enemy_message: MessageReader<SpawnEnemy>,
) {
    let random: i32 = random_range(1..=4);
    let texture = match random {
        1 => assets.bird1.clone(),
        2 => assets.bird2.clone(),
        3 => assets.bird3.clone(),
        _ => assets.bird4.clone(), // 4 und alle anderen weil random noch nicht bestimmt ist
    };
    for msg in enemy_message.read() {
        commands.spawn((
            Enemy,
            Transform::from_xyz(msg.position.x, msg.position.y, 1.0),
            Sprite::from_image(texture.clone()),
        ));
    }
}

fn move_enemy(
    mut enemy_query: Query<&mut Transform, (With<Enemy>, Without<Player>)>,
    player_query: Single<&Transform, With<Player>>,
    kb: Res<ButtonInput<KeyCode>>,
    mut enemy_message: MessageWriter<SpawnEnemy>,
    time: Res<Time>,
) {
    if kb.just_pressed(KeyCode::Space) {
        enemy_message.write(SpawnEnemy {
            position: Vec2 { x: 0., y: 0. },
        });
    }

    for mut enemy_transform in &mut enemy_query {
        let to_player =
            (player_query.translation.xy() - enemy_transform.translation.xy()).normalize();
        enemy_transform.translation.x += to_player.x * ENEMYSPEED * time.delta_secs();
        enemy_transform.translation.y += to_player.y * ENEMYSPEED * time.delta_secs();
    }
}

fn enemy_respawn(
    mut enemy_message: MessageWriter<SpawnEnemy>,
    enemies: Query<&Transform, (With<Enemy>, Without<Player>)>,
    player: Single<&Transform, With<Player>>,
) {
    let num_enemies: usize = enemies.iter().len();

    if num_enemies >= MAX_ENEMIES - SPF {
        return;
    }

    for _ in 0..SPF {
        let angle = random_range(0.0..TAU);
        let distance = random_range(50.0..700.0);

        let spawn =
            player.translation.truncate() + (Vec2::new(angle.cos(), angle.sin()) * distance);

        enemy_message.write(SpawnEnemy {
            position: Vec2 {
                x: spawn.x,
                y: spawn.y,
            },
        });
    }
}

fn spawn_all_enemies(
    mut enemy_message: MessageWriter<SpawnEnemy>,
    player: Single<&Transform, With<Player>>,
) {
    for _ in 0..MAX_ENEMIES {
        let angle = random_range(0.0..TAU);
        let distance = random_range(50.0..700.0);

        let spawn =
            player.translation.truncate() + (Vec2::new(angle.cos(), angle.sin()) * distance);

        enemy_message.write(SpawnEnemy {
            position: Vec2 {
                x: spawn.x,
                y: spawn.y,
            },
        });
    }
}

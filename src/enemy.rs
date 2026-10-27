use crate::GameAssets;
use crate::player::Player;
use bevy::prelude::*;
use rand::random_range;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<SpawnEnemy>()
            .add_systems(Update, (spawn_enemy, move_enemy));
    }
}

// Siehe Shoot und Player

#[derive(Component)]
pub struct Enemy;

#[derive(Message)]
struct SpawnEnemy;

const ENEMYSPEED: f32 = 40.;

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
    for _ in enemy_message.read() {
        commands.spawn((
            Enemy,
            Transform::from_xyz(0., 0., 1.0),
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
    if kb.pressed(KeyCode::Space) {
        enemy_message.write(SpawnEnemy);
    }

    let mut num = 0;

    for mut enemy_transform in &mut enemy_query {
        let to_player =
            (player_query.translation.xy() - enemy_transform.translation.xy()).normalize();
        enemy_transform.translation.x += to_player.x * ENEMYSPEED * time.delta_secs();
        enemy_transform.translation.y += to_player.y * ENEMYSPEED * time.delta_secs();
        num += 1;
    }
    println!("Enemies: {}", num.to_string());
}

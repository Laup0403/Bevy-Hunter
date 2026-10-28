use crate::gamemanager::{GameAssets, GameState};
use crate::player::Player;
use bevy::{input::mouse::MouseWheel, prelude::*};
use rand::random_range;
use std::f32::consts::TAU;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MaxEnemies(0)).add_systems(
            Update,
            (move_enemy, enemy_respawn, change_max).run_if(in_state(GameState::InGame)),
        );
    }
}

// Siehe Shoot und Player

#[derive(Component)]
pub struct Enemy;

const ENEMYSPEED: f32 = 10.;
#[derive(Resource)]
pub struct MaxEnemies(pub usize);

fn spawn_enemy(position: Vec2, commands: &mut Commands, assets: &Res<GameAssets>) {
    let random: i32 = random_range(1..=4);
    let texture = match random {
        1 => assets.bird1.clone(),
        2 => assets.bird2.clone(),
        3 => assets.bird3.clone(),
        _ => assets.bird4.clone(), // 4 und alle anderen weil random noch nicht bestimmt ist
    };
    commands.spawn((
        Enemy,
        Transform::from_xyz(position.x, position.y, 1.0),
        Sprite::from_image(texture.clone()),
    ));
}

fn move_enemy(
    mut enemy_query: Query<&mut Transform, (With<Enemy>, Without<Player>)>,
    player_query: Single<&Transform, With<Player>>,
    time: Res<Time>,
) {
    for mut enemy_transform in &mut enemy_query {
        let to_player =
            (player_query.translation.xy() - enemy_transform.translation.xy()).normalize();
        enemy_transform.translation.x += to_player.x * ENEMYSPEED * time.delta_secs();
        enemy_transform.translation.y += to_player.y * ENEMYSPEED * time.delta_secs();
    }
}

fn enemy_respawn(
    enemies: Query<&Transform, (With<Enemy>, Without<Player>)>,
    player: Single<&Transform, With<Player>>,
    mut commands: Commands,
    assets: Res<GameAssets>,
    max_enemies: Res<MaxEnemies>,
) {
    let num_enemies: usize = enemies.iter().len();

    if num_enemies >= max_enemies.0 {
        return;
    }

    let spawn_rate = if max_enemies.0 - num_enemies <= 1000 {
        max_enemies.0 - num_enemies
    } else {
        1000
    };

    for _ in 0..spawn_rate {
        let angle = random_range(0.0..TAU);
        let distance = random_range(50.0..700.0);

        let spawn =
            player.translation.truncate() + (Vec2::new(angle.cos(), angle.sin()) * distance);

        spawn_enemy(spawn, &mut commands, &assets);
    }
}

fn change_max(
    mut scroll_event: MessageReader<MouseWheel>,
    mut max_enemies: ResMut<MaxEnemies>,
    enemies: Query<Entity, With<Enemy>>,
    mut commands: Commands,
) {
    for event in scroll_event.read() {
        let delta = (event.y * 1000.).round() as i32;
        if delta > 0 {
            max_enemies.0 = max_enemies.0.saturating_add(delta as usize);
        } else if delta < 0 {
            let sub = delta.unsigned_abs() as usize;
            max_enemies.0 = max_enemies.0.saturating_sub(sub);
        }
        for entity in enemies {
            commands.entity(entity).despawn();
        }
    }
}

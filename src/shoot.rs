use crate::player::Player;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct ShootPlugin;

impl Plugin for ShootPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ShootEvent>()
            .add_systems(Startup, load_weapons)
            .add_systems(Update, (bullet_spawner, switch_weapon, shoot));
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

// Der Feuer mode
#[derive(Component)]
enum FireMode {
    Single,
    Auto,
}

// Waffen vorlage
#[derive(Component)]
struct Weapon {
    name: String,
    fire_mode: FireMode,
    enabled: bool,
}

// Inventar
#[derive(Component)]
struct Inventory {
    weapons: Vec<Weapon>,
    selected: usize,
}

fn load_weapons(mut commands: Commands) {
    commands.spawn(Inventory {
        weapons: vec![
            Weapon {
                name: "Glock".to_string(),
                fire_mode: FireMode::Single,
                enabled: true,
            },
            Weapon {
                name: "Ak-47".to_string(),
                fire_mode: FireMode::Single,
                enabled: true,
            },
        ],
        selected: 0,
    });
}

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
                // Movement controlled by rapier
                RigidBody::Dynamic,
                Velocity {
                    linvel: dir * 600.,
                    ..Default::default()
                },
                GravityScale(0.0),
                Collider::ball(2.),
                ActiveEvents::COLLISION_EVENTS,
                Sensor::default(),
            ));
        }
    }
}

fn shoot(mouse: Res<ButtonInput<MouseButton>>, mut shoot_event: EventWriter<ShootEvent>) {
    if mouse.just_pressed(MouseButton::Left) {
        shoot_event.send(ShootEvent);
    }
}

fn switch_weapon(kb: Res<ButtonInput<KeyCode>>, mut query: Query<&mut Inventory>) {
    let Ok(mut inventory) = query.get_single_mut() else {
        return;
    };

    if kb.just_pressed(KeyCode::KeyE) {
        inventory.selected = (inventory.selected + 1) % inventory.weapons.len();
        println!(
            "Weapon switched to: {}",
            inventory.weapons[inventory.selected].name
        );
    }

    if kb.just_pressed(KeyCode::KeyQ) {
        if inventory.selected == 0 {
            inventory.selected = inventory.weapons.len() - 1;
        } else {
            inventory.selected -= 1;
        }
        println!(
            "Weapon switched to: {}",
            inventory.weapons[inventory.selected].name
        );
    }
}

use crate::gamemanager::GameState;
use bevy::{
    picking::hover::Hovered,
    prelude::*,
    ui::Val::Percent,
    ui_widgets::{Activate, Button, observe},
};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), load_menu)
            .add_systems(OnEnter(GameState::InGame), delete_menu);
    }
}

#[derive(Component)]
struct Menu;

fn load_menu(mut commands: Commands) {
    commands.spawn((
        Menu,
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        children![
            (
                button("Resume".to_string()),
                observe(|_activate: On<Activate>, mut commands: Commands| {
                    commands.set_state(GameState::InGame)
                }),
            ),
            (),
            (
                button("Quit".to_string()),
                observe(
                    |_activate: On<Activate>, mut app_exit_writer: MessageWriter<AppExit>| {
                        app_exit_writer.write(AppExit::Success);
                    }
                ),
            ),
        ],
    ));
}

fn delete_menu(mut commands: Commands, query: Query<Entity, With<Menu>>) {
    for menu in query {
        commands.entity(menu).despawn();
    }
}

fn button(text: String) -> impl Bundle {
    (
        // The headless button widget. It handles pointer/keyboard input and pressed-state
        // tracking for us; we only provide the look below.
        Button,
        // `Hovered` is used by the picking backend to track if the pointer is over the button.
        Hovered::default(),
        Node {
            width: px(150),
            height: px(65),
            border: UiRect::all(px(5)),
            // horizontally center child text
            justify_content: JustifyContent::Center,
            // vertically center child text
            align_items: AlignItems::Center,
            border_radius: BorderRadius::MAX,
            margin: UiRect {
                left: Percent(0.),
                right: Percent(0.),
                top: Percent(0.),
                bottom: Percent(1.),
            },
            ..default()
        },
        BorderColor::all(Color::BLACK),
        BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
        children![(Text::new(text), TextColor(Color::srgb(0.9, 0.9, 0.9)),)],
    )
}

use crate::GameState;
use crate::Resources::DataResource::DataResource;
use bevy::prelude::*;

const ROOT_PERCENT: f32 = 100.0;
const PLAY_BUTTON_WIDTH: f32 = 140.0;
const BUTTON_HEIGHT: f32 = 50.0;
const PLAY_TEXT_SIZE: f32 = 40.0;
const PLAY_BUTTON_COLOR: Color = Color::linear_rgb(0.15, 0.15, 0.15);
const PLAY_BUTTON_HOVER_COLOR: Color = Color::linear_rgb(0.25, 0.25, 0.25);
const LABEL_TEXT_COLOR: Color = Color::linear_rgb(0.9, 0.9, 0.9);

/// Creates and tears down the main menu UI and handles returning to the menu
/// from gameplay via the M key.
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), setup_menu)
            .add_systems(Update, click_play_button.run_if(in_state(GameState::Menu)))
            .add_systems(
                Update,
                return_to_menu.run_if(in_state(GameState::Playing)),
            )
            .add_systems(OnExit(GameState::Menu), cleanup_menu);
    }
}

/// Stores the normal and hovered background colors for a button.
#[derive(Component)]
struct ButtonColors {
    normal: Color,
    hovered: Color,
}

impl Default for ButtonColors {
    fn default() -> Self {
        Self {
            normal: PLAY_BUTTON_COLOR,
            hovered: PLAY_BUTTON_HOVER_COLOR,
        }
    }
}

/// Tags entities that should be removed when leaving the menu.
#[derive(Component)]
struct Menu;

/// Spawns the menu UI shown before gameplay starts.
fn setup_menu(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(ROOT_PERCENT),
                height: Val::Percent(ROOT_PERCENT),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            Menu,
        ))
        .with_children(|children| {
            let button_colors = ButtonColors::default();
            children
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(PLAY_BUTTON_WIDTH),
                        height: Val::Px(BUTTON_HEIGHT),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(button_colors.normal),
                    button_colors,
                    ChangeState(GameState::Playing),
                ))
                .with_child((
                    Text::new("Play"),
                    TextFont {
                        font_size: PLAY_TEXT_SIZE,
                        ..default()
                    },
                    TextColor(LABEL_TEXT_COLOR),
                ));
        });
}

/// Changes the app state when the associated button is pressed.
#[derive(Component)]
struct ChangeState(GameState);

/// Handles menu button interaction, hover feedback, and state changes.
fn click_play_button(
    mut next_state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &ButtonColors, Option<&ChangeState>),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, button_colors, change_state) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                if let Some(state) = change_state {
                    next_state.set(state.0.clone());
                }
            }
            Interaction::Hovered => {
                *color = button_colors.hovered.into();
            }
            Interaction::None => {
                *color = button_colors.normal.into();
            }
        }
    }
}

/// Returns to the main menu when the player presses M during gameplay.
/// Clears `will_skip_menu` so the menu is always shown on M � skip is
/// reserved for the initial load and R (which stays in `Playing` state).
fn return_to_menu(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut config: ResMut<DataResource>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyM) {
        config.will_skip_menu = false;
        next_state.set(GameState::Menu);
    }
}

/// Removes all menu UI entities when leaving the menu state.
fn cleanup_menu(mut commands: Commands, menu: Query<Entity, With<Menu>>) {
    for entity in menu.iter() {
        commands.entity(entity).despawn();
    }
}
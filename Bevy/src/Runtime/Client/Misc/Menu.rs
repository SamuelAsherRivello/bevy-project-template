use crate::GameState;
use crate::Resources::AssetsResource::TextureAssets;
use bevy::prelude::*;

const ROOT_PERCENT: f32 = 100.0;
const PLAY_BUTTON_WIDTH: f32 = 140.0;
const LINK_BUTTON_WIDTH: f32 = 170.0;
const BUTTON_HEIGHT: f32 = 50.0;
const BUTTON_ICON_WIDTH: f32 = 32.0;
const PLAY_TEXT_SIZE: f32 = 40.0;
const LINK_TEXT_SIZE: f32 = 15.0;
const MENU_PADDING: f32 = 5.0;
const PLAY_BUTTON_COLOR: Color = Color::linear_rgb(0.15, 0.15, 0.15);
const PLAY_BUTTON_HOVER_COLOR: Color = Color::linear_rgb(0.25, 0.25, 0.25);
const LABEL_TEXT_COLOR: Color = Color::linear_rgb(0.9, 0.9, 0.9);
const BEVY_WEBSITE_URL: &str = "https://bevyengine.org";
const TEMPLATE_REPOSITORY_URL: &str = "https://github.com/NiklasEi/bevy_game_template";

/// Creates and tears down the main menu UI.
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), setup_menu)
            .add_systems(Update, click_play_button.run_if(in_state(GameState::Menu)))
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
fn setup_menu(mut commands: Commands, textures: Res<TextureAssets>) {
    commands.spawn((Camera2d, Msaa::Off));
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
    commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceAround,
                bottom: Val::Px(MENU_PADDING),
                width: Val::Percent(ROOT_PERCENT),
                position_type: PositionType::Absolute,
                ..default()
            },
            Menu,
        ))
        .with_children(|children| {
            children
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(LINK_BUTTON_WIDTH),
                        height: Val::Px(BUTTON_HEIGHT),
                        justify_content: JustifyContent::SpaceAround,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(MENU_PADDING)),
                        ..default()
                    },
                    BackgroundColor(Color::NONE),
                    ButtonColors {
                        normal: Color::NONE,
                        ..default()
                    },
                    OpenLink(BEVY_WEBSITE_URL),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Made with Bevy"),
                        TextFont {
                            font_size: LINK_TEXT_SIZE,
                            ..default()
                        },
                        TextColor(LABEL_TEXT_COLOR),
                    ));
                    parent.spawn((
                        ImageNode {
                            image: textures.bevy.clone(),
                            ..default()
                        },
                        Node {
                            width: Val::Px(BUTTON_ICON_WIDTH),
                            ..default()
                        },
                    ));
                });
            children
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(LINK_BUTTON_WIDTH),
                        height: Val::Px(BUTTON_HEIGHT),
                        justify_content: JustifyContent::SpaceAround,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(MENU_PADDING)),
                        ..default()
                    },
                    BackgroundColor(Color::NONE),
                    ButtonColors {
                        normal: Color::NONE,
                        hovered: PLAY_BUTTON_HOVER_COLOR,
                    },
                    OpenLink(TEMPLATE_REPOSITORY_URL),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Open source"),
                        TextFont {
                            font_size: LINK_TEXT_SIZE,
                            ..default()
                        },
                        TextColor(LABEL_TEXT_COLOR),
                    ));
                    parent.spawn((
                        ImageNode::new(textures.github.clone()),
                        Node {
                            width: Val::Px(BUTTON_ICON_WIDTH),
                            ..default()
                        },
                    ));
                });
        });
}

/// Changes the app state when the associated button is pressed.
#[derive(Component)]
struct ChangeState(GameState);

/// Opens an external URL when the associated button is pressed.
#[derive(Component)]
struct OpenLink(&'static str);

/// Handles menu button interaction, hover feedback, and state changes.
fn click_play_button(
    mut next_state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &ButtonColors,
            Option<&ChangeState>,
            Option<&OpenLink>,
        ),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, button_colors, change_state, open_link) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                if let Some(state) = change_state {
                    next_state.set(state.0.clone());
                } else if let Some(link) = open_link
                    && let Err(error) = webbrowser::open(link.0)
                {
                    warn!("Failed to open link {error:?}");
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

/// Removes all menu UI entities when leaving the menu state.
fn cleanup_menu(mut commands: Commands, menu: Query<Entity, With<Menu>>) {
    for entity in menu.iter() {
        commands.entity(entity).despawn();
    }
}

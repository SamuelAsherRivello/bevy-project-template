use crate::GameState;
use crate::Resources::DataResource::DataResource;
use bevy::prelude::*;

const ROOT_PERCENT: f32 = 100.0;
const PANEL_MIN_WIDTH: f32 = 200.0;
const PANEL_HEIGHT: f32 = 28.0;
const PANEL_OFFSET: f32 = 0.0;
const PANEL_PADDING_X: f32 = 16.0;
const PANEL_PADDING_Y: f32 = 10.0;
const PANEL_BACKGROUND: Color = Color::srgba(0.35, 0.35, 0.35, 0.9);
const PANEL_TEXT_COLOR: Color = Color::linear_rgb(0.95, 0.95, 0.95);
const PANEL_TEXT_SIZE: f32 = 9.0;

/// Displays the in-game HUD while the player is in gameplay.
pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DataResource>()
            .add_systems(OnEnter(GameState::Playing), setup_hud)
            .add_systems(
                Update,
                update_hud
                    .run_if(in_state(GameState::Playing))
                    .run_if(resource_changed::<DataResource>),
            )
            .add_systems(OnExit(GameState::Playing), cleanup_hud);
    }
}

#[derive(Component)]
struct HudRoot;

/// Marks the Text node that displays the lives counter.
#[derive(Component)]
struct LivesText;

/// Marks the Text node that displays the score counter.
#[derive(Component)]
struct ScoreText;

#[derive(Clone, Copy)]
enum HudCorner {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

fn format_lives(data: &DataResource) -> String {
    format!("Lives: {:03}/{:03}", data.lives_current, data.lives_max)
}

fn format_score(data: &DataResource) -> String {
    format!("Score: {:03}/{:03}", data.score_current, data.score_max)
}

fn setup_hud(mut commands: Commands, data: Res<DataResource>) {
    commands
        .spawn((
            Name::new("HUD"),
            Node {
                width: Val::Percent(ROOT_PERCENT),
                height: Val::Percent(ROOT_PERCENT),
                position_type: PositionType::Absolute,
                ..default()
            },
            HudRoot,
        ))
        .with_children(|children| {
            spawn_panel(children, &format_lives(&data), HudCorner::TopLeft, LivesText);
            spawn_panel(children, &format_score(&data), HudCorner::TopRight, ScoreText);
            spawn_panel(
                children,
                "Instructions: Wasd/Arrows, Space, R, M, T",
                HudCorner::BottomLeft,
                (),
            );
            spawn_panel(children, "Game", HudCorner::BottomRight, ());
        });
}

/// Rerenders the dynamic HUD panels whenever `DataResource` changes.
fn update_hud(
    data: Res<DataResource>,
    mut lives_text: Single<&mut Text, (With<LivesText>, Without<ScoreText>)>,
    mut score_text: Single<&mut Text, (With<ScoreText>, Without<LivesText>)>,
) {
    **lives_text = Text::new(format_lives(&data));
    **score_text = Text::new(format_score(&data));
}

fn spawn_panel<B: Bundle>(
    parent: &mut ChildSpawnerCommands,
    label: &str,
    corner: HudCorner,
    extra: B,
) {
    parent
        .spawn((
            Node {
                min_width: Val::Px(PANEL_MIN_WIDTH),
                height: Val::Px(PANEL_HEIGHT),
                position_type: PositionType::Absolute,
                padding: UiRect::axes(Val::Px(PANEL_PADDING_X), Val::Px(PANEL_PADDING_Y)),
                align_items: AlignItems::Center,
                justify_content: panel_justify(corner),
                ..panel_position(corner)
            },
            BackgroundColor(PANEL_BACKGROUND),
        ))
        .with_child((
            Text::new(label),
            TextFont {
                font_size: PANEL_TEXT_SIZE,
                ..default()
            },
            text_layout(corner),
            TextColor(PANEL_TEXT_COLOR),
            extra,
        ));
}

fn text_layout(corner: HudCorner) -> TextLayout {
    match corner {
        HudCorner::TopLeft | HudCorner::BottomLeft => TextLayout::new_with_justify(Justify::Left),
        HudCorner::TopRight | HudCorner::BottomRight => {
            TextLayout::new_with_justify(Justify::Right)
        }
    }
}

fn panel_justify(corner: HudCorner) -> JustifyContent {
    match corner {
        HudCorner::TopLeft | HudCorner::BottomLeft => JustifyContent::FlexStart,
        HudCorner::TopRight | HudCorner::BottomRight => JustifyContent::FlexEnd,
    }
}

fn panel_position(corner: HudCorner) -> Node {
    match corner {
        HudCorner::TopLeft => Node {
            left: Val::Px(PANEL_OFFSET),
            top: Val::Px(PANEL_OFFSET),
            ..default()
        },
        HudCorner::TopRight => Node {
            right: Val::Px(PANEL_OFFSET),
            top: Val::Px(PANEL_OFFSET),
            ..default()
        },
        HudCorner::BottomLeft => Node {
            left: Val::Px(PANEL_OFFSET),
            bottom: Val::Px(PANEL_OFFSET),
            ..default()
        },
        HudCorner::BottomRight => Node {
            right: Val::Px(PANEL_OFFSET),
            bottom: Val::Px(PANEL_OFFSET),
            ..default()
        },
    }
}

fn cleanup_hud(mut commands: Commands, hud_roots: Query<Entity, With<HudRoot>>) {
    for entity in &hud_roots {
        commands.entity(entity).despawn();
    }
}
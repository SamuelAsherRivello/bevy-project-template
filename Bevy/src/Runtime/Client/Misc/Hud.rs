use crate::GameState;
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
        app.add_systems(OnEnter(GameState::Playing), setup_hud)
            .add_systems(OnExit(GameState::Playing), cleanup_hud);
    }
}

#[derive(Component)]
struct HudRoot;

#[derive(Clone, Copy)]
enum HudCorner {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

fn setup_hud(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(ROOT_PERCENT),
                height: Val::Percent(ROOT_PERCENT),
                position_type: PositionType::Absolute,
                ..default()
            },
            HudRoot,
        ))
        .with_children(|children| {
            spawn_panel(children, "Lives: 003/003", HudCorner::TopLeft);
            spawn_panel(children, "Score: 000/005", HudCorner::TopRight);
            spawn_panel(
                children,
                "Instructions: Wasd/Arrows, Spacebar, R",
                HudCorner::BottomLeft,
            );
            spawn_panel(children, "Game", HudCorner::BottomRight);
        });
}

fn spawn_panel(parent: &mut ChildSpawnerCommands, label: &str, corner: HudCorner) {
    parent
        .spawn((
            Node {
                min_width: Val::Px(PANEL_MIN_WIDTH),
                height: Val::Px(PANEL_HEIGHT),
                position_type: PositionType::Absolute,
                padding: UiRect::axes(Val::Px(PANEL_PADDING_X), Val::Px(PANEL_PADDING_Y)),
                align_items: AlignItems::Center,
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
        ));
}

fn text_layout(corner: HudCorner) -> TextLayout {
    match corner {
        HudCorner::TopLeft | HudCorner::BottomLeft => TextLayout::new_with_justify(Justify::Left),
        HudCorner::TopRight | HudCorner::BottomRight => TextLayout::new_with_justify(Justify::Right),
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
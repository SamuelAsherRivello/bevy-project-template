use bevy::prelude::*;

use crate::{
    game_component::GameComponent, game_frame_resource::GameFrameResource, ui::UiTextResource,
    ui_text_component::UITextComponent,
};

pub fn ui_startup_system(mut commands: Commands) {
    commands.spawn((
        Text::new("Waiting for game UI..."),
        TextFont {
            font_size: 22.0,
            ..Default::default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(24.0),
            top: Val::Px(24.0),
            padding: UiRect::axes(Val::Px(12.0), Val::Px(8.0)),
            ..Default::default()
        },
        BackgroundColor(Color::srgba(0.02, 0.02, 0.02, 0.72)),
        BorderRadius::all(Val::Px(8.0)),
        UITextComponent,
        GameComponent,
    ));
}

pub fn ui_update_system(
    frame: Res<GameFrameResource>,
    mut ui_text: ResMut<UiTextResource>,
    mut text_query: Query<&mut Text, With<UITextComponent>>,
) {
    ui_text.text = format!(
        "Bevy Project Template\nReloads : {}\nGlobal Frame: {}\nLocal Frame: {}",
        frame.reload_count, frame.frame_global_count, frame.frame_local_count
    );

    for mut text in &mut text_query {
        *text = Text::new(ui_text.text.clone());
    }
}

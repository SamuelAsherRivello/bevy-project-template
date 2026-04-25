use bevy::prelude::*;
use bevy_simple_subsecond_system as hot_reload;
use hot_reload::prelude::hot;
use shared::context_resource::ContextResource;

use crate::{
    hud_resource::HUDTextResource, hud_text_component::HUDTextComponent,
};

pub fn hud_startup_system(mut commands: Commands) {
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
            border_radius: BorderRadius::all(Val::Px(8.0)),
            ..Default::default()
        },
        BackgroundColor(Color::srgba(0.02, 0.02, 0.02, 0.72)),
        HUDTextComponent,
    ));
}

#[hot]
pub fn hud_update_system(
    context: Res<ContextResource>,
    mut hud_text: ResMut<HUDTextResource>,
    mut text_query: Query<&mut Text, With<HUDTextComponent>>,
) {
    hud_text.text = format!(
        "Bevy Project Template\nFrame: {}\nReloads: {}\nKeys: WASD, T",
        context.frame_local_count,
        context.reload_count
    );

    for mut text in &mut text_query {
        *text = Text::new(hud_text.text.clone());
    }
}

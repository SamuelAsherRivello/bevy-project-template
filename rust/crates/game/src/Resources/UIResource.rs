use bevy_ecs::prelude::Resource;
use game_api::UI_TEXT_CAPACITY;

#[derive(Default, Resource)]
pub struct UiTextResource {
    pub text: String,
}

pub fn build_ui_text_bytes(text: &str) -> (u32, [u8; UI_TEXT_CAPACITY]) {
    let mut bytes = [0; UI_TEXT_CAPACITY];
    let source = text.as_bytes();
    let len = source.len().min(UI_TEXT_CAPACITY);
    bytes[..len].copy_from_slice(&source[..len]);
    (len as u32, bytes)
}

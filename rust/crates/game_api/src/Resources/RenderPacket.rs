use crate::{
    api_constants::{MAX_RENDER_ITEMS, UI_TEXT_CAPACITY},
    color_rgba::ColorRgba,
    render_item::RenderItem,
};

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct RenderPacket {
    pub clear_color: ColorRgba,
    pub reload_count: u32,
    pub ui_text_len: u32,
    pub ui_text: [u8; UI_TEXT_CAPACITY],
    pub render_item_count: u32,
    pub render_items: [RenderItem; MAX_RENDER_ITEMS],
}

impl Default for RenderPacket {
    fn default() -> Self {
        Self {
            clear_color: ColorRgba {
                red: 0.05,
                green: 0.07,
                blue: 0.10,
                alpha: 1.0,
            },
            reload_count: 0,
            ui_text_len: 0,
            ui_text: [0; UI_TEXT_CAPACITY],
            render_item_count: 0,
            render_items: [RenderItem::default(); MAX_RENDER_ITEMS],
        }
    }
}


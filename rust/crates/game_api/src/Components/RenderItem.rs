use crate::{color_rgba::ColorRgba, vec3_data::Vec3Data};

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct RenderItem {
    pub kind: u32,
    pub color: ColorRgba,
    pub translation: Vec3Data,
    pub rotation_radians: Vec3Data,
    pub scale: Vec3Data,
}

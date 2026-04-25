#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct ColorRgba {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32,
}

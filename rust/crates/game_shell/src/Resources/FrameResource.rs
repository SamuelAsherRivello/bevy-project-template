pub struct FrameResource {
    pub delta_seconds: f32,
    pub shell_frame: u64,
    pub turn_input: f32,
}

impl Default for FrameResource {
    fn default() -> Self {
        Self {
            delta_seconds: 0.0,
            shell_frame: 0,
            turn_input: 0.0,
        }
    }
}


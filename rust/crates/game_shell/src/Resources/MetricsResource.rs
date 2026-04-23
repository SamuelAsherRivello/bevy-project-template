pub struct MetricsResource {
    pub elapsed_seconds: f32,
    pub last_shell_frame: u64,
}

impl Default for MetricsResource {
    fn default() -> Self {
        Self {
            elapsed_seconds: 0.0,
            last_shell_frame: 0,
        }
    }
}


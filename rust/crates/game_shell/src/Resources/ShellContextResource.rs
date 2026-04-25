use game_api::Context;

pub struct ShellContextResource {
    pub reload_count: u64,
    pub frame_global_count: u64,
    pub frame_local_count: u64,
}

impl Context for ShellContextResource {
    fn reload_count(&self) -> u64 {
        self.reload_count
    }

    fn frame_global_count(&self) -> u64 {
        self.frame_global_count
    }

    fn frame_local_count(&self) -> u64 {
        self.frame_local_count
    }

    fn log(&mut self, message: &str) {
        println!("[frame {:04}] {message}", self.frame_global_count);
    }
}

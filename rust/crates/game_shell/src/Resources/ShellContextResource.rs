use game_api::Context;

pub struct ShellContextResource {
    pub frame: u64,
    pub left_pressed: bool,
    pub right_pressed: bool,
    pub delta_seconds: f32,
    pub turn_input: f32,
    pub elapsed_seconds: f32,
}

impl Context for ShellContextResource {
    fn frame(&self) -> u64 {
        self.frame
    }

    fn log(&mut self, message: &str) {
        println!("[frame {:04}] {message}", self.frame);
    }

    fn left_pressed(&self) -> bool {
        self.left_pressed
    }

    fn right_pressed(&self) -> bool {
        self.right_pressed
    }

    fn delta_seconds(&self) -> f32 {
        self.delta_seconds
    }

    fn turn_input(&self) -> f32 {
        self.turn_input
    }

    fn elapsed_seconds(&self) -> f32 {
        self.elapsed_seconds
    }
}

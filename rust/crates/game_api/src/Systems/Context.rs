pub trait Context {
    fn frame(&self) -> u64;
    fn log(&mut self, message: &str);
    fn left_pressed(&self) -> bool;
    fn right_pressed(&self) -> bool;
    fn delta_seconds(&self) -> f32;
    fn turn_input(&self) -> f32;
    fn elapsed_seconds(&self) -> f32;
}

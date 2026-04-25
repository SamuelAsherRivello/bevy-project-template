pub trait Context {
    fn reload_count(&self) -> u64;
    fn frame_global_count(&self) -> u64;
    fn frame_local_count(&self) -> u64;
    fn log(&mut self, message: &str);
}

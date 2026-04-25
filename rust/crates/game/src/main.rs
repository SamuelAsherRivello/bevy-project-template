fn main() {
    if std::env::var_os("WGPU_BACKEND").is_none() {
        unsafe {
            std::env::set_var("WGPU_BACKEND", "dx12");
        }
    }

    game::create_app().run();
}

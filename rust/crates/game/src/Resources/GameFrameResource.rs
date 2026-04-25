use bevy_ecs::prelude::Resource;

#[derive(Resource, Default)]
pub struct GameFrameResource {
    pub reload_count: u64,
    pub frame_global_count: u64,
    pub frame_local_count: u64,
}

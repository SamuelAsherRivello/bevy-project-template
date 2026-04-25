use bevy::prelude::*;

use crate::context_resource::ContextResource;

pub fn context_update_system(mut context: ResMut<ContextResource>) {
    context.frame_global_count += 1;
    context.frame_local_count += 1;
}

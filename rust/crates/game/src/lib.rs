use std::cell::RefCell;

use bevy_app::App;
use game_api::{
    ColorRgba, Context, MAX_RENDER_ITEMS, RENDER_ITEM_KIND_CUBE, RenderItem, RenderPacket, Vec3Data,
};

#[path = "Plugins/GamePlugin.rs"]
mod game_plugin;
#[path = "Plugins/PlayerPlugin.rs"]
mod player_plugin;
#[path = "Plugins/InputPlugin.rs"]
mod input_plugin;
#[path = "Components/PlayerComponent.rs"]
mod player_component;
#[path = "Components/InputComponent.rs"]
mod input_component;
#[path = "Resources/UIResource.rs"]
mod ui;
#[path = "Systems/PlayerSystem.rs"]
mod player_system;
#[path = "Systems/InputSystem.rs"]
mod input_system;

use game_plugin::GamePlugin;
use input_component::InputComponent;
use player_component::PlayerComponent;
use ui::{UiTextResource, build_ui_text_bytes};

thread_local! {
    static GAME_STATE: RefCell<Option<GameState>> = const { RefCell::new(None) };
}

struct GameState {
    app: App,
    reload_count: u64,
}

impl GameState {
    fn new(context: &mut dyn Context, reload_count: u64) -> Self {
        let mut app = App::new();
        app.add_plugins(GamePlugin { reload_count });

        let mut game_state = Self {
            app,
            reload_count,
        };

        game_state.app.update();
        game_state.update_input_component(context);
        game_state.app.update();
        game_state
    }

    fn frame(&mut self, context: &mut dyn Context) {
        self.update_input_component(context);
        self.app.update();
    }

    fn update_input_component(&mut self, context: &dyn Context) {
        let world = self.app.world_mut();
        let mut input_query = world.query::<&mut InputComponent>();
        let Some(mut input) = input_query.iter_mut(world).next() else {
            return;
        };

        input.delta_seconds = context.delta_seconds();
        input.elapsed_seconds = context.elapsed_seconds();
        input.turn_input = context.turn_input();
        input.left_pressed = context.left_pressed();
        input.right_pressed = context.right_pressed();
    }

}

fn build_render_packet_from_world(app: &mut App, reload_count: u64) -> RenderPacket {
    let world = app.world_mut();

    let mut player_query = world.query::<&PlayerComponent>();
    let player = player_query.iter(world).next();
    let cube_angle_degrees = player.map_or(0.0, |p| p.angle_degrees);
    let cube_scale = player.map_or(1.0, |p| p.scale);

    let mut render_items = [RenderItem::default(); MAX_RENDER_ITEMS];
    let base_rotation = cube_angle_degrees.to_radians();

    render_items[0] = RenderItem {
        kind: RENDER_ITEM_KIND_CUBE,
        color: ColorRgba {
            red: 0.22,
            green: 0.72,
            blue: 1.0,
            alpha: 1.0,
        },
        translation: Vec3Data {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        rotation_radians: Vec3Data {
            x: 0.0,
            y: base_rotation,
            z: 0.0,
        },
        scale: Vec3Data {
            x: 1.5 * cube_scale,
            y: 1.5 * cube_scale,
            z: 1.5 * cube_scale,
        },
    };

    render_items[1] = RenderItem {
        kind: RENDER_ITEM_KIND_CUBE,
        color: ColorRgba {
            red: 0.18,
            green: 0.22,
            blue: 0.28,
            alpha: 1.0,
        },
        translation: Vec3Data {
            x: 0.0,
            y: -1.0,
            z: 0.0,
        },
        rotation_radians: Vec3Data::default(),
        scale: Vec3Data {
            x: 4.5,
            y: 0.25,
            z: 4.5,
        },
    };

    let (ui_text_len, ui_text) = {
        let ui_text = world.resource::<UiTextResource>();
        build_ui_text_bytes(&ui_text.text)
    };

    RenderPacket {
        clear_color: ColorRgba {
            red: 0.05,
            green: 0.07,
            blue: 0.10,
            alpha: 1.0,
        },
        reload_count: reload_count as u32,
        ui_text_len,
        ui_text,
        render_item_count: 2,
        render_items,
    }
}

#[unsafe(no_mangle)]
#[allow(improper_ctypes_definitions, non_snake_case)]
pub extern "C" fn AppInitialize(context: &mut dyn Context) {
    app_initialize(context);
}

pub fn app_initialize(context: &mut dyn Context) {
    context.log("Game crate initialized with Bevy app demo");
}

#[unsafe(no_mangle)]
#[allow(improper_ctypes_definitions, non_snake_case)]
pub extern "C" fn AppHotReload(context: &mut dyn Context, reload_count: u64) {
    app_hot_reload(context, reload_count);
}

pub fn app_hot_reload(context: &mut dyn Context, reload_count: u64) {
    GAME_STATE.with(|game_state| {
        *game_state.borrow_mut() = Some(GameState::new(context, reload_count));
    });
}

#[unsafe(no_mangle)]
#[allow(improper_ctypes_definitions)]
pub extern "C" fn hot_frame(context: &mut dyn Context) {
    hot_frame_rust(context);
}

pub fn hot_frame_rust(context: &mut dyn Context) {
    GAME_STATE.with(|game_state| {
        let mut game_state = game_state.borrow_mut();

        if game_state.is_none() {
            *game_state = Some(GameState::new(context, 1));
        }

        if let Some(game_state) = game_state.as_mut() {
            game_state.frame(context);
        }
    });
}

#[unsafe(no_mangle)]
pub extern "C" fn hot_render_packet() -> RenderPacket {
    hot_render_packet_rust()
}

pub fn hot_render_packet_rust() -> RenderPacket {
    GAME_STATE.with(|game_state| {
        let mut game_state = game_state.borrow_mut();
        let Some(game_state) = game_state.as_mut() else {
            return RenderPacket::default();
        };

        build_render_packet_from_world(&mut game_state.app, game_state.reload_count)
    })
}

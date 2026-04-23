use std::{
    env,
    error::Error,
    fs,
    path::{Path, PathBuf},
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowClosing, WindowMoved, WindowPosition};
use game_api::{Context, RenderPacket};
use std::time::SystemTime as StdSystemTime;

use crate::{host_state_resource::HostStateResource, shell_context_resource::ShellContextResource};

#[allow(improper_ctypes_definitions)]
type AppInitialize = extern "C" fn(&mut dyn Context);
#[allow(improper_ctypes_definitions)]
type AppHotReload = extern "C" fn(&mut dyn Context, u64);
#[allow(improper_ctypes_definitions)]
type GameFrame = extern "C" fn(&mut dyn Context);
type GameRenderPacket = extern "C" fn() -> RenderPacket;

pub struct GameRuntime {
    initialize: AppInitialize,
    hot_reload: AppHotReload,
    frame: GameFrame,
    render_packet: GameRenderPacket,
    dll_modified: StdSystemTime,
    loaded_from: PathBuf,
    library: Option<libloading::Library>,
}

impl GameRuntime {
    pub fn load(
        context: &mut ShellContextResource,
        run_initialize: bool,
        reload_count: u64,
    ) -> Result<Self, Box<dyn Error + Send + Sync>> {
        let source = game_dll_path();
        let dll_modified = fs::metadata(&source)?.modified()?;
        let loaded_from = copy_game_dll(&source)?;

        let library = unsafe { libloading::Library::new(&loaded_from)? };
        let initialize = unsafe {
            let symbol: libloading::Symbol<AppInitialize> = library.get(b"AppInitialize")?;
            *symbol
        };
        let hot_reload = unsafe {
            let symbol: libloading::Symbol<AppHotReload> = library.get(b"AppHotReload")?;
            *symbol
        };
        let frame = unsafe {
            let symbol: libloading::Symbol<GameFrame> = library.get(b"hot_frame")?;
            *symbol
        };
        let render_packet = unsafe {
            let symbol: libloading::Symbol<GameRenderPacket> = library.get(b"hot_render_packet")?;
            *symbol
        };

        println!("loaded {}", loaded_from.display());

        let game = Self {
            initialize,
            hot_reload,
            frame,
            render_packet,
            dll_modified,
            loaded_from,
            library: Some(library),
        };

        if run_initialize {
            game.initialize(context);
        }
        game.hot_reload(context, reload_count);

        Ok(game)
    }

    fn initialize(&self, context: &mut dyn Context) {
        (self.initialize)(context);
    }

    fn hot_reload(&self, context: &mut dyn Context, reload_count: u64) {
        (self.hot_reload)(context, reload_count);
    }

    pub fn frame(&self, context: &mut dyn Context) {
        (self.frame)(context);
    }

    pub fn render_packet(&self) -> RenderPacket {
        (self.render_packet)()
    }

    pub fn artifact_changed(&self) -> bool {
        fs::metadata(game_dll_path())
            .and_then(|metadata| metadata.modified())
            .is_ok_and(|modified| modified > self.dll_modified)
    }

    pub fn unload(&mut self, context: &mut ShellContextResource) {
        self.library.take();
        context.log(&format!("unloaded {}", self.loaded_from.display()));
    }
}

pub struct GameSourceWatcher {
    last_modified: StdSystemTime,
}

impl GameSourceWatcher {
    pub fn new() -> Result<Self, Box<dyn Error + Send + Sync>> {
        Ok(Self {
            last_modified: game_source_modified()?,
        })
    }

    pub fn rebuild_if_changed(&mut self, context: &mut ShellContextResource) -> bool {
        let Ok(modified) = game_source_modified() else {
            return false;
        };

        if modified <= self.last_modified {
            return false;
        }

        self.last_modified = modified;
        context.log("game source changed; rebuilding game DLL");
        build_game(&mut Some(context));
        true
    }
}

pub fn supports_hot_reload() -> bool {
    true
}

pub fn configure_runtime() {
    if env::var_os("WGPU_BACKEND").is_none() {
        unsafe {
            env::set_var("WGPU_BACKEND", "dx12");
        }
    }
}

pub fn debug_log(_message: &str) {}

pub fn debug_render_packet(_frame: u64, _packet: &RenderPacket) {}

pub fn ensure_game_ready() {
    if !game_dll_path().exists() {
        build_game(&mut None);
    }
}

pub fn primary_window(initial_window_position: Option<IVec2>) -> Window {
    Window {
        title: "Rust Hot Reload Demo - Bevy".to_owned(),
        resolution: (800.0, 600.0).into(),
        position: initial_window_position
            .map(WindowPosition::At)
            .unwrap_or(WindowPosition::Automatic),
        ..Default::default()
    }
}

pub fn track_window_position_update_system(
    mut host: NonSendMut<HostStateResource>,
    mut moved_events: EventReader<WindowMoved>,
    primary_window: Query<Entity, With<PrimaryWindow>>,
) {
    let Ok(primary_window) = primary_window.get_single() else {
        return;
    };

    for event in moved_events.read() {
        if event.window == primary_window {
            host.last_window_position = Some(event.position);
        }
    }
}

pub fn persist_window_position_on_close_update_system(
    host: NonSend<HostStateResource>,
    mut closing_events: EventReader<WindowClosing>,
    primary_window: Query<(Entity, &Window), With<PrimaryWindow>>,
) {
    let Ok((primary_window, window)) = primary_window.get_single() else {
        return;
    };

    for event in closing_events.read() {
        if event.window != primary_window {
            continue;
        }

        let position = match window.position {
            WindowPosition::At(position) => Some(position),
            _ => host.last_window_position,
        };

        if let Some(position) = position {
            save_window_position(position);
        }
    }
}

pub fn load_saved_window_position() -> Option<IVec2> {
    let contents = fs::read_to_string(window_state_path()).ok()?;
    let (x, y) = contents.trim().split_once(',')?;
    let x = x.parse().ok()?;
    let y = y.parse().ok()?;
    Some(IVec2::new(x, y))
}

fn build_game(context: &mut Option<&mut ShellContextResource>) {
    let output = Command::new("cargo")
        .args(["build", "-p", "game"])
        .current_dir(workspace_root())
        .output();

    match output {
        Ok(output) if output.status.success() => append_log(context, "game build finished"),
        Ok(output) => {
            append_log(context, format!("game build failed with {}", output.status));
            append_command_output(context, &output.stderr);
        }
        Err(error) => append_log(context, format!("failed to run cargo build -p game: {error}")),
    }
}

fn append_command_output(context: &mut Option<&mut ShellContextResource>, output: &[u8]) {
    let output = String::from_utf8_lossy(output);
    for line in output.lines() {
        append_log(context, line);
    }
}

fn append_log(context: &mut Option<&mut ShellContextResource>, message: impl AsRef<str>) {
    if let Some(context) = context {
        context.log(message.as_ref());
    } else {
        println!("{}", message.as_ref());
    }
}

fn game_source_modified() -> Result<StdSystemTime, Box<dyn Error + Send + Sync>> {
    newest_rust_file_modified(
        &workspace_root()
            .join("rust")
            .join("crates")
            .join("game")
            .join("src"),
    )
}

fn newest_rust_file_modified(path: &Path) -> Result<StdSystemTime, Box<dyn Error + Send + Sync>> {
    let mut newest = StdSystemTime::UNIX_EPOCH;

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            newest = newest.max(newest_rust_file_modified(&path)?);
        } else if path.extension().is_some_and(|extension| extension == "rs") {
            newest = newest.max(fs::metadata(path)?.modified()?);
        }
    }

    Ok(newest)
}

fn game_dll_path() -> PathBuf {
    workspace_root()
        .join("target")
        .join("debug")
        .join(dll_file_name())
}

fn dll_file_name() -> &'static str {
    if cfg!(target_os = "windows") {
        "game.dll"
    } else if cfg!(target_os = "macos") {
        "libgame.dylib"
    } else {
        "libgame.so"
    }
}

fn copy_game_dll(source: &Path) -> Result<PathBuf, Box<dyn Error + Send + Sync>> {
    let hot_dir = workspace_root().join("target").join("hot-reload");
    fs::create_dir_all(&hot_dir)?;

    let stamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    let destination = hot_dir.join(format!("game_hot_{stamp}.dll"));
    fs::copy(source, &destination)?;

    Ok(destination)
}

fn save_window_position(position: IVec2) {
    let path = window_state_path();

    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }

    let _ = fs::write(path, format!("{},{}", position.x, position.y));
}

fn window_state_path() -> PathBuf {
    workspace_root()
        .join("target")
        .join("window-state")
        .join("primary-window-position.txt")
}

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|path| path.parent())
        .and_then(|path| path.parent())
        .expect("game_shell crate should be inside rust/crates/")
        .to_path_buf()
}


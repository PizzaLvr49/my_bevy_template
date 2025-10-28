use bevy::prelude::*;
use bevy::window::{PresentMode, WindowMode};
use panic_handler::PanicHandler;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: PresentMode::AutoVsync,
                mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                name: Some("Pong".to_string()),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(PanicHandler::default())
        .run();
}

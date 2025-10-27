mod panic_handler;

use bevy::prelude::*;
use panic_handler::PanicHandler;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: bevy::window::PresentMode::AutoVsync,
                mode: bevy::window::WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                name: Some("Pong".to_string()),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(PanicHandler::default());
}

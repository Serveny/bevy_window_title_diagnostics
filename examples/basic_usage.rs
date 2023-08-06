use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy_window_title_diagnostics::WindowTitleLoggerDiagnosticsPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_plugins((
            FrameTimeDiagnosticsPlugin,
            // Insert same way as usual LogDiagnosticsPlugin
            WindowTitleLoggerDiagnosticsPlugin {
                // It is possible to filter Diagnostics same way as default LogDiagnosticsPlugin
                // filter: Some(vec![FrameTimeDiagnosticsPlugin::FPS]),
                ..Default::default()
            },
            // Works with any diagnostics
            //bevy::diagnostic::EntityCountDiagnosticsPlugin::default(),
        ))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::new_with_far(10.0));
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::CYAN,
            custom_size: Some(Vec2::new(50.0, 50.0)),
            ..default()
        },
        ..default()
    });
}

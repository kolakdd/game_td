use bevy::window::WindowResolution;
use bevy::prelude::*;
use td_game::GamePlugin;
use bevy::dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(800., 600.),
                title: "TD GAME".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(GamePlugin)
        .add_plugins((
            FpsOverlayPlugin {
                config: FpsOverlayConfig {
                    text_config: TextFont {
                        // Here we define size of our overlay
                        font_size: 42.0,
                        // If we want, we can use a custom font
                        font: default(),
                        font_smoothing: bevy::text::FontSmoothing::None,
                    },
                    // We can also change color of the overlay
                    text_color: Color::srgb(0., 1., 0.),
                    enabled: true,
                },
            },
        ))
        .run();
}

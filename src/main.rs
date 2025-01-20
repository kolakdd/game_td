use bevy::window::WindowResolution;
use bevy::{
    image::{ImageLoaderSettings, ImageSampler},
    prelude::*,
    ui::widget::NodeImageMode,
    winit::WinitSettings,
};
use td_game::AppState;
use td_game::GamePlugin;

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
        .run();
}

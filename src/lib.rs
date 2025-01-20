use crate::map::MapPlugin;
use crate::menu::MenuPlugin;
use bevy::prelude::*;
use bevy_prototype_lyon::plugin::ShapePlugin;
use loading::AssetsWarmUpPlugin;

mod loading;
mod map;
mod menu;
mod ui;

const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);

pub const MAP_Z: f32 = 0.;
pub const TOWER_Z: f32 = 1.;
pub const ENEMY_Z: f32 = 3.;
pub const BULLET_Z: f32 = 4.;

#[derive(Debug, States, Hash, Eq, PartialEq, Clone, Default)]
pub enum AppState {
    AssetsWarmup,
    Restart,
    #[default]
    LoadingMenu,
    LoadingLevel,
    Saving,
    InGame,
    Menu,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        println!("DO GAME PLUGIN");
        app.insert_resource(ClearColor(Color::BLACK))
            .insert_state(AppState::AssetsWarmup)
            .add_plugins((AssetsWarmUpPlugin, ShapePlugin, MenuPlugin, MapPlugin));
    }
}

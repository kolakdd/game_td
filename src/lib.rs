use bevy::prelude::*;

use crate::map::MapPlugin;

mod loading;
mod map;



pub const MAP_Z: f32 = 0.;
pub const TOWER_Z: f32 = 1.;
pub const ENEMY_Z: f32 = 3.;
pub const BULLET_Z: f32 = 4.;

#[derive(Debug, States, Hash, Eq, PartialEq, Clone, Default)]
pub enum AppState {
    Restart,
    #[default]
    Loading,
    Saving,
    InGame,
    Menu,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        println!("DO GAME PLUGIN");
        app.insert_resource(ClearColor(Color::BLACK))
            .insert_state(AppState::Loading)
            .add_plugins((MapPlugin,));
    }
}

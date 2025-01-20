use crate::map::Tile;
use crate::AppState;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(AppState::Loading)
                .continue_to_state(AppState::Menu)
                .load_collection::<TextureAssets>()
        );
    }
}

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(path = "textures/blank.png")]
    pub blank: Handle<Image>,
    #[asset(path = "textures/towerplot.png")]
    pub tower_plot: Handle<Image>,
    #[asset(path = "textures/tower.png")]
    pub tower: Handle<Image>,
    #[asset(path = "textures/path.png")]
    pub path: Handle<Image>,
    #[asset(path = "textures/castle.png")]
    pub castle: Handle<Image>,
    #[asset(path = "textures/spawn.png")]
    pub spawn: Handle<Image>,
}

impl TextureAssets {
    pub fn get_handle_for_tile(&self, tile: &Tile) -> Handle<Image> {
        match *tile {
            Tile::Empty => self.blank.clone(),
            Tile::TowerPlot => self.tower_plot.clone(),
            Tile::Tower => self.tower.clone(),
            Tile::Path => self.path.clone(),
            Tile::Castle => self.castle.clone(),
            Tile::Spawn => self.spawn.clone(),
        }
    }
}
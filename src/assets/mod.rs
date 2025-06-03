use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::prelude::*;

pub mod game_assets;
pub mod tower_sprites;
pub mod ui_assets;

pub use {game_assets::GameAssets, tower_sprites::TowerSprites, ui_assets::UiAssets};

pub(super) fn plugin(app: &mut App) {
    app.add_loading_state(
        LoadingState::new(Screen::Loading)
            .continue_to_state(Screen::Splash)
            .load_collection::<GameAssets>()
            .load_collection::<TowerSprites>()
            .load_collection::<UiAssets>(),
    );

    app.add_systems(OnEnter(Screen::Splash), GameAssets::meshes_and_materials);
}

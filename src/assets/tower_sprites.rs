use crate::data::Tower;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct TowerSprites {
    #[asset(path = "images/towers/tower_tesla.png")]
    pub tesla_sprite: Handle<Image>,
    #[asset(texture_atlas_layout(tile_size_x = 256, tile_size_y = 256, columns = 2, rows = 1))]
    pub tesla_layout: Handle<TextureAtlasLayout>,
}

impl TowerSprites {
    pub fn tower_sprite(&self, tower: &Tower) -> (Handle<Image>, TextureAtlas) {
        match tower {
            Tower::Tesla => (
                self.tesla_sprite.clone(),
                TextureAtlas::from(self.tesla_layout.clone()),
            ),
            _ => (
                self.tesla_sprite.clone(),
                TextureAtlas::from(self.tesla_layout.clone()),
            ),
        }
    }

    pub fn tower_bundle(&self, tower: &Tower) -> impl Bundle {
        let (image, atlas) = self.tower_sprite(tower);

        (Sprite {
            image,
            custom_size: Some(Vec2::splat(10.0)),
            texture_atlas: Some(TextureAtlas::from(atlas)),
            ..default()
        })
    }
}

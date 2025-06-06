use crate::data::StatusEffect;
use crate::data::projectiles::LiquidType;
use crate::gameplay::animation::AnimationFrameQueue;
use crate::level::components::LEVEL_SCALING;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct LiquidSprites {
    #[asset(path = "images/liquids/water.png")]
    oil_sprite: Handle<Image>,
    #[asset(texture_atlas_layout(tile_size_x = 32, tile_size_y = 32, columns = 10, rows = 1))]
    oil_layout: Handle<TextureAtlasLayout>,

    #[asset(path = "images/liquids/water.png")]
    water_sprite: Handle<Image>,
    #[asset(texture_atlas_layout(tile_size_x = 32, tile_size_y = 32, columns = 10, rows = 1))]
    water_layout: Handle<TextureAtlasLayout>,

    #[asset(path = "images/liquids/water.png")]
    acid_sprite: Handle<Image>,
    #[asset(texture_atlas_layout(tile_size_x = 32, tile_size_y = 32, columns = 10, rows = 1))]
    acid_layout: Handle<TextureAtlasLayout>,

    #[asset(path = "images/liquids/water.png")]
    oil_puddle_sprite: Handle<Image>,
    #[asset(texture_atlas_layout(tile_size_x = 64, tile_size_y = 64, columns = 10, rows = 1))]
    oil_puddle_layout: Handle<TextureAtlasLayout>,

    #[asset(path = "images/liquids/water-puddle.png")]
    water_puddle_sprite: Handle<Image>,
    #[asset(texture_atlas_layout(tile_size_x = 64, tile_size_y = 64, columns = 10, rows = 2))]
    water_puddle_layout: Handle<TextureAtlasLayout>,

    #[asset(path = "images/liquids/water.png")]
    acid_puddle_sprite: Handle<Image>,
    #[asset(texture_atlas_layout(tile_size_x = 64, tile_size_y = 64, columns = 10, rows = 1))]
    acid_puddle_layout: Handle<TextureAtlasLayout>,
}

impl LiquidSprites {
    pub fn droplet(
        &self,
        liquid_type: &LiquidType,
    ) -> (&Handle<Image>, &Handle<TextureAtlasLayout>) {
        match liquid_type {
            LiquidType::Oil => (&self.oil_sprite, &self.oil_layout),
            LiquidType::Water => (&self.water_sprite, &self.water_layout),
            LiquidType::Acid => (&self.acid_sprite, &self.acid_layout),
        }
    }

    pub fn droplet_animation_frames(&self, liquid_type: &LiquidType) -> &'static [usize] {
        match liquid_type {
            LiquidType::Oil => &[0],
            LiquidType::Water => &[0],
            LiquidType::Acid => &[0],
        }
    }

    pub fn droplet_sprite(&self, liquid_type: &LiquidType) -> Sprite {
        let (image, atlas) = self.droplet(liquid_type);
        Sprite {
            image: image.clone(),

            custom_size: Some(Vec2::new(3., 3.)),
            texture_atlas: Some(TextureAtlas::from(atlas.clone())),
            ..default()
        }
    }

    pub fn droplet_frame_queue(&self, liquid_type: &LiquidType) -> AnimationFrameQueue {
        AnimationFrameQueue::new(self.droplet_animation_frames(liquid_type))
    }

    pub fn puddle(
        &self,
        liquid_type: &LiquidType,
    ) -> (&Handle<Image>, &Handle<TextureAtlasLayout>) {
        match liquid_type {
            LiquidType::Oil => (&self.oil_puddle_sprite, &self.oil_puddle_layout),
            LiquidType::Water => (&self.water_puddle_sprite, &self.water_puddle_layout),
            LiquidType::Acid => (&self.acid_puddle_sprite, &self.acid_puddle_layout),
        }
    }

    pub fn puddle_animation_frames(&self, liquid_type: &LiquidType) -> &'static [usize] {
        match liquid_type {
            LiquidType::Oil => &[0],
            LiquidType::Water => &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
            LiquidType::Acid => &[0],
        }
    }

    pub fn stagnant_puddle_animation_frames(&self, liquid_type: &LiquidType) -> &'static [usize] {
        match liquid_type {
            LiquidType::Oil => &[0],
            LiquidType::Water => &[11],
            LiquidType::Acid => &[0],
        }
    }

    pub fn puddle_sprite(&self, liquid_type: &LiquidType) -> Sprite {
        let (image, atlas) = self.puddle(liquid_type);
        Sprite {
            image: image.clone(),

            custom_size: Some(Vec2::new(7., 7.)),
            texture_atlas: Some(TextureAtlas::from(atlas.clone())),
            ..default()
        }
    }

    pub fn puddle_frame_queue(&self, liquid_type: &LiquidType) -> AnimationFrameQueue {
        let mut a = AnimationFrameQueue::new(self.stagnant_puddle_animation_frames(liquid_type));
        a.set_override(self.puddle_animation_frames(liquid_type));
        a
    }

    pub fn bundle(&self, liquid_type: &LiquidType) -> impl Bundle {}
}

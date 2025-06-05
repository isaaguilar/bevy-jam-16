use crate::data::StatusEffect;
use crate::gameplay::animation::AnimationFrameQueue;
use crate::level::components::LEVEL_SCALING;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct StatusSprites {
    #[asset(path = "images/statuseffects/wet.png")]
    wet_sprite: Handle<Image>,
    #[asset(texture_atlas_layout(tile_size_x = 16, tile_size_y = 16, columns = 6, rows = 1))]
    wet_layout: Handle<TextureAtlasLayout>,

    #[asset(path = "images/statuseffects/acidic.png")]
    acidic_sprite: Handle<Image>,
    #[asset(texture_atlas_layout(tile_size_x = 16, tile_size_y = 16, columns = 6, rows = 1))]
    acidic_layout: Handle<TextureAtlasLayout>,

    #[asset(path = "images/statuseffects/burning.png")]
    burning_sprite: Handle<Image>,
    #[asset(texture_atlas_layout(tile_size_x = 16, tile_size_y = 16, columns = 6, rows = 1))]
    burning_layout: Handle<TextureAtlasLayout>,

    #[asset(path = "images/statuseffects/electrified.png")]
    electrified_sprite: Handle<Image>,
    #[asset(texture_atlas_layout(tile_size_x = 16, tile_size_y = 16, columns = 6, rows = 1))]
    electrified_layout: Handle<TextureAtlasLayout>,

    #[asset(path = "images/statuseffects/frozen.png")]
    frozen_sprite: Handle<Image>,
    #[asset(texture_atlas_layout(tile_size_x = 16, tile_size_y = 16, columns = 7, rows = 1))]
    frozen_layout: Handle<TextureAtlasLayout>,

    #[asset(path = "images/statuseffects/oiled.png")]
    oiled_sprite: Handle<Image>,
    #[asset(texture_atlas_layout(tile_size_x = 16, tile_size_y = 16, columns = 6, rows = 1))]
    oiled_layout: Handle<TextureAtlasLayout>,

    #[asset(path = "images/statuseffects/pushed.png")]
    pushed_sprite: Handle<Image>,
    #[asset(texture_atlas_layout(tile_size_x = 16, tile_size_y = 16, columns = 1, rows = 1))]
    pushed_layout: Handle<TextureAtlasLayout>,

    #[asset(path = "images/statuseffects/slowed.png")]
    slowed_sprite: Handle<Image>,
    #[asset(texture_atlas_layout(tile_size_x = 16, tile_size_y = 16, columns = 1, rows = 1))]
    slowed_layout: Handle<TextureAtlasLayout>,
}

impl StatusSprites {
    pub fn status_sprite(
        &self,
        status_effect: &StatusEffect,
    ) -> (&Handle<Image>, &Handle<TextureAtlasLayout>) {
        match status_effect {
            StatusEffect::Wet => (&self.wet_sprite, &self.wet_layout),
            StatusEffect::Ignited => (&self.burning_sprite, &self.burning_layout),
            StatusEffect::Frozen => (&self.frozen_sprite, &self.frozen_layout),
            StatusEffect::Electrified => (&self.electrified_sprite, &self.electrified_layout),
            StatusEffect::Acidic => (&self.acidic_sprite, &self.acidic_layout),
            StatusEffect::Oiled => (&self.oiled_sprite, &self.oiled_layout),
            StatusEffect::Slowed => (&self.slowed_sprite, &self.slowed_layout),
            StatusEffect::Burned => (&self.burning_sprite, &self.burning_layout),
            StatusEffect::Chilled => (&self.frozen_sprite, &self.frozen_layout),
        }
    }

    pub fn status_animation_frames(&self, status_effect: &StatusEffect) -> &'static [usize] {
        match status_effect {
            StatusEffect::Wet => &[0, 1, 2, 3, 4, 5],
            StatusEffect::Ignited => &[0, 1, 2, 3, 4, 5],
            StatusEffect::Frozen => &[0, 1, 2, 3, 4, 5, 6, 6, 6, 6],
            StatusEffect::Electrified => &[0, 1, 2, 3, 4, 5],
            StatusEffect::Acidic => &[0, 1, 2, 3, 4, 5],
            StatusEffect::Oiled => &[0, 1, 2, 3, 4, 5],
            StatusEffect::Slowed => &[0],
            StatusEffect::Burned => &[0, 2, 4],
            StatusEffect::Chilled => &[0, 2, 4],
        }
    }

    pub fn status_bundle(&self, status_effect: &StatusEffect) -> impl Bundle {
        let (image, atlas) = self.status_sprite(status_effect);

        (
            Sprite {
                image: image.clone(),
                custom_size: Some(Vec2::splat(LEVEL_SCALING)),
                texture_atlas: Some(TextureAtlas::from(atlas.clone())),
                ..default()
            },
            AnimationFrameQueue::new(self.status_animation_frames(status_effect)),
        )
    }
}

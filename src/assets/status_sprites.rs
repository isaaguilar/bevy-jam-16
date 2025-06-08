use crate::data::status_effects::StatusEnum;
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
        status_effect: StatusEnum,
    ) -> (&Handle<Image>, &Handle<TextureAtlasLayout>) {
        match status_effect {
            StatusEnum::Wet => (&self.wet_sprite, &self.wet_layout),
            StatusEnum::Burned => (&self.burning_sprite, &self.burning_layout),
            StatusEnum::Frozen => (&self.frozen_sprite, &self.frozen_layout),
            StatusEnum::Electrocuted => (&self.electrified_sprite, &self.electrified_layout),
            StatusEnum::Acidified => (&self.acidic_sprite, &self.acidic_layout),
            StatusEnum::Oiled => (&self.oiled_sprite, &self.oiled_layout),
            StatusEnum::Ignited => (&self.burning_sprite, &self.burning_layout),
            StatusEnum::Chilled => (&self.frozen_sprite, &self.frozen_layout),
        }
    }

    pub fn status_animation_frames(&self, status_effect: StatusEnum) -> &'static [usize] {
        match status_effect {
            StatusEnum::Wet => &[0, 1, 2, 3, 4, 5],
            StatusEnum::Burned => &[0, 1, 2, 3, 4, 5],
            StatusEnum::Frozen => &[0, 2, 4],
            StatusEnum::Electrocuted => &[0, 2, 4],
            StatusEnum::Acidified => &[0, 1, 2, 3, 4, 5, 6, 6, 6, 6],
            StatusEnum::Oiled => &[0, 1, 2, 3, 4, 5],
            StatusEnum::Ignited => &[0, 1, 2, 3, 4, 5],
            StatusEnum::Chilled => &[0, 1, 2, 3, 4, 5],
        }
    }

    pub fn status_bundle(&self, status_effect: StatusEnum) -> impl Bundle {
        let (image, atlas) = self.status_sprite(status_effect);
        let frames = self.status_animation_frames(status_effect);

        (
            Sprite {
                image: image.clone(),
                custom_size: Some(Vec2::splat(LEVEL_SCALING / 2.0)),
                texture_atlas: Some(TextureAtlas::from(atlas.clone())),
                ..default()
            },
            AnimationFrameQueue::new(frames),
        )
    }
}

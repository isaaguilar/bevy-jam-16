use crate::gameplay::animation::AnimationFrameQueue;
use bevy::{platform::collections::HashMap, prelude::*};
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct UiAssets {
    #[asset(
        paths(
            "audio/sound_effects/GenericButton5.wav",
            "audio/sound_effects/GenericButton6.wav",
            "audio/sound_effects/GenericButton7.wav",
        ),
        collection(typed)
    )]
    pub button_hover_sounds: Vec<Handle<AudioSource>>,

    #[asset(path = "audio/sound_effects/button_hover.ogg")]
    pub button_hover_sound: Handle<AudioSource>,
    #[asset(path = "audio/sound_effects/button_click.ogg")]
    pub button_click_sound: Handle<AudioSource>,
    #[asset(
        paths(
            "ui/icon_piston.png",
            "ui/icon_fan.png",
            "ui/icon_spike_pit.png",
            "ui/icon_oil.png",
            "ui/icon_trapdoor.png",
            "ui/icon_ice.png",
            "ui/icon_acid.png",
            "ui/icon_tesla.png",
            "ui/icon_water_bucket.png",
            "ui/icon_flame.png",
            "ui/icon_portal.png",
        ),
        collection(mapped, typed)
    )]
    pub hotbar_icons: HashMap<AssetFileStem, Handle<Image>>,

    #[asset(path = "images/intro.png")]
    pub intro: Handle<Image>,
    #[asset(texture_atlas_layout(tile_size_x = 256, tile_size_y = 128, columns = 12, rows = 1))]
    pub intro_layout: Handle<TextureAtlasLayout>,
    #[asset(path = "ui/spawnbtn.png")]
    pub spawnbtn: Handle<Image>,
    #[asset(texture_atlas_layout(tile_size_x = 128, tile_size_y = 128, columns = 4, rows = 1))]
    pub spawnbtn_layout: Handle<TextureAtlasLayout>,
}

impl UiAssets {
    pub fn intro_bundle(&self) -> impl Bundle {
        let atlas = TextureAtlas::from(self.intro_layout.clone());
        (
            ImageNode {
                image: self.intro.clone(),
                texture_atlas: Some(atlas),
                ..default()
            },
            AnimationFrameQueue::new(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]),
        )
    }
}

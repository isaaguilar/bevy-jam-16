use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct GameAssets {
    #[asset(path = "images/ducky.png")]
    #[asset(image(sampler(filter = nearest)))]
    pub ducky: Handle<Image>,
    #[asset(texture_atlas_layout(
        tile_size_x = 32,
        tile_size_y = 32,
        columns = 6,
        rows = 2,
        padding_x = 1,
        padding_y = 1,
        offset_x = 0,
        offset_y = 0
    ))]
    pub ducky_layout: Handle<TextureAtlasLayout>,
    #[asset(
        paths(
            "audio/sound_effects/step1.ogg",
            "audio/sound_effects/step2.ogg",
            "audio/sound_effects/step3.ogg",
            "audio/sound_effects/step4.ogg",
        ),
        collection(typed)
    )]
    pub steps: Vec<Handle<AudioSource>>,
    #[asset(path = "audio/music/Fluffing A Duck.ogg")]
    pub music: Handle<AudioSource>,
    #[asset(path = "audio/music/Monkeys Spinning Monkeys.ogg")]
    pub credit_music: Handle<AudioSource>,
    #[asset(path = "images/badguy.png")]
    #[asset(image(sampler(filter = nearest)))]
    pub enemysprite: Handle<Image>,
}

impl GameAssets {
    pub fn badguy(&self) -> Handle<Image> {
        self.enemysprite.clone()
    }

    pub fn badguy_layout(&self) -> Handle<TextureAtlasLayout> {
        self.ducky_layout.clone()
    }

    pub fn ducky(&self) -> Handle<Image> {
        self.ducky.clone()
    }
}

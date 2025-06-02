use bevy::platform::collections::HashMap;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct UiAssets {
    #[asset(path = "audio/sound_effects/button_hover.ogg")]
    pub button_hover_sound: Handle<AudioSource>,
    #[asset(path = "audio/sound_effects/button_click.ogg")]
    pub button_click_sound: Handle<AudioSource>,
    #[asset(
        paths(
            "ui/icon_tesla.png",
            "ui/icon_water_bucket.png",
            "ui/icon_trapdoor.png",
            "ui/icon_piston.png",
            "ui/icon_oil.png",
            "ui/icon_spike_pit.png",
        ),
        collection(mapped, typed)
    )]
    pub hotbar_icons: HashMap<AssetFileStem, Handle<Image>>,
}

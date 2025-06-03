use crate::{
    level::{
        components::LevelParent,
        resource::{Level, MAP_TEXT},
    },
    prelude::*,
    screens::Screen,
};

use bevy::prelude::*;
use bevy_composable::{
    app_impl::{ComplexSpawnable, ComponentTreeable},
    wrappers::name,
};

pub(super) fn plugin(_app: &mut App) {
    // empty
}

/// A system that spawns the main level.
pub fn spawn_level(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut level: ResMut<Level>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    *level = Level::from_str(MAP_TEXT);
    commands.compose(
        LevelParent::from_data(&level)
            + name("Level Parent")
            + StateScoped(Screen::Gameplay).store(),
    );
}

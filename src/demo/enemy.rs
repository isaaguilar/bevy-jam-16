use crate::demo::enemy_movement::EnemyControllerBundle;
use crate::prelude::*;
use avian2d::{math::*, prelude::*};

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Enemy>();
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
struct Enemy;

/// The enemy spawn.
pub fn enemy_spawn_bundle(
    max_speed: f32,
    game_assets: &GameAssets,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
) -> impl Bundle {
    // A texture atlas is a way to split a single image into a grid of related images.
    // You can learn more in this example: https://github.com/bevyengine/bevy/blob/latest/examples/2d/texture_atlas.rs
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 6, 2, Some(UVec2::splat(1)), None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    (
        Name::new("Jack"),
        Enemy,
        Sprite {
            image: game_assets.enemysprite.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout,
                index: 0,
            }),
            ..default()
        },
        EnemyControllerBundle::new(Collider::round_rectangle(32., 32., 0.5), Vector::splat(0.))
            .with_movement(max_speed, 0.96),
        Transform::from_scale(Vec2::splat(0.4).extend(1.0))
            .with_translation(Vec3::new(-54.0, -26.0, 1.)),
    )
}

use bevy::{
    color::Color, ecs::component::Component, math::Vec2, reflect::Reflect,
    render::view::Visibility, sprite::Sprite, transform::components::Transform, utils::default,
};
use bevy_composable::{app_impl::ComponentTreeable, tree::ComponentTree, wrappers::name};

use super::resource::Level;

pub const WALL_TOTAL_WIDTH: f32 = 0.05;
pub const FLOOR_TOTAL_HEIGHT: f32 = 0.05;
pub const LEVEL_SCALING: f32 = 10.;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Reflect)]
pub enum WallDirection {
    Left,
    Right,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Component, Reflect)]
pub struct LevelParent;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Component, Reflect)]
pub struct Wall(pub WallDirection);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Component, Reflect)]
pub struct Floor;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Component, Reflect)]
pub struct Ceiling;

impl LevelParent {
    pub fn from_data(level_data: Level) -> ComponentTree {
        let mut level = (LevelParent, Transform::default(), Visibility::default()).store();

        for x in 0..(level_data.width + 1) {
            for y in 0..level_data.height {
                if level_data.walls[x][y] {
                    level = level
                        << (Wall(WallDirection::Left).store()
                            + name("Wall")
                            + rect_sprite(
                                (x as f32 - 0.5 - (WALL_TOTAL_WIDTH / 4.)) * LEVEL_SCALING,
                                y as f32 * LEVEL_SCALING,
                                LEVEL_SCALING,
                                WALL_TOTAL_WIDTH / 2. * LEVEL_SCALING,
                                Color::srgba(0.9, 0.1, 0.1, 1.0),
                            ))
                        << (Wall(WallDirection::Right).store()
                            + name("Wall")
                            + rect_sprite(
                                ((x as f32) - 0.5 + (WALL_TOTAL_WIDTH / 4.)) * LEVEL_SCALING,
                                y as f32 * LEVEL_SCALING,
                                LEVEL_SCALING,
                                WALL_TOTAL_WIDTH / 2. * LEVEL_SCALING,
                                Color::srgba(0.8, 0.3, 0.0, 1.0),
                            ));
                }
            }
        }
        for x in 0..level_data.width {
            for y in 0..(level_data.height + 1) {
                if level_data.floors[x][y] {
                    level = level
                        << (Ceiling.store()
                            + name("Ceiling")
                            + rect_sprite(
                                x as f32 * LEVEL_SCALING,
                                ((y as f32) - 0.5 - FLOOR_TOTAL_HEIGHT / 4.) * LEVEL_SCALING,
                                FLOOR_TOTAL_HEIGHT / 2. * LEVEL_SCALING,
                                LEVEL_SCALING,
                                Color::srgba(0.0, 0.2, 0.8, 1.0),
                            ))
                        << (Floor.store()
                            + name("Floor")
                            + rect_sprite(
                                x as f32 * LEVEL_SCALING,
                                ((y as f32) - 0.5 + FLOOR_TOTAL_HEIGHT / 4.) * LEVEL_SCALING,
                                FLOOR_TOTAL_HEIGHT / 2. * LEVEL_SCALING,
                                LEVEL_SCALING,
                                Color::srgba(0.4, 0.4, 0.0, 1.0),
                            ));
                }
            }
        }
        level
    }
}

pub fn pos(x: f32, y: f32) -> ComponentTree {
    Transform::from_xyz(x, y, 0.).store()
}

pub fn rect_sprite(x: f32, y: f32, h: f32, w: f32, color: Color) -> ComponentTree {
    (
        Sprite {
            color,
            custom_size: Some(Vec2::new(w, h)),
            ..default()
        },
        Visibility::Visible,
    )
        .store()
        + pos(x, y)
}

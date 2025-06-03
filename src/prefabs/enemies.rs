use avian2d::prelude::{Collider, CollisionLayers, RigidBody, ShapeCaster};
use bevy::{
    color::palettes::basic::*,
    math::{Vec2, Vec3},
    prelude::*,
    render::view::Visibility,
    state::state_scoped::StateScoped,
    transform::components::Transform,
};

use bevy_composable::{app_impl::ComponentTreeable, tree::ComponentTree, wrappers::name};
use std::default::Default;

use super::{
    physics::GamePhysicsLayer as GPL,
    utils::{color, image, layout, mesh},
};
use crate::{
    assets::{GameAssets, game_assets},
    demo::{
        enemy_health::{self, EnemyHealth, EnemyHealthBar},
        enemy_movement::{
            EnemyController, MovementAcceleration, MovementBundle, MovementDampingFactor,
            MovementDirection,
        },
    },
    level::components::pos,
    screens::Screen,
};

pub fn basic_trooper() -> ComponentTree {
    name("Minor Trooper")
        << (Transform::from_scale(Vec3::splat(0.2)).store()
            + enemy_requirements(Vec2::new(12., 14.), 35.)
            + image(GameAssets::badguy)
            + layout(GameAssets::badguy_layout)
            << health_bar())
}

pub fn chonkus_trooper() -> ComponentTree {
    name("Minor Trooper")
        << (Transform::from_scale(Vec3::splat(0.25)).store()
            + enemy_requirements(Vec2::new(12., 14.0), 25.)
            + image(GameAssets::ducky)
            + layout(GameAssets::badguy_layout)
            << health_bar())
}

pub fn turbo_trooper() -> ComponentTree {
    name("Minor Trooper")
        << (Transform::from_scale(Vec3::splat(0.15)).store()
            + enemy_requirements(Vec2::new(12., 14.), 45.)
            + image(GameAssets::ducky)
            + layout(GameAssets::badguy_layout)
            << health_bar())
}

pub fn enemy_requirements(size: Vec2, speed: f32) -> ComponentTree {
    (
        // Transform::from_scale(Vec3::splat(scale)),
        StateScoped(Screen::Gameplay),
        EnemyHealth::new(),
        EnemyController,
        MovementDirection::default(),
        RigidBody::Kinematic,
        Visibility::Hidden,
        ShowDelay::new(),
        MovementAcceleration(speed),
        MovementDampingFactor(0.96),
        Collider::round_rectangle(size.x, size.y, 0.5),
        CollisionLayers::new(GPL::Enemy, [GPL::Default, GPL::Level, GPL::Projectiles]),
    )
        .store()
}

pub fn health_bar() -> ComponentTree {
    (
        EnemyHealthBar,
        Transform::from_translation(Vec3::new(0., 14., 0.)),
    )
        .store()
        + mesh(GameAssets::health_bar_mesh)
        + color(GameAssets::health_color)
}

// There's a strange glitch where sprites are the incorrect size when first spawned, so if we hide
// them for 10 ms, they look fine
#[derive(Component, Clone)]
pub struct ShowDelay(pub Timer);

impl ShowDelay {
    pub fn new() -> Self {
        Self(Timer::from_seconds(0.01, TimerMode::Once))
    }
}

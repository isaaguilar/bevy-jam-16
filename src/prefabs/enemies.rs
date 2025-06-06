use super::{
    physics::GamePhysicsLayer as GPL,
    utils::{color, image, layout, mesh},
};
use crate::demo::enemy_health::Bounty;
use crate::{
    assets::GameAssets,
    demo::{
        enemy_health::{EnemyHealth, EnemyHealthBar},
        enemy_movement::{MovementAcceleration, MovementDirection},
    },
    gameplay::animation::AnimationFrameQueue,
    screens::Screen,
};
use avian2d::prelude::{
    Collider, CollisionLayers, GravityScale, LinearDamping, LockedAxes, Mass, RigidBody,
};
use bevy::prelude::*;
use bevy_composable::{app_impl::ComponentTreeable, tree::ComponentTree, wrappers::name};
use std::default::Default;

pub fn basic_trooper() -> ComponentTree {
    let animation = AnimationFrameQueue::new(&[8, 9, 10, 11, 12, 13, 14]);
    name("Minor Trooper") + enemy_requirements(Vec2::new(3., 4.), 35., 10)
        << (Transform::from_scale(Vec3::splat(0.11)).store()
            + animation.store()
            + image(GameAssets::troopers)
            + layout(GameAssets::troopers_layout)
            + Bounty(10).store()
            + Pickable::default().store()
            << health_bar(24.))
}

pub fn chonkus_trooper() -> ComponentTree {
    let animation = AnimationFrameQueue::new(&[16, 16, 16, 17, 17, 17, 18, 18, 18, 19, 19, 19]);
    name("Major Trooper") + enemy_requirements(Vec2::new(4., 5.0), 25., 20)
        << (Transform::from_scale(Vec3::splat(0.16)).store()
            + animation.store()
            + image(GameAssets::troopers)
            + layout(GameAssets::troopers_layout)
            + Pickable::default().store()
            + Bounty(20).store()
            << health_bar(24.))
}

pub fn turbo_trooper() -> ComponentTree {
    let animation = AnimationFrameQueue::new(&[0, 1, 2, 3, 4, 5, 6, 7]);
    name("Turbo Trooper") + enemy_requirements(Vec2::new(2., 3.), 45., 15)
        << (Transform::from_scale(Vec3::splat(0.10)).store()
            + animation.store()
            + image(GameAssets::troopers)
            + layout(GameAssets::troopers_layout)
            + Pickable::default().store()
            << health_bar(24.))
}

pub fn enemy_requirements(size: Vec2, speed: f32, bounty: i32) -> ComponentTree {
    (
        // Transform::from_scale(Vec3::splat(scale)),
        StateScoped(Screen::Gameplay),
        Bounty(bounty),
        EnemyHealth::new(),
        MovementDirection::default(),
        RigidBody::Dynamic,
        Visibility::Hidden,
        ShowDelay::new(),
        MovementAcceleration(speed),
        LinearDamping(1.5),
        GravityScale(1.0),
        Mass(5.),
        LockedAxes::ROTATION_LOCKED,
        Collider::round_rectangle(size.x, size.y, 0.5),
        CollisionLayers::new(GPL::Enemy, [GPL::Default, GPL::Level, GPL::Projectiles]),
    )
        .store()
}

pub fn health_bar(y_offset: f32) -> ComponentTree {
    (
        EnemyHealthBar,
        Transform::from_translation(Vec3::new(0., y_offset, 0.)),
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

use avian2d::prelude::{Collider, CollisionLayers, RigidBody, ShapeCaster};
use bevy::{
    math::{Vec2, Vec3},
    render::view::Visibility,
    state::state_scoped::StateScoped,
    transform::components::Transform,
};
use bevy_composable::{app_impl::ComponentTreeable, tree::ComponentTree, wrappers::name};
use std::default::Default;

use super::{
    physics::GamePhysicsLayer as GPL,
    utils::{image, layout},
};
use crate::{
    assets::GameAssets,
    demo::enemy_movement::{
        EnemyController, MovementAcceleration, MovementBundle, MovementDampingFactor,
        MovementDirection,
    },
    level::components::pos,
    screens::Screen,
};

pub fn basic_trooper() -> ComponentTree {
    name("Minor Trooper") + enemy_requirements(Vec2::new(3., 3.5), 35.)
        << Transform::from_scale(Vec3::splat(0.2)).store()
            + image(GameAssets::badguy)
            + layout(GameAssets::badguy_layout)
}

pub fn chonkus_trooper() -> ComponentTree {
    name("Minor Trooper") + enemy_requirements(Vec2::new(4., 5.0), 25.)
        << Transform::from_scale(Vec3::splat(0.25)).store()
            + image(GameAssets::ducky)
            + layout(GameAssets::badguy_layout)
}

pub fn turbo_trooper() -> ComponentTree {
    name("Minor Trooper") + enemy_requirements(Vec2::new(2., 2.5), 45.)
        << Transform::from_scale(Vec3::splat(0.15)).store()
            + image(GameAssets::badguy)
            + layout(GameAssets::badguy_layout)
}

pub fn enemy_requirements(size: Vec2, speed: f32) -> ComponentTree {
    (
        StateScoped(Screen::Gameplay),
        EnemyController,
        MovementDirection::default(),
        RigidBody::Kinematic,
        Visibility::Visible,
        MovementAcceleration(speed),
        MovementDampingFactor(0.96),
        Collider::round_rectangle(size.x, size.y, 0.5),
        CollisionLayers::new(GPL::Enemy, [GPL::Default, GPL::Level, GPL::Projectiles]),
    )
        .store()
}

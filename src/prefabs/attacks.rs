use avian2d::prelude::{
    Collider, CollisionEventsEnabled, CollisionLayers, LinearVelocity, Mass, RigidBody, Sensor,
};
use bevy::{
    color::{
        Color,
        palettes::css::{AQUA, BROWN, LIME},
    },
    math::Vec2,
    render::view::Visibility,
    sprite::Sprite,
    utils::default,
};
use bevy_composable::{app_impl::ComponentTreeable, tree::ComponentTree, wrappers::name};

use super::physics::GamePhysicsLayer as GPL;
use crate::data::projectiles::{Droplet, Lifetime, LiquidType, Puddle};

pub fn droplet(liquid: LiquidType) -> ComponentTree {
    (
        Sprite {
            color: match liquid {
                LiquidType::Water => AQUA,
                LiquidType::Oil => BROWN,
                LiquidType::Acid => LIME,
            }
            .into(),
            custom_size: Some(Vec2::new(3., 3.)),
            ..default()
        },
        Visibility::Visible,
        Droplet(liquid),
        Collider::circle(1.5),
        CollisionLayers::new(GPL::Projectiles, [GPL::Enemy, GPL::Level]),
        RigidBody::Dynamic,
        Mass(0.1),
        CollisionEventsEnabled,
    )
        .store()
        + name("Droplet")
}

pub fn puddle(liquid: LiquidType) -> ComponentTree {
    (
        Sprite {
            color: match liquid {
                LiquidType::Water => AQUA,
                LiquidType::Oil => BROWN,
                LiquidType::Acid => LIME,
            }
            .into(),
            custom_size: Some(Vec2::new(7., 1.5)),
            ..default()
        },
        Visibility::Visible,
        Puddle(liquid),
        Collider::ellipse(3.5, 0.75),
        CollisionLayers::new(GPL::Projectiles, [GPL::Enemy, GPL::Level]),
        Sensor,
        RigidBody::Kinematic,
        LinearVelocity(Vec2::new(0., -20.)),
        CollisionEventsEnabled,
        Lifetime::new(4.0),
    )
        .store()
        + name("Puddle")
}

use avian2d::prelude::{
    Collider, CollisionEventsEnabled, CollisionLayers, LinearVelocity, RigidBody,
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
use bevy_composable::{app_impl::ComponentTreeable, tree::ComponentTree};

use super::physics::GamePhysicsLayer as GPL;
use crate::data::projectiles::{Droplet, LiquidType};

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
        RigidBody::Kinematic,
        CollisionEventsEnabled,
        LinearVelocity(Vec2::NEG_Y),
    )
        .store()
}

//pub fn puddle(liquid: LiquidType) -> ComponentTree {}

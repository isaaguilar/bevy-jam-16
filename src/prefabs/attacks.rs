use avian2d::prelude::{
    Collider, CollisionEventsEnabled, CollisionLayers, LinearVelocity, Mass, RigidBody, Sensor,
};
use bevy::{ecs::system::Res, math::Vec2, render::view::Visibility};
use bevy_composable::{app_impl::ComponentTreeable, tree::ComponentTree, wrappers::name};

use super::physics::GamePhysicsLayer as GPL;
use crate::assets::LiquidSprites;
use crate::data::projectiles::{Droplet, LiquidType, Puddle};
use crate::gameplay::shared_systems::Lifetime;

pub fn droplet(liquid: LiquidType, liquid_sprites: &Res<LiquidSprites>) -> ComponentTree {
    (
        liquid_sprites.droplet_sprite(&liquid),
        liquid_sprites.droplet_frame_queue(&liquid),
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

pub fn puddle(liquid: LiquidType, liquid_sprites: &Res<LiquidSprites>) -> ComponentTree {
    (
        liquid_sprites.puddle_sprite(&liquid),
        liquid_sprites.puddle_frame_queue(&liquid),
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

use std::cell;

use avian2d::prelude::{LinearVelocity, OnCollisionStart, Sensor};
use bevy::{
    ecs::{
        entity::Entity,
        event::{EventReader, EventWriter},
        observer::Trigger,
        query::With,
        system::{Commands, Query, Res},
    },
    math::{Vec2, Vec3Swizzles},
    prelude::warn,
    time::Time,
    transform::components::{GlobalTransform, Transform},
};
use bevy_composable::app_impl::ComplexSpawnable;

use super::attacks::{ApplyAttackEffect, DropLiquid};
use crate::{
    data::{
        Tower,
        projectiles::{DamageType, Droplet, Puddle},
    },
    demo::enemy_health::{EnemyHealth, TryDamageToEnemy},
    gameplay::{animation::AnimationFrameQueue, shared_systems::Lifetime},
    level::{
        components::{Architecture, pos},
        resource::CellDirection,
    },
    prefabs::attacks::{droplet, puddle},
};

pub fn drop_liquids(
    mut events: EventReader<DropLiquid>,
    mut commands: Commands,
    mut towers: Query<(
        &Tower,
        &GlobalTransform,
        &CellDirection,
        &mut AnimationFrameQueue,
    )>,
) {
    for DropLiquid(e, liquid) in events.read() {
        let Ok((tower, global_transform, cell_direction, mut animation)) = towers.get_mut(*e)
        else {
            warn!("Tower not found in dispatch_attack_effects");
            return;
        };
        let loc = global_transform.to_scale_rotation_translation().2.xy();
        commands.compose(droplet(*liquid) + pos(loc.x, loc.y));
        animation.set_override(cell_direction.attack_frames(&tower));
    }
}

pub fn splat_droplets(
    trigger: Trigger<OnCollisionStart>,
    sensors: Query<(), With<Sensor>>,
    droplets: Query<(&Transform, &Droplet)>,
    mut commands: Commands,
) {
    let droplet = trigger.target();
    let other = trigger.collider;

    // We don't want droplets to do things when they hit sensors
    if sensors.get(other).is_err() {
        if let Ok((transform, Droplet(liquid))) = droplets.get(droplet) {
            let loc = transform.translation;
            commands.entity(droplet).despawn();
            commands.compose(puddle(*liquid) + pos(loc.x, loc.y));
        }
    }
}

pub fn puddle_attacks(
    trigger: Trigger<OnCollisionStart>,
    enemies: Query<(), With<EnemyHealth>>,
    puddles: Query<&Puddle>,
    mut attack_events: EventWriter<ApplyAttackEffect>,
) {
    let puddle = trigger.target();
    let other = trigger.collider;

    if enemies.get(other).is_ok() {
        if let Ok(Puddle(liquid)) = puddles.get(puddle) {
            for effect in liquid.contact_effects() {
                attack_events.write(ApplyAttackEffect {
                    target: other,
                    source: puddle,
                    effect,
                });
            }
        }
    }
}

pub fn stop_dropping_puddles(
    trigger: Trigger<OnCollisionStart>,
    level_parts: Query<(), With<Architecture>>,
    mut droplets: Query<&mut LinearVelocity, With<Puddle>>,
) {
    let puddle = trigger.target();
    let other = trigger.collider;

    if level_parts.get(other).is_ok() {
        if let Ok(mut vel) = droplets.get_mut(puddle) {
            vel.0 = Vec2::ZERO;
        }
    }
}

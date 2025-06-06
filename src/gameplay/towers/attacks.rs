use avian2d::prelude::{Collisions, LinearVelocity, OnCollisionStart, Sensor};
use bevy::{
    ecs::{
        entity::Entity,
        event::{Event, EventReader, EventWriter},
        hierarchy::Children,
        observer::Trigger,
        query::With,
        system::{Commands, Query, Res},
    },
    math::{Vec2, Vec3Swizzles},
    prelude::warn,
    reflect::Reflect,
    time::Time,
    transform::components::{GlobalTransform, Transform},
};
use bevy_composable::app_impl::ComplexSpawnable;

use crate::{
    data::{
        Tower,
        projectiles::{AttackEffect, AttackType, DamageType, Droplet, LiquidType, Puddle},
    },
    demo::enemy_health::{EnemyHealth, TryDamageToEnemy},
    gameplay::animation::AnimationFrameQueue,
    level::{
        components::{Architecture, pos},
        resource::CellDirection,
    },
    prefabs::attacks::{droplet, puddle},
};

#[derive(Event, Reflect, Debug, PartialEq, Clone)]
pub struct AttackEnemiesInContact(pub Entity, pub Vec<AttackEffect>);

#[derive(Event, Reflect, Debug, PartialEq, Clone, Copy)]
pub struct DropLiquid(pub Entity, pub LiquidType);

use super::common::{TowerFired, TowerTriggerRange};

pub fn dispatch_attack_effects(
    mut fire_events: EventReader<TowerFired>,
    mut contact_events: EventWriter<AttackEnemiesInContact>,
    mut drop_events: EventWriter<DropLiquid>,
    mut towers: Query<(
        &Tower,
        &Children,
        &GlobalTransform,
        &CellDirection,
        &mut AnimationFrameQueue,
    )>,
    ranges: Query<(), With<TowerTriggerRange>>,
) {
    for event in fire_events.read() {
        let Ok((tower, children, global_pos, cell_direction, mut animation)) =
            towers.get_mut(event.0)
        else {
            warn!("Tower not found in dispatch_attack_effects");
            return;
        };

        animation.set_override(cell_direction.attack_frames(&tower));

        match tower.attack_def() {
            AttackType::EntireCell(attack_effects) => {
                contact_events.write(AttackEnemiesInContact(
                    *children
                        .iter()
                        .filter(|w| ranges.get(**w).is_ok())
                        .next()
                        .unwrap(),
                    attack_effects,
                ));
            }
            AttackType::Contact(attack_effects) => todo!(),
            AttackType::DropsLiquid(liquid_type) => {
                drop_events.write(DropLiquid(event.0, liquid_type));
            }
            AttackType::ModifiesSelf => todo!(),
        }
    }
}

pub fn attack_contact_enemies(
    mut events: EventReader<AttackEnemiesInContact>,
    mut damage_events: EventWriter<TryDamageToEnemy>,
    collisions: Collisions,
    enemies: Query<(), With<EnemyHealth>>,
) {
    for &AttackEnemiesInContact(sensor_entity, ref damage_def) in events.read() {
        for effect in damage_def {
            match effect {
                AttackEffect::Damage(damage_type) => {
                    let enemies = collisions
                        .entities_colliding_with(sensor_entity)
                        .filter(|w| enemies.get(*w).is_ok());
                    for enemy in enemies {
                        damage_events.write(TryDamageToEnemy {
                            damage_range: (0.05, 0.1), // TODO: Add damage details
                            damage_type: *damage_type,
                            enemy: enemy,
                        });
                    }
                }
                AttackEffect::Push => todo!(),
                AttackEffect::Status(status_effect) => todo!(),
            }
        }
    }
}

pub fn drop_liquids(
    mut events: EventReader<DropLiquid>,
    mut commands: Commands,
    towers: Query<&GlobalTransform, With<Tower>>,
) {
    for DropLiquid(e, liquid) in events.read() {
        let Ok(global_transform) = towers.get(*e) else {
            warn!("Tower not found in drop_liquids");
            return;
        };

        let loc = global_transform.to_scale_rotation_translation().2.xy();
        commands.compose(droplet(*liquid) + pos(loc.x, loc.y));
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

pub fn puddle_damage(
    trigger: Trigger<OnCollisionStart>,
    enemies: Query<(), With<EnemyHealth>>,
    puddles: Query<&Puddle>,
    mut damage_events: EventWriter<TryDamageToEnemy>,
) {
    let droplet = trigger.target();
    let other = trigger.collider;

    if enemies.get(other).is_ok() {
        if let Ok(Puddle(liquid)) = puddles.get(droplet) {
            damage_events.write(TryDamageToEnemy {
                damage_range: (0.1, 0.2),
                damage_type: DamageType::Chemical,
                enemy: other,
            });
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

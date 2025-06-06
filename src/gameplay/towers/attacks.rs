use avian2d::prelude::Collisions;
use bevy::{
    ecs::{
        entity::Entity,
        event::{Event, EventReader, EventWriter},
        hierarchy::Children,
        query::With,
        system::Query,
    },
    reflect::Reflect,
    transform::components::GlobalTransform,
};

use crate::{
    data::{
        Tower,
        projectiles::{AttackEffect, AttackType, LiquidType},
    },
    demo::enemy_health::{EnemyHealth, TryDamageToEnemy},
    gameplay::status_effects::common::TryApplyStatus,
};

#[derive(Event, Reflect, Debug, PartialEq, Clone)]
pub struct ApplyAttackEffect {
    pub target: Entity,
    pub source: Entity,
    pub effect: AttackEffect,
}

#[derive(Event, Reflect, Debug, PartialEq, Clone)]
pub struct AttackEnemiesInContact(pub Entity, pub Vec<AttackEffect>);

#[derive(Event, Reflect, Debug, PartialEq, Clone, Copy)]
pub struct DropLiquid(pub Entity, pub LiquidType);

use super::common::{TowerFired, TowerTriggerRange};

pub fn do_tower_attacks(
    mut fire_events: EventReader<TowerFired>,
    mut contact_events: EventWriter<AttackEnemiesInContact>,
    mut drop_events: EventWriter<DropLiquid>,
    towers: Query<(&Tower, &Children, &GlobalTransform)>,
    ranges: Query<(), With<TowerTriggerRange>>,
) {
    for event in fire_events.read() {
        let (tower, children, global_pos) = towers.get(event.0).unwrap();
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

pub fn dispatch_attack_effects(
    mut attackeffect_events: EventReader<ApplyAttackEffect>,
    mut damage_events: EventWriter<TryDamageToEnemy>,
    mut status_events: EventWriter<TryApplyStatus>,
) {
    for (ApplyAttackEffect {
        target,
        source,
        effect,
    }) in attackeffect_events.read()
    {
        match effect {
            AttackEffect::Damage(damage_type) => {
                damage_events.write(TryDamageToEnemy {
                    damage_range: (0.05, 0.1), // TODO: Add damage details
                    damage_type: *damage_type,
                    enemy: *target,
                });
            }
            AttackEffect::Push => todo!(),
            AttackEffect::Status(status_effect) => {
                status_events.write(TryApplyStatus {
                    status: *status_effect,
                    enemy: *target,
                    strength: 1, // TODO: Update with strength system
                });
            }
        }
    }
}

pub fn attack_contact_enemies(
    mut events: EventReader<AttackEnemiesInContact>,
    mut attack_events: EventWriter<ApplyAttackEffect>,
    collisions: Collisions,
    enemies: Query<(), With<EnemyHealth>>,
) {
    for &AttackEnemiesInContact(sensor_entity, ref damage_def) in events.read() {
        let enemies: Vec<_> = collisions
            .entities_colliding_with(sensor_entity)
            .filter(|w| enemies.get(*w).is_ok())
            .collect();
        for effect in damage_def {
            for enemy in &enemies {
                attack_events.write(ApplyAttackEffect {
                    target: *enemy,
                    source: sensor_entity,
                    effect: effect.clone(),
                });
            }
        }
    }
}

use avian2d::prelude::{CollisionEnded, CollisionStarted, Collisions};
use bevy::{
    ecs::{
        component::Component,
        entity::Entity,
        event::{Event, EventReader, EventWriter},
        hierarchy::{ChildOf, Children},
        query::{Changed, With, Without},
        system::{Commands, Query, Res},
    },
    reflect::Reflect,
    time::{Time, Timer},
};

use crate::{data::Tower, demo::enemy_movement::EnemyController};

// Attached to sensor colliders that detect enemies for towers
#[derive(Copy, Clone, Debug, Reflect, Component, PartialEq, Eq)]
pub struct TowerTriggerRange;

// Attached to sensor colliders that detect enemies for towers that drop things
#[derive(Copy, Clone, Debug, Reflect, Component, PartialEq, Eq)]
pub struct TowerTriggerNeedsGravity;

// Attached to towers that cannot fire until the timer is up
#[derive(Clone, Debug, Reflect, Component, PartialEq, Eq)]
pub struct Cooldown(pub Timer);

// Signal component attached to towers that have something to shoot at
#[derive(Copy, Clone, Debug, Reflect, Component, PartialEq, Eq)]
pub struct TowerHasTargets;

#[derive(Copy, Clone, Debug, Reflect, Event, PartialEq, Eq)]
pub struct TowerFired(pub Entity);

pub fn add_tower_targets_from_zone(
    mut collision_events: EventReader<CollisionStarted>,
    trigger_zones: Query<&ChildOf, With<TowerTriggerRange>>,
    towers: Query<Entity, (With<Tower>, Without<TowerHasTargets>)>,
    enemies: Query<Entity, With<EnemyController>>,
    mut commands: Commands,
) {
    for event in collision_events.read() {
        let sensor_tower_collision = match (
            trigger_zones.get(event.0),
            trigger_zones.get(event.1),
            enemies.get(event.0),
            enemies.get(event.1),
        ) {
            (Ok(a), Err(_), Err(_), Ok(b)) | (Err(_), Ok(a), Ok(b), Err(_)) => Some((a, b)),
            _ => None,
        };

        if let Some((trigger_zone, enemy)) = sensor_tower_collision {
            commands.entity(trigger_zone.0).insert(TowerHasTargets);
        }
    }
}

pub fn remove_tower_targets(
    trigger_zones: Query<Entity, With<TowerTriggerRange>>,
    towers: Query<(Entity, &Children), (With<Tower>, With<TowerHasTargets>)>,
    collisions: Collisions,
    enemies: Query<Entity, With<EnemyController>>,
    mut commands: Commands,
) {
    for (e, children) in towers.iter() {
        let anyone_in_trigger_zones: bool = children
            .iter()
            .filter(|child_entity| trigger_zones.contains(**child_entity))
            .map(|trigger_zone| {
                collisions
                    .entities_colliding_with(*trigger_zone)
                    .next()
                    .is_some()
            })
            .fold(true, |a, b| a && b);
        if !anyone_in_trigger_zones {
            commands.entity(e).remove::<TowerHasTargets>();
        }
    }
}

pub fn towers_fire(
    towers: Query<(Entity, &Tower), (With<TowerHasTargets>, Without<Cooldown>)>,
    mut fire_events: EventWriter<TowerFired>,
    mut commands: Commands,
) {
    for (e, tower) in towers.iter() {
        println!("Firing {:?}!", tower);
        commands.entity(e).insert(Cooldown::new(tower.cooldown()));
        fire_events.write(TowerFired(e));
    }
}

pub fn remove_cooldown(
    towers: Query<(Entity, &Cooldown), Changed<Cooldown>>,
    mut commands: Commands,
) {
    for (e, cooldown) in towers.iter() {
        if cooldown.0.finished() {
            commands.entity(e).remove::<Cooldown>();
        }
    }
}

pub fn tick_cooldown(mut cooldown_timers: Query<&mut Cooldown>, time: Res<Time>) {
    for mut timer in cooldown_timers.iter_mut() {
        timer.0.tick(time.delta());
    }
}

impl Cooldown {
    pub fn new(time_secs: f32) -> Self {
        Self(Timer::from_seconds(time_secs, bevy::time::TimerMode::Once))
    }
}

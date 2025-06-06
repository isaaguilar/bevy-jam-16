use bevy::{
    ecs::{
        entity::Entity,
        event::{Event, EventReader, EventWriter},
        query::With,
        schedule::{IntoScheduleConfigs, ScheduleConfigs},
        system::{Commands, Local, Query, Res, ScheduleSystem},
    },
    reflect::Reflect,
    time::{Time, Timer},
};
use std::marker::PhantomData;

use crate::{
    data::status_effects::{StatusEffect, StatusEffectTrait, StatusEnum, duration_multiplier},
    demo::enemy_health::{EnemyHealth, TryDamageToEnemy},
};

use super::display::StatusAnimation;

#[derive(Reflect, Debug, Event, PartialEq, Eq, Clone, Copy)]
pub struct TryApplyStatus {
    pub status: StatusEnum,
    pub enemy: Entity,
    pub strength: usize,
}

#[derive(Reflect, Debug, Event, PartialEq, Eq)]
pub struct ApplyStatus<T: StatusEffectTrait> {
    pub enemy: Entity,
    pub strength: usize,
    #[reflect(ignore)]
    _phantom: PhantomData<T>,
}

#[derive(Reflect, Debug, Event, PartialEq, Eq)]
pub struct StatusTimeout<T: StatusEffectTrait> {
    pub enemy: Entity,
    pub strength: usize,
    #[reflect(ignore)]
    _phantom: PhantomData<T>,
}

pub fn periodic_damage<T: StatusEffectTrait>(dps: f32) -> ScheduleConfigs<ScheduleSystem> {
    let damage_per_tick = dps / 2.;
    (move |enemies: Query<Entity, (With<EnemyHealth>, With<StatusEffect<T>>)>,
           mut cooldown: Local<Timer>,
           time: Res<Time>,
           mut damage_events: EventWriter<TryDamageToEnemy>| {
        cooldown.tick(time.delta());
        if (*cooldown).just_finished() {
            for enemy in enemies.iter() {
                damage_events.write(TryDamageToEnemy {
                    damage_range: (damage_per_tick, damage_per_tick),
                    damage_type: T::damage_element(),
                    enemy: enemy,
                });
            }
        }
    })
    .into_configs()
}

pub fn dispatch_typed_events<T: StatusEffectTrait>(
    mut reader: EventReader<TryApplyStatus>,
    mut writer: EventWriter<ApplyStatus<T>>,
) {
    for TryApplyStatus {
        status,
        enemy,
        strength,
    } in reader
        .read()
        .filter(|w| w.status == T::corresponding_enum())
    {
        writer.write(ApplyStatus::new(*enemy, *strength));
    }
}

pub fn apply_status_effects<T: StatusEffectTrait>(
    mut events: EventReader<ApplyStatus<T>>,
    mut commands: Commands,
) {
    for ApplyStatus {
        enemy,
        strength,
        _phantom,
    } in events.read()
    {
        commands.entity(*enemy).insert(StatusEffect::<T>::new(
            *strength,
            T::base_duration() * duration_multiplier(*strength),
        ));
    }
}

pub fn tick_statuses<T: StatusEffectTrait>(
    mut enemies: Query<&mut StatusEffect<T>, With<EnemyHealth>>,
    time: Res<Time>,
) {
    for mut enemy in enemies.iter_mut() {
        enemy.duration.tick(time.delta());
    }
}

pub fn timeout_statuses<T: StatusEffectTrait>(
    mut enemies: Query<(Entity, &StatusEffect<T>)>,
    mut commands: Commands,
    mut events: EventWriter<StatusTimeout<T>>,
) {
    for (enemy, status) in enemies.iter() {
        if status.duration.finished() {
            events.write(StatusTimeout::new(enemy, status.strength));
            commands.entity(enemy).remove::<StatusEffect<T>>();
        }
    }
}

impl<T: StatusEffectTrait> ApplyStatus<T> {
    pub fn new(enemy: Entity, strength: usize) -> ApplyStatus<T> {
        ApplyStatus {
            enemy,
            strength,
            _phantom: PhantomData,
        }
    }
}

impl<T: StatusEffectTrait> StatusTimeout<T> {
    pub fn new(enemy: Entity, strength: usize) -> StatusTimeout<T> {
        StatusTimeout {
            enemy,
            strength,
            _phantom: PhantomData,
        }
    }
}

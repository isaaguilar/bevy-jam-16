use avian2d::prelude::{OnCollisionEnd, OnCollisionStart};
use bevy::prelude::*;
use rand::Rng;

use crate::assets::game_assets::HEALTH_BAR_WIDTH;
use crate::assets::{GameAssets, StatusSprites, game_assets};
use crate::data::{Ailments, StatusEffect, get_ailment, get_collison};
use crate::data::{Tower, TowerCollision};
use crate::{AppSystems, PausableSystems};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            active_tower_collision,
            status_effect_ailments,
            update_health_bars,
            animate_status_effects,
            enemy_despawn,
        )
            .in_set(PausableSystems)
            .in_set(AppSystems::Update),
    );
    app.add_observer(start_collision_damage_event);
    app.add_observer(stop_collision_damage_event);
    app.add_observer(record_damage);
    app.add_observer(record_init_collision_damage);
    app.add_observer(display_status);
    // Debug picker helpers
    //app.add_observer(pick_enemy_to_do_damage_example);
    // app.add_observer(pick_enemy_to_add_status_example);
}

#[derive(Component, Default, Clone, Copy, PartialEq, Reflect)]
pub struct EnemyHealth(pub f32);

#[derive(Component, Default, Clone, Copy, PartialEq, Reflect)]
pub struct EnemyChild;

#[derive(Component, Default, Clone, Copy, PartialEq, Reflect)]
pub struct EnemyHealthBar;

#[derive(Event, Debug)]
pub struct RecordInitCollisionDamage {
    tower_entity: Entity,
}

#[derive(Event, Debug)]
pub struct RecordDamage {
    min: f32,
    max: f32,
}

#[derive(Event, Debug)]
pub struct DisplayStatusEvent {
    status_effect: StatusEffect,
    timer: StatusAnimationTimer,
    ttl: StatusAnimationTtl,
}

#[derive(Component, Clone, Debug)]
pub struct StatusAnimationTimer(Timer);

#[derive(Component, Clone, Debug)]
pub struct StatusAnimationTtl(Timer);

impl EnemyHealth {
    pub fn new() -> Self {
        Self(1.0)
    }
}

pub fn enemy_despawn(enemies: Query<(Entity, &EnemyHealth)>, mut commands: Commands) {
    for (entity, health) in enemies {
        if health.0 <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

pub fn animate_status_effects(
    time: Res<Time>,
    mut statuses: Query<(
        Entity,
        &mut Sprite,
        &mut StatusAnimationTimer,
        &mut StatusAnimationTtl,
    )>,
    mut commands: Commands,
) {
    for (entity, mut sprite, mut timer, mut ttl) in statuses.iter_mut() {
        ttl.0.tick(time.delta());
        if ttl.0.just_finished() || ttl.0.finished() {
            commands.entity(entity).despawn();
            continue;
        }
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            let Some(atlas) = &mut sprite.texture_atlas else {
                continue;
            };
            if atlas.index >= 5 {
                atlas.index = 0
            } else {
                atlas.index += 1
            }
        }
    }
}

pub fn update_health_bars(
    mut bars: Query<(Entity, &mut Transform), With<EnemyHealthBar>>,
    enemies: Query<&EnemyHealth>,
    parents: Query<&ChildOf>,
) {
    for (health_bar_entity, mut transform) in bars.iter_mut() {
        let Ok(health_bar_parent) = parents.get(health_bar_entity) else {
            continue;
        };
        let Ok(enemy_entity) = parents.get(health_bar_parent.0) else {
            continue;
        };
        let Ok(enemy) = enemies.get(enemy_entity.0) else {
            continue;
        };

        transform.scale.x = enemy.0;
        transform.translation.x = -(HEALTH_BAR_WIDTH * (1.0 - enemy.0)) / 2.0;
    }
}

fn start_collision_damage_event(
    trigger: Trigger<OnCollisionStart>,
    towers: Query<&Tower>,
    mut commands: Commands,
) {
    let enemy_target = trigger.event().collider;

    let Ok(tower) = towers.get(trigger.target()) else {
        warn!(target=?trigger.target(), "Tower not found");
        return;
    };

    let tower_collision = get_collison(&tower);

    // Perform a immediate damage hit
    commands.trigger_targets(
        RecordInitCollisionDamage {
            tower_entity: trigger.target(),
        },
        enemy_target,
    );

    // Add a collision entity that deals damage on a timer while collision is active
    commands.entity(enemy_target).insert(tower.clone());
    commands.entity(enemy_target).insert(tower_collision);
}

fn stop_collision_damage_event(trigger: Trigger<OnCollisionEnd>, mut commands: Commands) {
    let enemy_target = trigger.event().collider;

    commands.entity(enemy_target).remove::<Tower>();
    commands.entity(enemy_target).remove::<TowerCollision>();
}

pub fn record_init_collision_damage(
    trigger: Trigger<RecordInitCollisionDamage>,
    towers: Query<&Tower>,
    mut enemies: Query<&mut EnemyHealth>,
) {
    let Ok(mut parent_health) = enemies.get_mut(trigger.target()) else {
        warn!(target=?trigger.target(), "Enemy target not found");
        return;
    };

    let Ok(tower) = towers.get(trigger.tower_entity) else {
        warn!(target=?trigger.target(), "Tower not found");
        return;
    };

    let tower_collision = get_collison(tower);
    let rng = &mut rand::rng();
    let damage = rng.random_range(tower_collision.min_damage..=tower_collision.max_damage);

    parent_health.0 -= damage;
    parent_health.0 = parent_health.0.clamp(0.0, 1.0);
}

// This is the timer function that deals damage when activley in collision
fn active_tower_collision(
    time: Res<Time>,
    mut enemies: Query<(Entity, &mut TowerCollision)>,
    mut commands: Commands,
) {
    for (entity, mut tower_collision) in enemies.iter_mut() {
        tower_collision.iframe.tick(time.delta());
        if tower_collision.iframe.just_finished() {
            commands.trigger_targets(
                RecordDamage {
                    min: tower_collision.min_damage,
                    max: tower_collision.max_damage,
                },
                entity,
            );
        }
    }
}

// This is the timer function that deals damage when an ailment is present
fn status_effect_ailments(
    time: Res<Time>,
    mut enemies: Query<(Entity, &mut Ailments)>,
    mut commands: Commands,
) {
    for (entity, mut ailments) in enemies.iter_mut() {
        ailments.damage_timer.tick(time.delta());
        if ailments.damage_timer.just_finished() {
            commands.trigger_targets(
                RecordDamage {
                    min: ailments.min_damage,
                    max: ailments.max_damage,
                },
                entity,
            );
        }

        if !ailments.ailment_timer.finished() {
            ailments.ailment_timer.tick(time.delta());
        } else {
            commands.entity(entity).remove::<StatusEffect>();
            commands.entity(entity).remove::<Ailments>();
        }
    }
}

// An ailment is triggered on a timer
pub fn record_damage(trigger: Trigger<RecordDamage>, mut enemies: Query<&mut EnemyHealth>) {
    let Ok(mut parent_health) = enemies.get_mut(trigger.target()) else {
        warn!(target=?trigger.target(), "Enemy target not found");
        return;
    };

    let rng = &mut rand::rng();
    let damage = rng.random_range(trigger.min..=trigger.max);

    parent_health.0 -= damage;
    parent_health.0 = parent_health.0.clamp(0.0, 1.0);

    debug!(
        "Enemy {:?} has {} health",
        trigger.target(),
        parent_health.0
    );
}

pub fn display_status(
    trigger: Trigger<DisplayStatusEvent>,
    statuses: Query<(Entity, &ChildOf), With<StatusAnimationTtl>>,

    sprites: Option<Res<StatusSprites>>,
    mut commands: Commands,
) {
    let sprites = sprites.expect("GameAssets should be available");
    let e = trigger.target();
    for (entity, parent) in statuses.iter() {
        if parent.0 == e {
            commands.entity(entity).despawn();
        }
    }

    let status_sprite_bundle = sprites.status_bundle(&trigger.status_effect);

    commands.entity(e).with_children(|p| {
        p.spawn((
            trigger.timer.clone(),
            trigger.ttl.clone(),
            status_sprite_bundle,
            Transform::from_translation(Vec3::new(15.0, 12.0, 1.0)),
        ));
    });
}

// Debug Helpers

fn pick_enemy_to_do_damage_example(
    trigger: Trigger<Pointer<Pressed>>,
    world: bevy::ecs::world::DeferredWorld,
    query: Query<Entity, With<Tower>>,
    mut commands: Commands,
) {
    todo!();
    // let tower_entity = match query.iter().next() {
    //     Some(e) => e,
    //     None => commands.spawn(Tower::SpikePit).id(),
    // };

    // let child_of_enemy_target = trigger.target();

    // let Some(enemy_target) = world.get::<ChildOf>(child_of_enemy_target) else {
    //     return;
    // };

    // commands
    //     .entity(enemy_target.0)
    //     .insert((StatusEffect::Acidic, ailment.clone()));

    // commands.trigger_targets(Attack { tower_entity }, enemy_target.0);
}

fn pick_enemy_to_add_status_example(
    trigger: Trigger<Pointer<Pressed>>,
    world: bevy::ecs::world::DeferredWorld,
    mut commands: Commands,
    // assets: Res<GameAssets>,
) {
    // This is the enemy sprite
    let child_of_enemy_target = trigger.target();

    // This is the enemy controller
    let Some(enemy_target) = world.get::<ChildOf>(child_of_enemy_target) else {
        return;
    };

    /*
    Wet,
    Burning,
    Frozen,
    Electrified,
    Acidic,
    Oiled,
    Slowed,
    Pushed,

     */
    let mut rng = rand::rng();
    let status_effect = match rng.random_range(0..8) {
        1 => StatusEffect::Burning,
        2 => StatusEffect::Frozen,
        3 => StatusEffect::Electrified,
        4 => StatusEffect::Acidic,
        5 => StatusEffect::Oiled,
        6 => StatusEffect::Slowed,
        7 => StatusEffect::Pushed,
        _ => StatusEffect::Wet,
    };

    let ailment = get_ailment(status_effect);

    let ttl_timer = ailment.ailment_timer.clone();

    commands
        .entity(enemy_target.0)
        .insert((status_effect, ailment.clone()));

    commands.trigger_targets(
        DisplayStatusEvent {
            status_effect: status_effect,
            timer: StatusAnimationTimer(Timer::from_seconds(0.15, TimerMode::Repeating)),
            ttl: StatusAnimationTtl(ttl_timer),
        },
        child_of_enemy_target,
    );
}

use avian2d::prelude::OnCollisionStart;
use bevy::prelude::*;
use rand::Rng;

use crate::assets::game_assets::HEALTH_BAR_WIDTH;
use crate::data::Tower;
use crate::data::{Ailments, StatusEffect, add_status_effect};
use crate::{AppSystems, PausableSystems};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (status_effect_ailments, update_health_bars)
            .in_set(PausableSystems)
            .in_set(AppSystems::Update),
    );
    app.add_observer(collision_event);
    app.add_observer(ailment_damage);
    app.add_observer(tower_collision_damage);
    // Debug picker helpers
    app.add_observer(pick_enemy_to_do_damage_example);
    app.add_observer(pick_enemy_to_add_status_example);
}

#[derive(Component, Default, Clone, Copy, PartialEq, Reflect)]
pub struct EnemyHealth(pub f32);

#[derive(Component, Default, Clone, Copy, PartialEq, Reflect)]
pub struct EnemyChild;

#[derive(Component, Default, Clone, Copy, PartialEq, Reflect)]
pub struct EnemyHealthBar;

#[derive(Event, Debug)]
pub struct Attack {
    tower_entity: Entity,
}

#[derive(Event, Debug)]
pub struct AilmentDamage {
    ailments: Ailments,
}

impl EnemyHealth {
    pub fn new() -> Self {
        Self(1.0)
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

fn collision_event(trigger: Trigger<OnCollisionStart>, mut commands: Commands) {
    let enemy_target = trigger.event().collider;

    commands.trigger_targets(
        Attack {
            tower_entity: trigger.target(),
        },
        enemy_target,
    );
}

pub fn tower_collision_damage(
    trigger: Trigger<Attack>,
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

    let tower_damage = tower.collision_damage();
    let rng = &mut rand::rng();
    let damage = rng.random_range(tower_damage.min..=tower_damage.max);

    parent_health.0 -= damage;
    parent_health.0 = parent_health.0.clamp(0.0, 1.0);
}

fn status_effect_ailments(
    time: Res<Time>,
    mut enemies: Query<(Entity, &mut Ailments)>,
    mut commands: Commands,
) {
    for (enemy_entity, mut ailments) in enemies.iter_mut() {
        ailments.damage_timer.tick(time.delta());
        if ailments.damage_timer.just_finished() {
            info!("Do damage {} to {:?}", ailments.max_damage, enemy_entity);
            commands.trigger_targets(
                AilmentDamage {
                    ailments: ailments.clone(),
                },
                enemy_entity,
            );
        }

        if !ailments.ailment_timer.finished() {
            ailments.ailment_timer.tick(time.delta());
        } else {
            commands.entity(enemy_entity).remove::<StatusEffect>();
            commands.entity(enemy_entity).remove::<Ailments>();
        }
    }
}

pub fn ailment_damage(trigger: Trigger<AilmentDamage>, mut enemies: Query<&mut EnemyHealth>) {
    let Ok(mut parent_health) = enemies.get_mut(trigger.target()) else {
        warn!(target=?trigger.target(), "Enemy target not found");
        return;
    };

    let ailment = &trigger.ailments;
    let rng = &mut rand::rng();
    let damage = rng.random_range(ailment.min_damage..=ailment.max_damage);

    parent_health.0 -= damage;
    parent_health.0 = parent_health.0.clamp(0.0, 1.0);

    info!(
        "Enemy {:?} has {} health",
        trigger.target(),
        parent_health.0
    );
}

// Debug Helpers

fn pick_enemy_to_do_damage_example(
    trigger: Trigger<Pointer<Pressed>>,
    world: bevy::ecs::world::DeferredWorld,
    query: Query<Entity, With<Tower>>,
    mut commands: Commands,
) {
    let tower_entity = match query.iter().next() {
        Some(e) => e,
        None => commands.spawn(Tower::SpikePit).id(),
    };

    let child_of_enemy_target = trigger.target();

    let Some(enemy_target) = world.get::<ChildOf>(child_of_enemy_target) else {
        return;
    };

    commands.trigger_targets(Attack { tower_entity }, enemy_target.0);
}

fn pick_enemy_to_add_status_example(
    trigger: Trigger<Pointer<Pressed>>,
    world: bevy::ecs::world::DeferredWorld,
    mut commands: Commands,
) {
    let child_of_enemy_target = trigger.target();

    let Some(enemy_target) = world.get::<ChildOf>(child_of_enemy_target) else {
        return;
    };

    let Some(status_bundle) = add_status_effect("Acidic") else {
        return;
    };

    commands.entity(enemy_target.0).insert(status_bundle);
}

use crate::data::PlayerState;
use crate::demo::enemy_movement::MovementDirection;
use crate::gameplay::shared_systems::Lifetime;
use crate::{
    AppSystems, PausableSystems,
    assets::{
        StatusSprites,
        game_assets::{self, HEALTH_BAR_WIDTH},
    },
    data::{
        Ailments, StatusEffect, Tower, TowerCollision, get_ailment, get_collision,
        projectiles::DamageType,
    },
};
use avian2d::prelude::{OnCollisionEnd, OnCollisionStart};
use bevy::prelude::*;
use bevy_turborand::{DelegatedRng, GlobalRng};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            active_tower_collision,
            status_effect_ailments,
            update_health_bars,
            animate_status_effects,
            (kill_at_0_health, do_kill_enemies).chain(),
            (try_enemy_damage, do_enemy_damage).chain(),
        )
            .in_set(PausableSystems)
            .in_set(AppSystems::Update),
    );
    app.add_event::<KillEnemy>()
        .add_event::<DoDamageToEnemy>()
        .add_event::<TryDamageToEnemy>();
    app.add_observer(start_collision_damage_event);
    app.add_observer(stop_collision_damage_event);
    app.add_observer(display_status);
    // Debug picker helpers
    //app.add_observer(pick_enemy_to_do_damage_example);
    // app.add_observer(pick_enemy_to_add_status_example);
}

#[derive(Component, Default, Clone, Copy, PartialEq, Reflect)]
pub struct EnemyHealth(pub f32);

#[derive(Component, Default, Clone, Copy, PartialEq, Reflect)]
/// Gives money when the entity is killed
pub struct Bounty(pub i32);

#[derive(Component, Default, Clone, Copy, PartialEq, Reflect)]
pub struct EnemyChild;

#[derive(Component, Default, Clone, Copy, PartialEq, Reflect)]
pub struct EnemyHealthBar;

#[derive(Event, Debug)]
pub struct RecordInitCollisionDamage {
    tower_entity: Entity,
}

#[derive(Event, Debug, Clone, Copy, PartialEq, Reflect)]
pub struct TryDamageToEnemy {
    pub damage_range: (f32, f32),
    pub damage_type: DamageType,
    pub enemy: Entity,
}

#[derive(Event, Debug, Clone, Copy, PartialEq, Reflect)]
pub struct DoDamageToEnemy {
    pub damage: f32,
    pub damage_type: DamageType,
    pub enemy: Entity,
}

#[derive(Event, Debug, Clone, Copy, PartialEq, Reflect)]
pub struct KillEnemy(pub Entity);

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

pub fn kill_at_0_health(
    enemies: Query<(Entity, &EnemyHealth), Without<Lifetime>>,
    mut events: EventWriter<KillEnemy>,
) {
    for (entity, health) in enemies {
        if health.0 <= 0.0 {
            events.write(KillEnemy(entity));
        }
    }
}

pub fn do_kill_enemies(
    mut events: EventReader<KillEnemy>,
    bounty: Query<&Bounty>,
    mut commands: Commands,
    mut player_state: ResMut<PlayerState>,
) {
    for event in events.read() {
        commands.entity(event.0).insert(Lifetime::new(0.1));
        if let Ok(bounty) = bounty.get(event.0) {
            player_state.money += bounty.0;
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
    mut damage_events: EventWriter<TryDamageToEnemy>,
) {
    let enemy_target = trigger.event().collider;

    if let Ok(tower) = towers.get(trigger.target()) {
        let tower_collision = get_collision(&tower);

        if let Some(collision) = tower_collision {
            // Perform a immediate damage hit
            damage_events.write(TryDamageToEnemy {
                damage_range: (collision.min_damage, collision.max_damage),
                damage_type: DamageType::Physical,
                enemy: enemy_target,
            });

            // Add a collision entity that deals damage on a timer while collision is active
            commands.entity(enemy_target).insert(collision);
        }
    };
}

fn stop_collision_damage_event(trigger: Trigger<OnCollisionEnd>, mut commands: Commands) {
    let enemy_target = trigger.event().collider;

    commands.entity(enemy_target).remove::<TowerCollision>();
}

// This is the timer function that deals damage when activeley in collision
fn active_tower_collision(
    time: Res<Time>,
    mut enemies: Query<(Entity, &mut TowerCollision)>,
    mut damage_events: EventWriter<TryDamageToEnemy>,
) {
    for (entity, mut tower_collision) in enemies.iter_mut() {
        tower_collision.iframe.tick(time.delta());
        if tower_collision.iframe.just_finished() {
            damage_events.write(TryDamageToEnemy {
                damage_range: (tower_collision.min_damage, tower_collision.max_damage),
                damage_type: DamageType::Physical,
                enemy: entity,
            });
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
pub fn try_enemy_damage(
    mut attempts: EventReader<TryDamageToEnemy>,
    mut successes: EventWriter<DoDamageToEnemy>,
    mut rng: ResMut<GlobalRng>,
) {
    for event in attempts.read() {
        let damage =
            rng.f32() * (event.damage_range.1 - event.damage_range.0) + event.damage_range.0;
        // TODO: I-Frame logic, which is how damage can fail
        // TODO: Elemental resistances and weaknesses from current status effects
        successes.write(DoDamageToEnemy {
            damage,
            damage_type: event.damage_type,
            enemy: event.enemy,
        });
    }
}

pub fn do_enemy_damage(
    mut events: EventReader<DoDamageToEnemy>,
    mut enemies: Query<&mut EnemyHealth>,
) {
    for event in events.read() {
        if let Ok(mut health) = enemies.get_mut(event.enemy) {
            health.0 -= event.damage;
            health.0 = health.0.clamp(0.0, 1.0);

            debug!("Enemy {:?} has {} health", event.enemy, health.0);
        } else {
            warn!(target=?event.enemy, "Enemy target not found");
            return;
        };
    }
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

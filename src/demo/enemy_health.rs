use crate::assets::game_assets::HEALTH_BAR_WIDTH;
use avian2d::prelude::OnCollisionStart;
use bevy::prelude::*;

use super::enemy_movement::EnemyController;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(hit_player);
    app.add_observer(update_health_bar);
}

#[derive(Component, Default, Clone, Copy, PartialEq, Reflect)]
pub struct EnemyHealth(pub f32);

#[derive(Component, Default, Clone, Copy, PartialEq, Reflect)]
pub struct EnemyHealthBar;

#[derive(Event, Debug)]
pub struct Attack {
    pub damage: f32,
}

impl EnemyHealth {
    pub fn new() -> Self {
        Self(1.0)
    }
}

pub fn update_health_bar(
    trigger: Trigger<Attack>,
    mut bars: Query<(Entity, &ChildOf, &mut Transform), With<EnemyHealthBar>>,
    mut enemies: Query<&mut EnemyHealth>,
) {
    let Ok(mut parent_health) = enemies.get_mut(trigger.target()) else {
        warn!(target=?trigger.target(), "Target not found");
        return;
    };

    for (_, child_of, mut transform) in bars.iter_mut() {
        if child_of.0 == trigger.target() {
            parent_health.0 -= trigger.damage.clamp(0.0, 1.0);
            transform.scale.x = parent_health.0;
            transform.translation.x = -(HEALTH_BAR_WIDTH * (1.0 - parent_health.0)) / 2.0;
        }
    }
}

fn hit_player(
    trigger: Trigger<OnCollisionStart>,
    enemies: Query<Entity, With<EnemyHealth>>,
    mut commands: Commands,
) {
    let target = trigger.event().collider;
    let Ok(_) = enemies.get(target) else {
        return;
    };

    let damage = 0.25;
    commands.trigger_targets(Attack { damage: damage }, target);
}

use bevy::ecs::{
    event::{EventReader, EventWriter},
    observer::Trigger,
    query::With,
    system::Query,
    world::OnAdd,
};

use crate::{
    data::{
        projectiles::DamageType,
        status_effects::{Burned, Chilled, Frozen, Oiled, StatusEffect, StatusEnum, Wet},
    },
    demo::enemy_health::{EnemyHealth, TryDamageToEnemy},
};

use super::common::{RemoveStatus, TryApplyStatus};

pub fn ignite_when_burned(
    trigger: Trigger<OnAdd, StatusEffect<Burned>>,
    oiled_enemies: Query<&StatusEffect<Oiled>, With<EnemyHealth>>,
    mut statuses: EventWriter<TryApplyStatus>,
    mut oiled: EventWriter<RemoveStatus<Oiled>>,
    mut burned: EventWriter<RemoveStatus<Burned>>,
) {
    let e = trigger.target();
    if let Ok(status) = oiled_enemies.get(e) {
        statuses.write(TryApplyStatus {
            status: StatusEnum::Ignited,
            enemy: e,
            strength: status.strength + 1,
        });
        oiled.write(RemoveStatus::new(e, status.strength));
        burned.write(RemoveStatus::new(e, status.strength));
    }
}

pub fn ignite_when_electrocuted(
    mut damage_events: EventReader<TryDamageToEnemy>,
    oiled_enemies: Query<&StatusEffect<Oiled>, With<EnemyHealth>>,
    mut statuses: EventWriter<TryApplyStatus>,
    mut oiled: EventWriter<RemoveStatus<Oiled>>,
) {
    for TryDamageToEnemy {
        damage,
        strength,
        damage_type,
        enemy,
    } in damage_events.read()
    {
        if *damage_type == DamageType::Lightning {
            if let Ok(status) = oiled_enemies.get(*enemy) {
                statuses.write(TryApplyStatus {
                    status: StatusEnum::Ignited,
                    enemy: *enemy,
                    strength: status.strength + strength,
                });
                oiled.write(RemoveStatus::new(*enemy, status.strength));
            }
        }
    }
}

use bevy::ecs::{event::EventWriter, observer::Trigger, query::With, system::Query, world::OnAdd};

use crate::{
    data::status_effects::{Chilled, Frozen, StatusEffect, StatusEnum, Wet},
    demo::enemy_health::EnemyHealth,
};

use super::common::{RemoveStatus, TryApplyStatus};

pub fn freeze_when_wet(
    trigger: Trigger<OnAdd, StatusEffect<Chilled>>,
    wet_enemies: Query<&StatusEffect<Wet>, With<EnemyHealth>>,
    mut frozen_statuses: EventWriter<TryApplyStatus>,
    mut wet_statuses: EventWriter<RemoveStatus<Wet>>,
) {
    let e = trigger.target();
    if let Ok(status) = wet_enemies.get(e) {
        frozen_statuses.write(TryApplyStatus {
            status: StatusEnum::Frozen,
            enemy: e,
            strength: 2,
        });
        wet_statuses.write(RemoveStatus::new(e, status.strength));
    }
}

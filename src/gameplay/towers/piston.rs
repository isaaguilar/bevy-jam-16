use avian2d::prelude::ExternalImpulse;
use bevy::{
    ecs::{
        entity::Entity,
        event::{Event, EventReader},
        query::With,
        system::Query,
    },
    reflect::Reflect,
};

use crate::{
    data::status_effects::damage_multiplier, demo::enemy_health::EnemyHealth,
    level::resource::CellDirection,
};

#[derive(Event, Reflect, Clone, Debug, Copy, PartialEq, Eq)]
pub struct Shove(Entity, CellDirection, usize);

pub fn do_shoves(
    mut events: EventReader<Shove>,
    mut enemies: Query<&mut ExternalImpulse, With<EnemyHealth>>,
) {
    for Shove(e, direction, power) in events.read() {
        if let Ok(mut impulse) = enemies.get_mut(*e) {
            **impulse += direction.into() * damage_multiplier(*power)
        }
    }
}

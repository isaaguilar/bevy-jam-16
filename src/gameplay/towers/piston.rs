use avian2d::prelude::ExternalImpulse;
use bevy::{
    ecs::{
        component::Component,
        entity::Entity,
        event::{Event, EventReader},
        query::With,
        system::{Commands, Query},
    },
    math::Vec2,
    reflect::Reflect,
    transform::components::Transform,
};

use crate::{
    data::status_effects::damage_multiplier,
    demo::enemy_health::EnemyHealth,
    level::{components::PathNode, resource::CellDirection},
};

use super::directional::FireDirection;

#[derive(Event, Reflect, Clone, Debug, Copy, PartialEq)]
pub struct Shove(pub Entity, pub CellDirection, pub f32);

pub fn do_shoves(
    mut events: EventReader<Shove>,
    mut enemies: Query<&mut ExternalImpulse, With<EnemyHealth>>,
) {
    for Shove(e, direction, power) in events.read() {
        if let Ok(mut impulse) = enemies.get_mut(*e) {
            **impulse += Into::<Vec2>::into(*direction) * power
        }
    }
}

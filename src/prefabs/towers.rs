use avian2d::prelude::{Collider, CollisionEventsEnabled, CollisionLayers, Sensor};
use bevy::math::Vec2;
use bevy_composable::{app_impl::ComponentTreeable, tree::ComponentTree, wrappers::name};

use crate::{
    data::Tower,
    gameplay::towers::common::{TowerTriggerNeedsGravity, TowerTriggerRange},
    level::resource::CellDirection,
};

use super::{physics::GamePhysicsLayer as GPL, utils::TowerSprite};

pub fn tower(tower: Tower, direction: CellDirection) -> ComponentTree {
    (tower, TowerSprite(tower, direction)).store() + name(tower.name()) + {
        if tower.has_trigger_zone() {
            if tower.gravity_influences_trigger() {
                ().store() << (trigger_zone(Vec2::new(10., 10.)) + TowerTriggerNeedsGravity.store())
            } else {
                ().store() << trigger_zone(Vec2::new(10., 10.))
            }
        } else {
            ().store()
        }
    }
}

pub fn trigger_zone(size: Vec2) -> ComponentTree {
    (
        CollisionEventsEnabled,
        Collider::rectangle(size.x, size.y),
        Sensor,
        CollisionLayers::new(GPL::Level, [GPL::Enemy]),
        TowerTriggerRange,
    )
        .store()
}

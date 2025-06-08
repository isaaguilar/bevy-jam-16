use avian2d::prelude::{Collider, CollisionEventsEnabled, CollisionLayers, Sensor};
use bevy::math::Vec2;
use bevy_composable::{app_impl::ComponentTreeable, tree::ComponentTree, wrappers::name};

use crate::{
    data::Tower,
    gameplay::towers::{
        common::{TowerTriggerNeedsGravity, TowerTriggerRange},
        directional::FireDirection,
        fan::FanNeedsDirection,
    },
    level::resource::CellDirection,
};

use super::{physics::GamePhysicsLayer as GPL, utils::TowerSprite};

pub fn tower(tower: Tower, direction: CellDirection) -> ComponentTree {
    let tower_specific_components = match tower {
        Tower::Piston => FireDirection(direction).store(),
        Tower::Fan => FanNeedsDirection.store(),
        Tower::SpikePit => todo!(),
        Tower::Portal => todo!(),
        _ => ().store(),
    };
    (tower, direction, TowerSprite(tower, direction)).store()
        + name(tower.name())
        + tower_specific_components
        + {
            if tower.has_trigger_zone() {
                if tower.gravity_influences_trigger() {
                    ().store()
                        << (trigger_zone(Vec2::new(9., 9.5)) + TowerTriggerNeedsGravity.store())
                } else if let Some(custom_trigger_zone) = tower.custom_trigger_zone() {
                    ().store() << (trigger_zone(custom_trigger_zone))
                } else {
                    ().store() << trigger_zone(Vec2::new(9., 9.))
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

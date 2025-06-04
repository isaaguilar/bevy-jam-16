use avian2d::prelude::{
    Collider, CollisionLayers, LayerMask, RayCaster, RayHits, ScalableCollider,
};
use bevy::{
    ecs::{
        component::Component,
        entity::{Entity, EntityHashSet},
        query::With,
        system::{Commands, Query},
    },
    math::{Dir2, Vec2, Vec3, Vec3Swizzles, bounding::RayCast2d},
    reflect::Reflect,
    transform::components::{GlobalTransform, Transform},
};

use crate::prefabs::physics::GamePhysicsLayer;

use super::common::TowerTriggerNeedsGravity;

#[derive(Clone, Copy, Debug, Reflect, Component, PartialEq, Eq)]
pub struct RangeDropper(pub Entity);

pub fn spawn_rangedroppers(
    ranges: Query<(Entity, &GlobalTransform), With<TowerTriggerNeedsGravity>>,
    mut commands: Commands,
) {
    let filter: LayerMask = [GamePhysicsLayer::Level].into();

    for (e, transform) in ranges.iter() {
        let (_, rotation, location) = transform.to_scale_rotation_translation();
        let raycaster = RayCaster::new(Vec2::ZERO, Dir2::SOUTH).with_query_filter(
            avian2d::prelude::SpatialQueryFilter {
                mask: [GamePhysicsLayer::Level].into(),
                excluded_entities: EntityHashSet::new(),
            },
        );
        commands.spawn((
            Transform::from_translation(location),
            RangeDropper(e),
            raycaster,
        ));
        commands.entity(e).remove::<TowerTriggerNeedsGravity>();
    }
}

pub fn drop_ranges(
    droppers: Query<(&RayHits, &RangeDropper)>,
    mut ranges: Query<(&mut Collider, &GlobalTransform, &mut Transform)>,
    mut commands: Commands,
) {
    for (hits, RangeDropper(target_entity)) in droppers.iter() {
        if let Ok((mut collider, center_pos, mut pos)) = ranges.get_mut(*target_entity) {
            let mut ray_iter = hits.iter_sorted();
            ray_iter.next();
            let floor = ray_iter.next();
            if let Some(hit_data) = floor {
                let distance = hit_data.distance;
                let collider_height = distance + 5.;
                pos.translation += Vec3::new(0., collider_height / 2. - 5., 0.);
                collider.scale_by(Vec2::new(1., collider_height / 10.), 0);
                commands.entity(*target_entity).despawn();
            }
        }
    }
}

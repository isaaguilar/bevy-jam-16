use avian2d::{
    math::*,
    prelude::{NarrowPhaseSet, *},
};
use bevy::{math::NormedVectorSpace, prelude::*};

use crate::{
    AppSystems, PausableSystems,
    gameplay::animation::AnimationFrameQueue,
    level::{components::PathNode, resource::CellDirection},
};

use super::enemy_health::EnemyHealth;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (flip_sprite, follow_path, movement)
            .chain()
            .in_set(AppSystems::Update)
            .in_set(PausableSystems),
    );
}

/// An component sent for a movement input action.
#[derive(Component, Default, Clone, Copy, PartialEq, Reflect)]
pub struct MovementDirection(pub Vec2);

/// The acceleration used for character movement.
#[derive(Component, Default, Clone, Copy, PartialEq, Reflect)]
pub struct MovementAcceleration(pub Scalar);

/// A bundle that contains the components needed for a basic
/// kinematic character controller.
#[derive(Bundle, Clone)]
pub struct EnemyControllerBundle {
    movement_direction: MovementDirection,
    body: RigidBody,
    collider: Collider,
    ground_caster: ShapeCaster,
    movement: MovementBundle,
}

/// A bundle that contains components for character movement.
#[derive(Bundle, Clone, Reflect)]
pub struct MovementBundle {
    acceleration: MovementAcceleration,
}

impl MovementBundle {
    pub const fn new(acceleration: Scalar) -> Self {
        Self {
            acceleration: MovementAcceleration(acceleration),
        }
    }
}

impl Default for MovementBundle {
    fn default() -> Self {
        Self::new(30.0)
    }
}

impl EnemyControllerBundle {
    pub fn new(collider: Collider, gravity: Vector) -> Self {
        // Create shape caster as a slightly smaller version of collider
        let mut caster_shape = collider.clone();
        caster_shape.set_scale(Vector::ONE * 0.15, 10);

        Self {
            movement_direction: MovementDirection::default(),
            body: RigidBody::Kinematic,
            collider,
            ground_caster: ShapeCaster::new(caster_shape, Vector::ZERO, 0.0, Dir2::NEG_Y)
                .with_max_distance(4.0),
            movement: MovementBundle::default(),
        }
    }

    pub fn with_movement(mut self, acceleration: Scalar) -> Self {
        self.movement = MovementBundle::new(acceleration);
        self
    }
}

/// Sends [`MovementAction`] events based on enemy's waypoint direction
fn follow_path(
    mut enemies: Query<(&Transform, &mut MovementDirection, &mut GravityScale), With<EnemyHealth>>,
    nodes: Query<(&Transform, &PathNode)>,
) {
    // path instructions to walk around in a circle
    for (enemy_transform, mut movement_direction, mut gravity_scale) in enemies.iter_mut() {
        let pos = enemy_transform.translation.xy();

        let mut nodes_sorted_by_distance = nodes
            .iter()
            .map(|w| (pos.distance(w.0.translation.xy()), (w.0, w.1.0)))
            .collect::<Vec<_>>();
        nodes_sorted_by_distance.sort_by(|w, other| w.0.total_cmp(&other.0));
        let (closest, second_closest) =
            (nodes_sorted_by_distance[0].1, nodes_sorted_by_distance[1].1);

        // I plan on adding more complicated movement logic later to help them go around corners
        // but this will work for now
        gravity_scale.0 = match closest.1 {
            CellDirection::Up => 0.3,
            _ => 1.,
        };
        movement_direction.0 = match closest.1 {
            CellDirection::Up | CellDirection::Down => closest.1.vec(),
            CellDirection::Left | CellDirection::Right => {
                ((closest.1.vec() + second_closest.1.vec()) / 2.).normalize_or_zero()
            }
        };
    }
}

/// Responds to [`MovementAction`] events and moves character controllers accordingly.
fn movement(
    time: Res<Time>,
    mut controllers: Query<(
        &MovementDirection,
        &MovementAcceleration,
        &mut LinearVelocity,
    )>,
) {
    // Precision is adjusted so that the example works with
    // both the `f32` and `f64` features. Otherwise you don't need this.
    let delta_time = time.delta_secs_f64().adjust_precision();

    for (movement_direction, movement_acceleration, mut linear_velocity) in &mut controllers {
        let x_direction = movement_direction.0.x;
        let y_direction = movement_direction.0.y;

        if x_direction != 0.0 {
            linear_velocity.x += x_direction * movement_acceleration.0 * delta_time;
        }

        if y_direction != 0.0 {
            linear_velocity.y += y_direction * movement_acceleration.0 * delta_time;
        }
    }
}

fn flip_sprite(
    controllers: Query<&MovementDirection>,
    mut character_sprites: Query<(&mut Sprite, &ChildOf), With<AnimationFrameQueue>>,
) {
    for (mut sprite, parent_entity) in character_sprites.iter_mut() {
        if let Ok(movement_direction) = controllers.get(parent_entity.0) {
            sprite.flip_x = movement_direction.0.x <= 0.0;
        }
    }
}

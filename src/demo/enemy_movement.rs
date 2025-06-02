use crate::{
    AppSystems, PausableSystems,
    level::{components::LEVEL_SCALING, resource::Level},
};
use avian2d::{
    math::*,
    prelude::{NarrowPhaseSet, *},
};
use bevy::math::NormedVectorSpace;
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (follow_path, movement, apply_movement_damping)
            .chain()
            .in_set(AppSystems::Update)
            .in_set(PausableSystems),
    )
    .add_systems(
        // Run collision handling after collision detection.
        //
        // NOTE: The collision implementation here is very basic and a bit buggy.
        //       A collide-and-slide algorithm would likely work better.
        PhysicsSchedule,
        kinematic_controller_collisions.in_set(NarrowPhaseSet::Last),
    )
    .add_systems(
        PreUpdate,
        sleep_physics
            .in_set(AppSystems::Update)
            .in_set(PausableSystems),
    );
}

fn sleep_physics(mut commands: Commands, enemies: Query<Entity, With<Collider>>) {
    for entity in enemies {
        commands.entity(entity).insert(Sleeping);
    }
}

/// An event sent for a movement input action.
#[derive(Component, Default)]
struct MovementDirection {
    x: f32,
    y: f32,
}

/// A marker component indicating that an entity is using a character controller.
#[derive(Component)]
pub struct EnemyController;

/// The acceleration used for character movement.
#[derive(Component)]
pub struct MovementAcceleration(Scalar);

/// The damping factor used for slowing down movement.
#[derive(Component)]
pub struct MovementDampingFactor(Scalar);

/// The gravitational acceleration used for a character controller.
#[derive(Component)]
#[allow(dead_code)]
pub struct ControllerGravity(Vector);

/// The maximum angle a slope can have for a character controller
/// to be able to climb and jump. If the slope is steeper than this angle,
/// the character will slide down.
#[derive(Component)]
pub struct MaxSlopeAngle(Scalar);

/// A bundle that contains the components needed for a basic
/// kinematic character controller.
#[derive(Bundle)]
pub struct EnemyControllerBundle {
    character_controller: EnemyController,
    movement_direction: MovementDirection,
    body: RigidBody,
    collider: Collider,
    ground_caster: ShapeCaster,
    gravity: ControllerGravity,
    movement: MovementBundle,
    waypoint: Waypoint,
}

/// A bundle that contains components for character movement.
#[derive(Bundle)]
pub struct MovementBundle {
    acceleration: MovementAcceleration,
    damping: MovementDampingFactor,
}

#[derive(Component, Default)]
pub struct Waypoint {
    index: usize,
}

impl MovementBundle {
    pub const fn new(acceleration: Scalar, damping: Scalar) -> Self {
        Self {
            acceleration: MovementAcceleration(acceleration),
            damping: MovementDampingFactor(damping),
        }
    }
}

impl Default for MovementBundle {
    fn default() -> Self {
        Self::new(30.0, 0.9)
    }
}

impl EnemyControllerBundle {
    pub fn new(collider: Collider, gravity: Vector) -> Self {
        // Create shape caster as a slightly smaller version of collider
        let mut caster_shape = collider.clone();
        caster_shape.set_scale(Vector::ONE * 0.15, 10);

        Self {
            character_controller: EnemyController,
            movement_direction: MovementDirection::default(),
            body: RigidBody::Kinematic,
            collider,
            ground_caster: ShapeCaster::new(caster_shape, Vector::ZERO, 0.0, Dir2::NEG_Y)
                .with_max_distance(4.0),
            gravity: ControllerGravity(gravity),
            movement: MovementBundle::default(),
            waypoint: Waypoint::default(),
        }
    }

    pub fn with_movement(mut self, acceleration: Scalar, damping: Scalar) -> Self {
        self.movement = MovementBundle::new(acceleration, damping);
        self
    }
}

/// Sends [`MovementAction`] events based on enemy's waypoint direction
fn follow_path(
    level: Res<Level>,
    mut enemies: Query<(&Transform, &mut Waypoint, &mut MovementDirection), With<EnemyController>>,
) {
    // path instructions to walk around in a circle
    for (enemy_transform, mut enemy_waypoint, mut movement_direction) in enemies.iter_mut() {
        let x = enemy_transform.translation.x;
        let y = enemy_transform.translation.y;

        let idx = enemy_waypoint.index;
        let heading_towards = level.path[idx].0 * LEVEL_SCALING;

        let arrived_x = if x.distance(heading_towards.x) > 5.0 {
            let direction = if heading_towards.x > x { 1. } else { -1. };
            movement_direction.x = direction;
            false
        } else {
            movement_direction.x = 0.0;
            true
        };

        let arrived_y = if y.distance(heading_towards.y) > 5.0 {
            let direction = if heading_towards.y > y { 1. } else { -1. };
            movement_direction.y = direction;
            false
        } else {
            movement_direction.y = 0.0;
            true
        };

        if arrived_x && arrived_y && enemy_waypoint.index < level.path.len() - 1 {
            enemy_waypoint.index += 1;
        }
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
        let x_direction = movement_direction.x;
        let y_direction = movement_direction.y;
        if x_direction != 0.0 {
            linear_velocity.x += x_direction * movement_acceleration.0 * delta_time;
        }

        if y_direction != 0.0 {
            linear_velocity.y += y_direction * movement_acceleration.0 * delta_time;
        }
    }
}

/// Slows down movement in the X direction.
fn apply_movement_damping(mut query: Query<(&MovementDampingFactor, &mut LinearVelocity)>) {
    for (damping_factor, mut linear_velocity) in &mut query {
        // We could use `LinearDamping`, but we don't want to dampen movement along the Y axis
        linear_velocity.x *= damping_factor.0;
        linear_velocity.y *= damping_factor.0;
    }
}

/// Kinematic bodies do not get pushed by collisions by default,
/// so it needs to be done manually.
///
/// This system handles collision response for kinematic character controllers
/// by pushing them along their contact normals by the current penetration depth,
/// and applying velocity corrections in order to snap to slopes, slide along walls,
/// and predict collisions using speculative contacts.
#[allow(clippy::type_complexity)]
fn kinematic_controller_collisions(
    collisions: Collisions,
    bodies: Query<&RigidBody>,
    collider_rbs: Query<&ColliderOf, Without<Sensor>>,
    mut character_controllers: Query<
        (&mut Position, &mut LinearVelocity, Option<&MaxSlopeAngle>),
        (With<RigidBody>, With<EnemyController>),
    >,
    time: Res<Time>,
) {
    // Iterate through collisions and move the kinematic body to resolve penetration
    for contacts in collisions.iter() {
        // Get the rigid body entities of the colliders (colliders could be children)
        let Ok([&ColliderOf { body: rb1 }, &ColliderOf { body: rb2 }]) =
            collider_rbs.get_many([contacts.collider1, contacts.collider2])
        else {
            continue;
        };

        // Get the body of the character controller and whether it is the first
        // or second entity in the collision.
        let is_first: bool;

        let character_rb: RigidBody;
        let is_other_dynamic: bool;

        let (mut position, mut linear_velocity, max_slope_angle) =
            if let Ok(character) = character_controllers.get_mut(rb1) {
                is_first = true;
                character_rb = *bodies.get(rb1).unwrap();
                is_other_dynamic = bodies.get(rb2).is_ok_and(|rb| rb.is_dynamic());
                character
            } else if let Ok(character) = character_controllers.get_mut(rb2) {
                is_first = false;
                character_rb = *bodies.get(rb2).unwrap();
                is_other_dynamic = bodies.get(rb1).is_ok_and(|rb| rb.is_dynamic());
                character
            } else {
                continue;
            };

        // This system only handles collision response for kinematic character controllers.
        if !character_rb.is_kinematic() {
            continue;
        }

        // Iterate through contact manifolds and their contacts.
        // Each contact in a single manifold shares the same contact normal.
        for manifold in contacts.manifolds.iter() {
            let normal = if is_first {
                -manifold.normal
            } else {
                manifold.normal
            };

            let mut deepest_penetration: Scalar = Scalar::MIN;

            // Solve each penetrating contact in the manifold.
            for contact in manifold.points.iter() {
                if contact.penetration > 0.0 {
                    position.0 += normal * contact.penetration;
                }
                deepest_penetration = deepest_penetration.max(contact.penetration);
            }

            // For now, this system only handles velocity corrections for collisions against static geometry.
            if is_other_dynamic {
                continue;
            }

            // Determine if the slope is climbable or if it's too steep to walk on.
            let slope_angle = normal.angle_to(Vector::Y);
            let climbable = max_slope_angle.is_some_and(|angle| slope_angle.abs() <= angle.0);

            if deepest_penetration > 0.0 {
                // If the slope is climbable, snap the velocity so that the character
                // up and down the surface smoothly.
                if climbable {
                    // Points either left or right depending on which side the normal is leaning on.
                    // (This could be simplified for 2D, but this approach is dimension-agnostic)
                    let normal_direction_x =
                        normal.reject_from_normalized(Vector::Y).normalize_or_zero();

                    // The movement speed along the direction above.
                    let linear_velocity_x = linear_velocity.dot(normal_direction_x);

                    // Snap the Y speed based on the speed at which the character is moving
                    // up or down the slope, and how steep the slope is.
                    //
                    // A 2D visualization of the slope, the contact normal, and the velocity components:
                    //
                    //             ╱
                    //     normal ╱
                    // *         ╱
                    // │   *    ╱   velocity_x
                    // │       * - - - - - -
                    // │           *       | velocity_y
                    // │               *   |
                    // *───────────────────*

                    let max_y_speed = -linear_velocity_x * slope_angle.tan();
                    linear_velocity.y = linear_velocity.y.max(max_y_speed);
                } else {
                    // The character is intersecting an unclimbable object, like a wall.
                    // We want the character to slide along the surface, similarly to
                    // a collide-and-slide algorithm.

                    // Don't apply an impulse if the character is moving away from the surface.
                    if linear_velocity.dot(normal) > 0.0 {
                        continue;
                    }

                    // Slide along the surface, rejecting the velocity along the contact normal.
                    let impulse = linear_velocity.reject_from_normalized(normal);
                    linear_velocity.0 = impulse;
                }
            } else {
                // The character is not yet intersecting the other object,
                // but the narrow phase detected a speculative collision.
                //
                // We need to push back the part of the velocity
                // that would cause penetration within the next frame.

                let normal_speed = linear_velocity.dot(normal);

                // Don't apply an impulse if the character is moving away from the surface.
                if normal_speed > 0.0 {
                    continue;
                }

                // Compute the impulse to apply.
                let impulse_magnitude =
                    normal_speed - (deepest_penetration / time.delta_secs_f64().adjust_precision());
                let mut impulse = impulse_magnitude * normal;

                // Apply the impulse differently depending on the slope angle.
                if climbable {
                    // Avoid sliding down slopes.
                    linear_velocity.y -= impulse.y.min(0.0);
                } else {
                    // Avoid climbing up walls.
                    impulse.y = impulse.y.max(0.0);
                    linear_velocity.0 -= impulse;
                }
            }
        }
    }
}

use avian2d::{math::*, prelude::*};
use bevy::{math::NormedVectorSpace, prelude::*};

use crate::{
    AppSystems, PausableSystems,
    data::stats::{MoveSpeed, Stat},
    gameplay::{animation::AnimationFrameQueue, stats::StatSet},
    level::{components::PathNode, resource::CellDirection},
    screens::Screen,
};

use super::enemy_health::EnemyHealth;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (flip_sprite, follow_path, enemy_movement)
            .chain()
            .run_if(in_state(Screen::Gameplay))
            .in_set(AppSystems::Update)
            .in_set(PausableSystems)
            .in_set(StatSet::Use),
    );
}

/// An component sent for a movement input action.
#[derive(Component, Default, Clone, Copy, PartialEq, Reflect)]
pub struct MovementDirection(pub Vec2);

/// A bundle that contains the components needed for a basic
/// kinematic character controller.
#[derive(Bundle, Clone)]
pub struct EnemyControllerBundle {
    movement_direction: MovementDirection,
    body: RigidBody,
    collider: Collider,
    ground_caster: ShapeCaster,
    movement: Stat<MoveSpeed>,
}

/// Sends [`MovementAction`] events based on enemy's waypoint direction
fn follow_path(
    mut enemies: Query<
        (
            &Transform,
            &mut MovementDirection,
            &mut GravityScale,
            &Stat<MoveSpeed>,
        ),
        With<EnemyHealth>,
    >,
    nodes: Query<(&Transform, &PathNode)>,
) {
    // path instructions to walk around in a circle
    for (enemy_transform, mut movement_direction, mut gravity_scale, speed) in enemies.iter_mut() {
        let pos = enemy_transform.translation.xy();

        let mut nodes_sorted_by_distance = nodes
            .iter()
            .map(|w| {
                (
                    pos.distance(w.0.translation.xy()),
                    (w.0, w.1.direction, w.1.prev_direction),
                )
            })
            .collect::<Vec<_>>();
        nodes_sorted_by_distance.sort_by(|w, other| w.0.total_cmp(&other.0));
        let (_, closest, prev) = nodes_sorted_by_distance[0].1;

        gravity_scale.0 = if speed.current_value() > 0.1
            && (closest == CellDirection::Up || prev == CellDirection::Up)
        {
            0.1
        } else {
            1.0
        };

        let average = ((closest.vec() + prev.vec()) / 2.).normalize_or_zero();
        movement_direction.0 = match closest {
            CellDirection::Up => closest.vec(),
            _ => match prev {
                CellDirection::Up => ((closest.vec() + prev.vec() * 2.) / 3.).normalize_or_zero(),
                CellDirection::Down => closest.vec(),
                CellDirection::Left | CellDirection::Right => average,
            },
        };
    }
}

/// Responds to [`MovementAction`] events and moves character controllers accordingly.
fn enemy_movement(
    time: Res<Time>,
    mut controllers: Query<(&MovementDirection, &Stat<MoveSpeed>, &mut LinearVelocity)>,
) {
    // Precision is adjusted so that the example works with
    // both the `f32` and `f64` features. Otherwise you don't need this.
    let delta_time = time.delta_secs_f64().adjust_precision();

    for (movement_direction, movement_acceleration, mut linear_velocity) in &mut controllers {
        let x_direction = movement_direction.0.x;
        let y_direction = movement_direction.0.y;

        if x_direction != 0.0 {
            linear_velocity.x += x_direction * movement_acceleration.current_value() * delta_time;
        }

        if y_direction != 0.0 {
            linear_velocity.y += y_direction * movement_acceleration.current_value() * delta_time;
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

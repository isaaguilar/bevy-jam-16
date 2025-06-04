use bevy::{
    app::{App, FixedUpdate, Update},
    ecs::{
        schedule::IntoScheduleConfigs,
        system::{Commands, Res},
    },
    math::{Quat, Vec3},
    state::{condition::in_state, state::OnEnter},
    transform::components::Transform,
};
use bevy_composable::app_impl::{ComplexSpawnable, ComponentTreeable};
use std::f32;

use crate::{
    PausableSystems, assets::TowerSprites, data::Tower, level::components::pos,
    prefabs::towers::tower, screens::Screen,
};
use common::*;

pub mod common;
pub mod tesla;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), test_system);

    app.add_systems(
        Update,
        (towers_fire, remove_cooldown, tick_cooldown)
            .in_set(PausableSystems)
            .run_if(in_state(Screen::Gameplay)),
    );

    app.add_systems(
        FixedUpdate,
        (add_tower_targets_from_zone, remove_tower_targets)
            .in_set(PausableSystems)
            .run_if(in_state(Screen::Gameplay)),
    );
}

pub fn test_system(mut commands: Commands, sprites: Res<TowerSprites>) {
    commands.compose(tower(Tower::Tesla) + pos(30., 30.));
    commands.compose(
        tower(Tower::Tesla)
            + Transform::from_xyz(0., 20., 0.)
                .with_rotation(Quat::from_axis_angle(Vec3::Z, -f32::consts::FRAC_PI_2))
                .store(),
    );
}

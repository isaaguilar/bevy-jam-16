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
use gravity_bullshit::{RangeDropper, drop_ranges, spawn_rangedroppers};
use std::f32;

use crate::{
    PausableSystems,
    assets::TowerSprites,
    data::Tower,
    level::{components::pos, resource::CellDirection},
    prefabs::towers::tower,
    screens::Screen,
};
use common::*;

pub mod common;
pub mod gravity_bullshit;
pub mod tesla;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<TowerTriggerRange>()
        .register_type::<TowerTriggerNeedsGravity>()
        .register_type::<Cooldown>()
        .register_type::<TowerFired>()
        .register_type::<TowerHasTargets>()
        .register_type::<RangeDropper>();

    app.add_event::<TowerFired>();

    app.add_systems(
        Update,
        (towers_fire, remove_cooldown, tick_cooldown)
            .in_set(PausableSystems)
            .run_if(in_state(Screen::Gameplay)),
    );

    app.add_systems(
        FixedUpdate,
        (
            add_tower_targets_from_zone,
            remove_tower_targets,
            spawn_rangedroppers,
            drop_ranges,
        )
            .in_set(PausableSystems)
            .run_if(in_state(Screen::Gameplay)),
    );
}

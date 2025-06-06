use attacks::{
    AttackEnemiesInContact, DropLiquid, attack_contact_enemies, dispatch_attack_effects,
    drop_liquids, puddle_damage, splat_droplets, stop_dropping_puddles,
};
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
    data::{
        Tower,
        projectiles::{Droplet, Puddle},
    },
    level::{components::pos, resource::CellDirection},
    prefabs::{towers::tower, wizardry::add_observer_to_component},
    screens::Screen,
};
use common::*;

pub mod attacks;
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

    app.add_event::<DropLiquid>()
        .add_event::<TowerFired>()
        .add_event::<AttackEnemiesInContact>();

    app.add_observer(add_observer_to_component::<Puddle, _, _, _, _>(
        stop_dropping_puddles,
    ));
    app.add_observer(add_observer_to_component::<Puddle, _, _, _, _>(
        puddle_damage,
    ));
    app.add_observer(add_observer_to_component::<Droplet, _, _, _, _>(
        splat_droplets,
    ));

    app.add_systems(
        Update,
        (
            (tick_cooldown, remove_cooldown).chain(),
            (
                towers_fire,
                dispatch_attack_effects,
                (attack_contact_enemies, drop_liquids),
            )
                .chain(),
        )
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

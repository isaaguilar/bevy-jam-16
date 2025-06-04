use bevy::{
    app::{App, FixedUpdate, Update},
    ecs::{
        schedule::IntoScheduleConfigs,
        system::{Commands, Res},
    },
    state::{condition::in_state, state::OnEnter},
    transform::components::Transform,
};
use bevy_composable::app_impl::ComplexSpawnable;

use crate::{
    PausableSystems, assets::TowerSprites, data::Tower, level::components::pos,
    prefabs::towers::tower, screens::Screen,
};
use common::{
    add_tower_targets_from_zone, remove_cooldown, remove_tower_targets, tick_cooldown, towers_fire,
};

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
}

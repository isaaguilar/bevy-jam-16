use bevy::{
    app::{App, PreUpdate, Update},
    ecs::{schedule::SystemSet, system::Query},
    prelude::IntoScheduleConfigs,
    reflect::Reflect,
    state::condition::in_state,
};

use crate::{
    data::stats::{DamageMultiplier, MoveSpeed, Stat, StatTrait},
    screens::Screen,
};

#[derive(SystemSet, Debug, Reflect, PartialEq, Eq, Hash, Clone, Copy)]
pub enum StatSet {
    Modify,
    Recalc,
    Use,
}

pub(super) fn plugin(app: &mut App) {
    app.configure_sets(
        Update,
        (StatSet::Modify, StatSet::Recalc, StatSet::Use).chain(),
    );

    implement_stat::<MoveSpeed>(app);
    implement_stat::<DamageMultiplier>(app);
}

pub fn recalculate_stats<T: StatTrait>(mut stats: Query<&mut Stat<T>>) {
    for mut stat in stats.iter_mut() {
        stat.recalculate()
    }
}

pub fn reset_stats<T: StatTrait>(mut stats: Query<&mut Stat<T>>) {
    for mut stat in stats.iter_mut() {
        stat.reset()
    }
}

pub fn implement_stat<T: StatTrait>(app: &mut App) {
    app.register_type::<Stat<T>>().register_type::<T>();

    app.add_systems(PreUpdate, reset_stats::<T>).add_systems(
        Update,
        (recalculate_stats::<T>)
            .in_set(StatSet::Recalc)
            .run_if(in_state(Screen::Gameplay)),
    );
}

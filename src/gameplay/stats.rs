use avian2d::prelude::Friction;
use bevy::{
    app::{App, PreUpdate, Update},
    ecs::{query::Changed, schedule::SystemSet, system::Query},
    prelude::IntoScheduleConfigs,
    reflect::Reflect,
    state::condition::in_state,
};

use crate::{
    PausableSystems,
    data::{
        projectiles::DamageType,
        stats::{DamageMultiplier, MoveSpeed, Stat, StatFriction, StatTrait},
    },
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
    implement_stat::<StatFriction>(app);
    implement_stat::<DamageMultiplier<{ DamageType::Physical }>>(app);
    implement_stat::<DamageMultiplier<{ DamageType::Cold }>>(app);
    implement_stat::<DamageMultiplier<{ DamageType::Burning }>>(app);
    implement_stat::<DamageMultiplier<{ DamageType::Chemical }>>(app);
    implement_stat::<DamageMultiplier<{ DamageType::Lightning }>>(app);

    app.add_systems(
        Update,
        update_friction_from_stats
            .in_set(PausableSystems)
            .in_set(StatSet::Use)
            .run_if(in_state(Screen::Gameplay)),
    );
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

pub fn update_friction_from_stats(
    mut query: Query<(&mut Friction, &Stat<StatFriction>), Changed<Stat<StatFriction>>>,
) {
    for (mut friction, stat) in query.iter_mut() {
        friction.dynamic_coefficient = stat.current_value();
        friction.static_coefficient = stat.current_value();
    }
}

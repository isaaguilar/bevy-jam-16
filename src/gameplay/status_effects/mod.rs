use bevy::{
    app::{App, FixedUpdate, Update},
    ecs::schedule::IntoScheduleConfigs,
    reflect::Reflect,
    state::condition::in_state,
    time::common_conditions::on_timer,
};
use common::{
    ApplyStatus, RemoveStatus, TryApplyStatus, apply_status_effects, dispatch_typed_events,
    do_remove_status, periodic_damage, status_debuff_multiplier, tick_statuses, timeout_statuses,
};
use display::{add_status_animation, animate_status_effect, remove_status_animation_on_timeout};
use fire::ignite_when_burned;
use ice::freeze_when_wet;
use std::time::Duration;
use tesla::{damage_after_electrocute, electrocute_on_damage};

use crate::{
    PausableSystems,
    data::{
        projectiles::DamageType,
        stats::{DamageMultiplier, DamageMultiplierAll, MoveSpeed, StatFriction},
        status_effects::{
            Acidified, Burned, Chilled, Electrocuted, Frozen, Ignited, Oiled, StatusEffect,
            StatusEffectTrait, Wet,
        },
    },
    screens::Screen,
};

use super::stats::StatSet;

pub mod common;
pub mod display;
pub mod fire;
pub mod ice;
pub mod tesla;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<TryApplyStatus>()
        .add_event::<TryApplyStatus>();

    app.add_systems(
        FixedUpdate,
        (
            periodic_damage::<Ignited>(12),
            periodic_damage::<Burned>(4),
            periodic_damage::<Acidified>(4),
            periodic_damage::<Chilled>(3),
        )
            .in_set(PausableSystems)
            .run_if(in_state(Screen::Gameplay)),
    );

    app.add_observer(freeze_when_wet);
    app.add_observer(ignite_when_burned);
    app.add_systems(
        Update,
        (damage_after_electrocute, electrocute_on_damage)
            .in_set(PausableSystems)
            .run_if(in_state(Screen::Gameplay)),
    );

    app.add_systems(
        Update,
        (
            status_debuff_multiplier::<Frozen, MoveSpeed>(0.),
            status_debuff_multiplier::<Frozen, StatFriction>(0.),
            status_debuff_multiplier::<Frozen, DamageMultiplier<{ DamageType::Physical }>>(4.),
            status_debuff_multiplier::<Chilled, MoveSpeed>(0.5),
            status_debuff_multiplier::<Wet, MoveSpeed>(0.9),
            status_debuff_multiplier::<Wet, DamageMultiplier<{ DamageType::Lightning }>>(1.5),
            status_debuff_multiplier::<Oiled, StatFriction>(0.),
            status_debuff_multiplier::<Burned, DamageMultiplierAll>(1.10),
            status_debuff_multiplier::<Burned, DamageMultiplier<{ DamageType::Cold }>>(1.35),
            status_debuff_multiplier::<Acidified, DamageMultiplierAll>(1.05),
            status_debuff_multiplier::<Acidified, DamageMultiplier<{ DamageType::Burning }>>(1.10),
            status_debuff_multiplier::<Acidified, DamageMultiplier<{ DamageType::Chemical }>>(1.10),
        )
            .in_set(StatSet::Modify)
            .in_set(PausableSystems)
            .run_if(in_state(Screen::Gameplay)),
    );

    implement_status_effect::<Wet>(app);
    implement_status_effect::<Ignited>(app);
    implement_status_effect::<Burned>(app);
    implement_status_effect::<Chilled>(app);
    implement_status_effect::<Frozen>(app);
    implement_status_effect::<Electrocuted>(app);
    implement_status_effect::<Acidified>(app);
    implement_status_effect::<Oiled>(app);
}

pub fn implement_status_effect<T: StatusEffectTrait>(app: &mut App) {
    app.register_type::<StatusEffect<T>>()
        .register_type::<T>()
        .add_event::<ApplyStatus<T>>()
        .add_event::<RemoveStatus<T>>()
        .add_systems(
            Update,
            (
                dispatch_typed_events::<T>,
                apply_status_effects::<T>,
                tick_statuses::<T>,
                timeout_statuses::<T>,
                do_remove_status::<T>,
                remove_status_animation_on_timeout::<T>,
            )
                .in_set(PausableSystems)
                .run_if(in_state(Screen::Gameplay)),
        )
        .add_systems(
            FixedUpdate,
            (
                (add_status_animation::<T>),
                (animate_status_effect::<T>).run_if(on_timer(Duration::from_secs_f32(0.25))),
            )
                .in_set(PausableSystems)
                .run_if(in_state(Screen::Gameplay)),
        );
}

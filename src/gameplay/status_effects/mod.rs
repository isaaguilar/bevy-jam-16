use bevy::{
    app::{App, FixedUpdate, Update},
    ecs::schedule::IntoScheduleConfigs,
    state::condition::in_state,
    time::common_conditions::on_timer,
};
use common::{
    ApplyStatus, StatusTimeout, TryApplyStatus, add_status_animation, animate_status_effect,
    apply_status_effects, dispatch_typed_events, periodic_damage, tick_statuses, timeout_statuses,
};
use std::time::Duration;

use crate::{
    PausableSystems,
    data::status_effects::{
        Acidified, Burned, Chilled, Electrocuted, Frozen, Ignited, Oiled, StatusEffect,
        StatusEffectTrait, Wet,
    },
    screens::Screen,
};

pub mod common;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<TryApplyStatus>()
        .add_event::<TryApplyStatus>();

    app.add_systems(
        FixedUpdate,
        (
            periodic_damage::<Ignited>(0.2),
            periodic_damage::<Burned>(0.1),
            periodic_damage::<Acidified>(0.1),
        ),
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
        .add_event::<ApplyStatus<T>>()
        .add_event::<StatusTimeout<T>>()
        .add_systems(
            Update,
            (
                dispatch_typed_events::<T>,
                apply_status_effects::<T>,
                tick_statuses::<T>,
                timeout_statuses::<T>,
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

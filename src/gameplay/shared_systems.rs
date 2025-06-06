use crate::prelude::*;
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (tick_lifetimes, timeout_lifetimes)
            .chain()
            .in_set(PausableSystems)
            .run_if(in_state(Screen::Gameplay)),
    );
}

/// Despawn after a certain time.
#[derive(Component, Clone, Debug, Reflect, PartialEq, Eq)]
pub struct Lifetime(pub Timer);

impl Lifetime {
    pub fn new(duration_sec: f32) -> Self {
        Lifetime(Timer::from_seconds(duration_sec, TimerMode::Once))
    }
}

pub fn tick_lifetimes(mut lifetimes: Query<&mut Lifetime>, time: Res<Time>) {
    for mut lifetime in lifetimes.iter_mut() {
        lifetime.0.tick(time.delta());
    }
}

pub fn timeout_lifetimes(mut commands: Commands, lifetimes: Query<(Entity, &Lifetime)>) {
    for (e, lifetime) in lifetimes.iter() {
        if lifetime.0.finished() {
            commands.entity(e).despawn();
        }
    }
}

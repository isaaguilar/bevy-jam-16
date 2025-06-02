use std::time::Duration;

use bevy::{app::App, ecs::component::Component, time::Timer};
use bevy_composable::tree::ComponentTree;

pub struct WaveManager {
    current_wave: Option<Wave>,
    upcoming_waves: Vec<Wave>,
    wave_timer: Timer,
}

#[derive(Copy, Component, Debug)]
pub struct Wave(pub Vec<(Group, Duration)>);

#[derive(Copy, Component, Debug)]
pub struct Group(pub Vec<ComponentTree>);

pub(super) fn plugin(app: &mut App) {}

impl Default for WaveManager {
    fn default() -> Self {
        Self {
            current_wave: Default::default(),
            upcoming_waves: Default::default(),
            wave_timer: Timer::new(Duration::from_secs(1), bevy::time::TimerMode::Once).paused(),
        }
    }
}

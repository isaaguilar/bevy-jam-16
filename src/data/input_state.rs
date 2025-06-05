use crate::data::towers::Tower;
use bevy::{prelude::States, reflect::Reflect};

#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default, Reflect)]
pub enum PointerInteractionState {
    #[default]
    Selecting,
    Placing(Tower),
}

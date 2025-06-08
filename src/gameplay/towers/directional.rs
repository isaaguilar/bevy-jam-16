use bevy::{ecs::component::Component, reflect::Reflect};

use crate::level::resource::CellDirection;

#[derive(Component, Reflect, Debug, Clone, Copy)]
pub struct FireDirection(pub CellDirection);

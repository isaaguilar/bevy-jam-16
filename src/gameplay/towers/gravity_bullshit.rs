use bevy::{
    ecs::{component::Component, entity::Entity},
    reflect::Reflect,
};

#[derive(Clone, Copy, Debug, Reflect, Component, PartialEq, Eq)]
pub struct DropRange(Entity);

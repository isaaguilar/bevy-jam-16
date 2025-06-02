use bevy::{ecs::resource::Resource, reflect::Reflect};

#[derive(Resource, Default, Debug, Clone, Copy, Hash, PartialEq, PartialOrd, Ord, Eq, Reflect)]
pub struct Lives(usize);

#[derive(Resource, Default, Debug, Clone, Copy, Hash, PartialEq, PartialOrd, Ord, Eq, Reflect)]
pub struct Money(isize);

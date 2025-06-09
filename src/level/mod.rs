use bevy::{
    app::{App, Update},
    color::Color,
    ecs::{
        query::With,
        system::{Query, command::insert_resource},
    },
    gizmos::gizmos::Gizmos,
    math::{Vec2, Vec3Swizzles},
    platform::collections::HashMap,
    transform::components::Transform,
};
use components::{Architecture, Ceiling, Floor, LevelParent, PathNode, Wall, WallDirection};
use resource::Level;

use crate::level::resource::*;

pub mod components;
pub mod resource;

pub const START_LEVEL: usize = 0;

pub fn plugin(app: &mut App) {
    app.insert_resource(Level::default())
        .insert_resource(CurrentLoadedLevel(START_LEVEL))
        .insert_resource(LevelSelect(START_LEVEL))
        .insert_resource(UnlockedLevels(vec![START_LEVEL]));

    app.register_type::<Level>()
        .register_type::<WallDirection>()
        .register_type::<Architecture>()
        .register_type::<PathNode>()
        .register_type::<LevelParent>()
        .register_type::<Floor>()
        .register_type::<Ceiling>()
        .register_type::<Wall>();

    app.add_event::<GotoNextLevel>();
}

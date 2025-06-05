//! Demo gameplay. All of these modules are only intended for demonstration
//! purposes and should be replaced with your own game logic.
//! Feel free to change the logic found here if you feel like tinkering around
//! to get a feeling for the template.

use avian2d::{
    PhysicsPlugins,
    prelude::{Gravity, PhysicsDebugPlugin},
};
use bevy::prelude::*;

mod animation;
pub mod enemy_health;
pub mod enemy_movement;
mod movement;
pub mod player;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        PhysicsPlugins::default().with_length_unit(2.0),
        #[cfg(debug_assertions)]
        PhysicsDebugPlugin::default(),
        enemy_movement::plugin,
        enemy_health::plugin,
        animation::plugin,
        movement::plugin,
        player::plugin,
    ))
    .insert_resource(Gravity(Vec2::new(0., -30.)));
}

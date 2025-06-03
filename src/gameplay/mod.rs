use bevy::prelude::*;

pub mod hotbar;
pub mod tower_placement;
pub mod towers;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((hotbar::plugin, tower_placement::plugin));
}

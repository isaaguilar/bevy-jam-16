use bevy::prelude::*;

pub mod hotbar;
pub mod level;
pub mod tower_placement;
pub mod wave_manager;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        hotbar::plugin,
        level::plugin,
        tower_placement::plugin,
        wave_manager::plugin,
    ));
}

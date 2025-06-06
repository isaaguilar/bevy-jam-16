use bevy::prelude::*;

pub mod animation;
pub mod hotbar;
pub mod hud;
pub mod level;
pub mod status_effects;
pub mod tower_placement;
pub mod towers;
pub mod wave_manager;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        hotbar::plugin,
        level::plugin,
        status_effects::plugin,
        towers::plugin,
        tower_placement::plugin,
        wave_manager::plugin,
        animation::plugin,
        hud::plugin,
    ));
}

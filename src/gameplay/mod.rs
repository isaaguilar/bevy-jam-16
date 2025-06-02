use bevy::prelude::*;

mod hotbar;
mod wave_manager;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(hotbar::plugin);
}

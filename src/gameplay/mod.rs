use bevy::prelude::*;

mod hotbar;
mod tower_placement;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((hotbar::plugin, tower_placement::plugin));
}

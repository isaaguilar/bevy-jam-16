use bevy::app::App;

pub mod enemies;
pub mod physics;
pub mod towers;
pub mod utils;
pub mod wizardry;

pub fn plugin(app: &mut App) {
    utils::plugin(app);
}

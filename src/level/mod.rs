use bevy::app::App;
use components::{Ceiling, Floor, LevelParent, Wall, WallDirection};
use resource::Level;

pub mod components;
pub mod pieces;
pub mod resource;

pub fn plugin(app: &mut App) {
    app.insert_resource(Level::default());
    app.register_type::<Level>()
        .register_type::<WallDirection>()
        .register_type::<LevelParent>()
        .register_type::<Floor>()
        .register_type::<Ceiling>()
        .register_type::<Wall>();
}

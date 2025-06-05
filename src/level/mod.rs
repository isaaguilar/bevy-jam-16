use bevy::{
    app::{App, Update},
    color::Color,
    ecs::{query::With, system::Query},
    gizmos::gizmos::Gizmos,
    math::{Vec2, Vec3Swizzles},
    transform::components::Transform,
};
use components::{Architecture, Ceiling, Floor, LevelParent, PathNode, Wall, WallDirection};
use resource::Level;

pub mod components;
pub mod resource;

pub fn plugin(app: &mut App) {
    app.insert_resource(Level::default());
    app.register_type::<Level>()
        .register_type::<WallDirection>()
        .register_type::<Architecture>()
        .register_type::<PathNode>()
        .register_type::<LevelParent>()
        .register_type::<Floor>()
        .register_type::<Ceiling>()
        .register_type::<Wall>();

    app.add_systems(Update, draw_nodes);
}

pub fn draw_nodes(mut gizmos: Gizmos, nodes: Query<(&Transform, &PathNode), With<PathNode>>) {
    for (pos, PathNode(dir)) in nodes.iter() {
        let node_pos = pos.translation.xy();
        gizmos.circle_2d(node_pos, 0.5, Color::WHITE);
        let node_offset = match dir {
            resource::CellDirection::Up => Vec2::new(0., 1.),
            resource::CellDirection::Down => Vec2::new(0., -1.),
            resource::CellDirection::Left => Vec2::new(-1., 0.),
            resource::CellDirection::Right => Vec2::new(1., 0.),
        };
        gizmos.arrow_2d(node_pos, node_pos + node_offset * 3., Color::WHITE);
    }
}

use avian2d::prelude::{Collider, CollisionLayers, RigidBody, ShapeCaster};
use bevy::{
    color::palettes::basic::*,
    math::{Vec2, Vec3},
    prelude::*,
    render::view::Visibility,
    state::state_scoped::StateScoped,
    transform::components::Transform,
};

use bevy_composable::{app_impl::ComponentTreeable, tree::ComponentTree, wrappers::name};
use std::default::Default;

use super::{
    physics::GamePhysicsLayer as GPL,
    utils::{image, layout},
};
use crate::{
    assets::{GameAssets, game_assets},
    demo::{
        enemy_health::{self, EnemyHealthBar},
        enemy_movement::{
            EnemyController, MovementAcceleration, MovementBundle, MovementDampingFactor,
            MovementDirection,
        },
    },
    level::components::pos,
    screens::Screen,
};

pub fn basic_trooper(game_assets: &GameAssets) -> ComponentTree {
    name("Minor Trooper") + enemy_requirements(Vec2::new(3., 3.5), 35.)
        << Transform::from_scale(Vec3::splat(0.2)).store()
            + image(GameAssets::badguy)
            + layout(GameAssets::badguy_layout)
}

pub fn chonkus_trooper(game_assets: &GameAssets) -> ComponentTree {
    name("Minor Trooper") + enemy_requirements(Vec2::new(4., 5.0), 25.)
        << Transform::from_scale(Vec3::splat(0.25)).store()
            + image(GameAssets::ducky)
            + layout(GameAssets::badguy_layout)
}

pub fn turbo_trooper(
    game_assets: &GameAssets, // not used
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
) -> ComponentTree {
    name("Minor Trooper") + enemy_requirements(Vec2::new(2., 2.5), 45.)
        << Transform::from_scale(Vec3::splat(0.15)).store()
            + image(GameAssets::ducky)
            + layout(GameAssets::badguy_layout)
        << Transform::from_scale(Vec3::splat(0.15)).store() + health_requirements(meshes, materials)

    // << health_requirements(meshes, materials)
}

pub fn enemy_requirements(size: Vec2, speed: f32) -> ComponentTree {
    (
        // Transform::from_scale(Vec3::splat(scale)),
        StateScoped(Screen::Gameplay),
        EnemyController,
        MovementDirection::default(),
        RigidBody::Kinematic,
        Visibility::Visible,
        MovementAcceleration(speed),
        MovementDampingFactor(0.96),
        Collider::round_rectangle(size.x, size.y, 0.5),
        CollisionLayers::new(GPL::Enemy, [GPL::Default, GPL::Level, GPL::Projectiles]),
    )
        .store()
}

pub fn health_requirements(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) -> ComponentTree {
    let enemy_health_bar = EnemyHealthBar::new(32., 3.0);

    let mesh = Mesh::from(enemy_health_bar.mesh_shape);
    let mesh_handle = meshes.add(mesh);

    (
        enemy_health_bar,
        Mesh2d(mesh_handle.clone()),
        MeshMaterial2d(materials.add(Color::from(GREEN))),
        Transform::from_translation(Vec3::new(0., 14., 0.)),
    )
        .store()
}

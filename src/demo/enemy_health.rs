use crate::{AppSystems, PausableSystems, assets::game_assets::HEALTH_BAR_WIDTH};
use avian2d::prelude::OnCollisionStart;
use bevy::{
    color::palettes::basic::*,
    ecs::{component::HookContext, world::DeferredWorld},
    prelude::*,
};

use super::enemy_movement::EnemyController;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        update_health_bar
            .in_set(AppSystems::Update)
            .in_set(PausableSystems),
    );

    app.add_observer(hit_player);
}

// #[derive(Component, Default)]
// struct EnemyHealth(f32);

#[derive(Component, Default, Clone, Copy, PartialEq, Reflect)]
pub struct EnemyHealth(pub f32);

#[derive(Component, Default, Clone, Copy, PartialEq, Reflect)]
pub struct EnemyHealthBar;

#[derive(Component)]
#[component(on_add = calculate_damage)]
pub struct Damage(f32);

// /// A bundle that contains the components needed for a basic
// /// kinematic character controller.
// #[derive(Bundle)]
// pub struct EnemyHealthBarBundle {
//     pub enemy_health_bar: EnemyHealthBar,
// }

impl EnemyHealth {
    pub fn new() -> Self {
        Self(1.0)
    }
}

pub fn health_bar_spawn(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) -> impl Bundle {
    let enemy_health_bar = EnemyHealthBar;

    let mesh = Mesh::from(Rectangle::new(HEALTH_BAR_WIDTH, 3.));
    let mesh_handle = meshes.add(mesh);
    (
        enemy_health_bar,
        Mesh2d(mesh_handle.clone()),
        MeshMaterial2d(materials.add(Color::from(GREEN))),
        Transform::from_translation(Vec3::new(0., 14., 0.)),
    )
}

fn update_health_bar(
    mut bars: Query<(Entity, &mut Transform), With<EnemyHealthBar>>,
    relationships: Query<(&ChildOf)>,
    enemies: Query<&EnemyHealth>,
) {
    for (e, mut transform) in bars.iter_mut() {
        if let Ok(parent_health) = enemies.get(relationships.root_ancestor(e)) {
            transform.scale.x = parent_health.0;
            transform.translation.x = -(HEALTH_BAR_WIDTH * (1.0 - parent_health.0)) / 2.0;
        }
    }
}

fn hit_player(
    trigger: Trigger<OnCollisionStart>,
    enemies: Query<Entity, With<EnemyController>>,
    mut commands: Commands,
) {
    let Some(body) = trigger.event().body else {
        return;
    };
    let collider = trigger.event().collider;

    for entity in enemies {
        info!(?entity);
        if body == entity || collider == entity {
            info!(target=?trigger.target(),event=?trigger.event(), "Player touched the enemy" )
        }
        commands.entity(entity).insert(Damage(0.25));
        commands.entity(entity).remove::<Damage>();
    }
}

fn calculate_damage(mut _world: DeferredWorld, HookContext { entity, .. }: HookContext) {
    info!(?entity, "on_add")
}

use crate::{
    assets::TowerSprites,
    data::PointerInteractionState,
    level::components::{Ceiling, Floor, Wall, WallDirection},
    utils::destroy_entity,
};
use bevy::prelude::*;
use std::f32::consts::PI;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(on_turret_placement_hover);
    app.add_observer(on_turret_placement_exit);
    app.add_systems(
        OnEnter(PointerInteractionState::Selecting),
        destroy_entity::<TowerPreview>,
    );
}

#[derive(Component, Debug, Clone, Copy, Reflect)]
struct TowerPreview;

fn on_turret_placement_hover(
    trigger: Trigger<Pointer<Over>>,
    walls: Query<&Wall>,
    floors: Query<(), With<Floor>>,
    ceilings: Query<(), With<Ceiling>>,
    pointer_input_state: Res<State<PointerInteractionState>>,
    sprites: Option<Res<TowerSprites>>,
    mut commands: Commands,
) {
    let PointerInteractionState::Placing(tower) = pointer_input_state.get() else {
        return;
    };
    let sprites = sprites.expect("GameAssets should be available during turret placement");

    let entity = trigger.target;

    let get_sprite = || (sprites.tower_bundle(tower), TowerPreview);

    if let Ok(wall) = walls.get(entity) {
        let transform = match wall.0 {
            WallDirection::Left => Transform::from_xyz(-5., 0., 0.),
            WallDirection::Right => Transform::from_xyz(5., 0., 0.),
        };
        let preview = get_sprite();
        commands.entity(entity).with_child((preview, transform));
    } else if ceilings.get(entity).is_ok() {
        let preview = get_sprite();
        commands.entity(entity).with_child((
            preview,
            Transform::from_xyz(0., -5., 0.).with_rotation(Quat::from_rotation_x(PI)),
        ));
    } else if floors.get(entity).is_ok() {
        let preview = get_sprite();
        commands
            .entity(entity)
            .with_child((preview, Transform::from_xyz(0., 5., 0.)));
    }
}

fn on_turret_placement_exit(
    trigger: Trigger<Pointer<Out>>,
    walls: Query<(), With<Wall>>,
    floors: Query<(), With<Floor>>,
    ceilings: Query<(), With<Ceiling>>,
    children_query: Query<&Children>,
    preview_query: Query<(), With<TowerPreview>>,
    // mut commands: Commands,
) {
    let entity = trigger.target;

    // Only proceed if the exited entity is a structure we're interested in
    if walls.get(entity).is_err() && ceilings.get(entity).is_err() && floors.get(entity).is_err() {
        return;
    }

    if let Ok(children) = children_query.get(entity) {
        for &child in children {
            if preview_query.get(child).is_ok() {
                // TODO
                // commands.entity(child).despawn();
                // break;
            }
        }
    }
}

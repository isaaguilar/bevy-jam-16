use crate::{
    assets::TowerSprites,
    data::*,
    level::components::{Ceiling, Floor, Wall, WallDirection},
    utils::destroy_entity,
};
use bevy::prelude::*;
use std::f32::consts::PI;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<TowerPlacementState>();
    app.add_observer(on_turret_placement_hover);
    app.add_systems(
        Update,
        tower_placement_change.run_if(state_changed::<TowerPlacementState>),
    );
    app.add_systems(
        OnEnter(PointerInteractionState::Selecting),
        destroy_entity::<TowerPreview>,
    );
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
enum TowerPlacement {
    Below,
    Above,
    Left,
    Right,
}

#[derive(States, Default, Debug, Hash, PartialEq, Eq, Clone)]
enum TowerPlacementState {
    #[default]
    None,
    Placing(Tower, Entity, TowerPlacement),
}

#[derive(Component, Debug, Clone, Copy, Reflect)]
struct TowerPreview;

fn tower_placement_change(
    tower_placement_state: Res<State<TowerPlacementState>>,
    previews: Query<Entity, With<TowerPreview>>,
    sprites: Option<Res<TowerSprites>>,
    mut commands: Commands,
) {
    let tower_placement_state = tower_placement_state.get();
    let TowerPlacementState::Placing(tower, parent, placement) = tower_placement_state else {
        return;
    };
    let sprites = sprites.expect("GameAssets should be available during turret placement");

    let transform = match placement {
        TowerPlacement::Above => Transform::from_xyz(0., 5., 0.),
        TowerPlacement::Below => {
            Transform::from_xyz(0., -5., 0.).with_rotation(Quat::from_rotation_z(PI))
        }
        TowerPlacement::Left => {
            Transform::from_xyz(-5., 0., 0.).with_rotation(Quat::from_rotation_z(PI / 2.0))
        }
        TowerPlacement::Right => {
            Transform::from_xyz(5., 0., 0.).with_rotation(Quat::from_rotation_z(-PI / 2.0))
        }
    };

    for entity in previews {
        commands.entity(entity).despawn()
    }

    commands
        .entity(*parent)
        .with_child((sprites.tower_bundle(tower), transform, TowerPreview));
}

fn on_turret_placement_hover(
    trigger: Trigger<Pointer<Over>>,
    walls: Query<&Wall>,
    floors: Query<(), With<Floor>>,
    ceilings: Query<(), With<Ceiling>>,
    pointer_input_state: Res<State<PointerInteractionState>>,
    tower_placement_state: Res<State<TowerPlacementState>>,
    mut next_tower_placement_state: ResMut<NextState<TowerPlacementState>>,
) {
    let PointerInteractionState::Placing(tower) = pointer_input_state.get() else {
        return;
    };

    let entity = trigger.target;

    if let TowerPlacementState::Placing(t, e, _) = tower_placement_state.get() {
        if *t == *tower && *e == entity {
            return;
        }
    }

    if let Ok(wall) = walls.get(entity) {
        let placement = match wall.0 {
            WallDirection::Left => TowerPlacement::Left,
            WallDirection::Right => TowerPlacement::Right,
        };
        next_tower_placement_state.set(TowerPlacementState::Placing(*tower, entity, placement));
    } else if ceilings.get(entity).is_ok() {
        next_tower_placement_state.set(TowerPlacementState::Placing(
            *tower,
            entity,
            TowerPlacement::Below,
        ));
    } else if floors.get(entity).is_ok() {
        next_tower_placement_state.set(TowerPlacementState::Placing(
            *tower,
            entity,
            TowerPlacement::Above,
        ));
    }
}

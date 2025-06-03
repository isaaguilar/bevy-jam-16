use crate::assets::TowerSprites;
use crate::data::PointerInteractionState;
use crate::level::components::{Ceiling, Floor, Wall};
use crate::utils::destroy_entity;
use bevy::prelude::*;
use std::f32::consts::PI;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(on_turret_placement_hover);
    app.add_systems(
        OnEnter(PointerInteractionState::Selecting),
        destroy_entity::<TowerPreview>,
    );
}

#[derive(Component, Debug, Clone, Copy, Reflect)]
struct TowerPreview;

pub fn on_turret_placement_hover(
    trigger: Trigger<Pointer<Over>>,
    walls: Query<(), With<Wall>>,
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

    if walls.get(entity).is_ok() {
        println!("{entity:?} is a wall");
        let preview = get_sprite();
        commands.entity(entity).with_child(preview);
    } else if ceilings.get(entity).is_ok() {
        println!("{entity:?} is a ceiling");
        let preview = get_sprite();
        commands.entity(entity).with_child((
            preview,
            Transform::from_xyz(0., -5., 0.).with_rotation(Quat::from_rotation_x(PI)),
        ));
    } else if floors.get(entity).is_ok() {
        println!("{entity:?} is a floor");
        let preview = get_sprite();
        commands
            .entity(entity)
            .with_child((preview, Transform::from_xyz(0., 5., 0.)));
    }
}

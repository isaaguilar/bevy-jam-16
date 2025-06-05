use bevy::prelude::*;
use projectiles::{AttackEffect, AttackType, DamageType, Droplet, LiquidType, Puddle};

use crate::prelude::*;

mod input_state;
pub mod projectiles;
mod state;
mod status_effects;
mod towers;

pub use {
    input_state::PointerInteractionState, state::PlayerState, status_effects::Ailments,
    status_effects::StatusEffect, status_effects::get_ailment, towers::Tower,
    towers::TowerCollision, towers::get_collision,
};

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<PlayerState>();
    app.init_state::<PointerInteractionState>();

    app.register_type::<AttackType>()
        .register_type::<AttackEffect>()
        .register_type::<Droplet>()
        .register_type::<Puddle>()
        .register_type::<LiquidType>()
        .register_type::<DamageType>()
        .register_type::<PlayerState>()
        .register_type::<StatusEffect>()
        .register_type::<Ailments>()
        .register_type::<Tower>()
        .register_type::<PointerInteractionState>();

    app.add_systems(OnExit(Screen::Loading), validate_assets);
}

fn validate_assets(ui_assets: Res<UiAssets>) {
    for tower in Tower::all() {
        let ui_key = tower.ui_asset_key();
        assert!(
            ui_assets.hotbar_icons.contains_key(ui_key),
            "missing ui asset for {tower:?} ({ui_key})"
        );
    }
}

use std::sync::Arc;

use bevy::prelude::*;

use crate::assets::{SoundEffects, sound_effects::SoundFn};

use super::{
    projectiles::{AttackSpecification, DamageType, LiquidType, TowerAttackType},
    status_effects::StatusEnum,
};

#[derive(Component, Copy, Clone, Eq, PartialEq, Hash, Debug, Reflect)]
pub enum Tower {
    Piston,
    Fan,
    SpikePit,
    Oil,
    TrapDoor,
    Ice,
    Acid,
    Tesla,
    Water,
    Flame,
    Portal,
}

impl Tower {
    pub fn all() -> Vec<Tower> {
        vec![
            Tower::Piston,
            Tower::Fan,
            Tower::SpikePit,
            Tower::Oil,
            Tower::TrapDoor,
            Tower::Ice,
            Tower::Acid,
            Tower::Tesla,
            Tower::Water,
            Tower::Flame,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            Tower::Piston => "Piston",
            Tower::Fan => "Fan",
            Tower::SpikePit => "Spike Pit",
            Tower::Oil => "Oil",
            Tower::TrapDoor => "Trap Door",
            Tower::Ice => "Ice",
            Tower::Acid => "Acid",
            Tower::Tesla => "Tesla Turret",
            Tower::Water => "Water Bucket",
            Tower::Flame => "Flame",
            Tower::Portal => "Portal",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Tower::Piston => "Shoves enemies with strong mechanical force, and shatters ice.",
            Tower::Fan => "Pushes against enemies. Use when they can't move.",
            Tower::SpikePit => "Triggers hidden spikes to damage passing enemies.",
            Tower::Oil => "Oil makes enemies slippery and flammable.",
            Tower::TrapDoor => "50% chance to let enemies through.",
            Tower::Ice => "Slows enemies and freezes water.",
            Tower::Acid => "Acid weakens enemies to all damage.",
            Tower::Tesla => "Shocks enemies. Ignites oil and bounces off water.",
            Tower::Water => "Slows and damages, empowers lightning and ice.",
            Tower::Flame => {
                "Burns enemies, ignites oil. Enemies cooked without oil are weak to cold."
            }
            Tower::Portal => "Teleports enemies backward along their path.",
        }
    }

    pub fn price(&self) -> i32 {
        match self {
            Tower::Piston => 50,
            Tower::Fan => 50,
            Tower::SpikePit => 25,
            Tower::Oil => 30,
            Tower::TrapDoor => 75,
            Tower::Ice => 100,
            Tower::Acid => 40,
            Tower::Tesla => 40,
            Tower::Water => 50,
            Tower::Flame => 40,
            Tower::Portal => 150,
        }
    }

    pub fn ui_asset_key(&self) -> &'static str {
        match self {
            Tower::Piston => "icon_piston",
            Tower::Fan => "icon_fan",
            Tower::SpikePit => "icon_spike_pit",
            Tower::Oil => "icon_oil",
            Tower::TrapDoor => "icon_trapdoor",
            Tower::Ice => "icon_ice",
            Tower::Acid => "icon_acid",
            Tower::Tesla => "icon_tesla",
            Tower::Water => "icon_water_bucket",
            Tower::Flame => "icon_flame",
            Tower::Portal => "icon_portal",
        }
    }

    pub fn has_trigger_zone(&self) -> bool {
        match self {
            Tower::Fan => false,
            _ => true,
        }
    }

    pub fn gravity_influences_trigger(&self) -> bool {
        match self {
            Tower::Oil | Tower::Acid | Tower::Water => true,
            _ => false,
        }
    }

    pub fn custom_trigger_zone(&self) -> Option<Vec2> {
        match self {
            Tower::TrapDoor => Some(Vec2::new(10.0, 10.0)),
            _ => None,
        }
    }

    pub fn attack_def(&self) -> TowerAttackType {
        match self {
            Tower::Piston => TowerAttackType::EntireCell(vec![
                AttackSpecification::Damage(DamageType::Physical, 10),
                AttackSpecification::Push(800.),
            ]),
            Tower::Fan => TowerAttackType::EntireCell(vec![AttackSpecification::Push(10.)]),
            //Tower::SpikePit => TowerAttackType::Contact(vec![AttackSpecification::Damage(
            //    DamageType::Physical,
            //    10,
            //)]),
            Tower::Oil => TowerAttackType::DropsLiquid(LiquidType::Oil),
            Tower::TrapDoor => TowerAttackType::ModifiesSelf,
            Tower::Ice => TowerAttackType::EntireCell(vec![
                AttackSpecification::Damage(DamageType::Cold, 10),
                AttackSpecification::Status(StatusEnum::Chilled),
            ]),
            Tower::SpikePit => {
                TowerAttackType::EntireCell(vec![AttackSpecification::Damage(
                    DamageType::Physical,
                    10,
                )])
                // Since the sprite of spikes are large there isn't a case where an
                // enemy will not be touching if they are in the cell.
                // AttackType::Contact(vec![AttackEffect::Damage(DamageType::Physical)])
            }
            Tower::Acid => TowerAttackType::DropsLiquid(LiquidType::Acid),
            Tower::Tesla => TowerAttackType::EntireCell(vec![AttackSpecification::Damage(
                DamageType::Lightning,
                15,
            )]),
            Tower::Water => TowerAttackType::DropsLiquid(LiquidType::Water),
            Tower::Flame => TowerAttackType::EntireCell(vec![
                AttackSpecification::Damage(DamageType::Burning, 10),
                AttackSpecification::Status(StatusEnum::Burned),
            ]),
            Tower::Portal => todo!(),
        }
    }

    pub fn cooldown(&self) -> f32 {
        match self {
            Tower::Piston => 3.5,
            Tower::Fan => 0.,
            Tower::SpikePit => 0.5,
            Tower::Oil => 5.0,
            Tower::TrapDoor => 3.0,
            Tower::Ice => 1.5,
            Tower::Acid => 5.0,
            Tower::Tesla => 0.67,
            Tower::Water => 5.0,
            Tower::Flame => 0.67,
            Tower::Portal => 3.0,
        }
    }

    pub fn requires_adjecent_wall(&self) -> bool {
        match self {
            Tower::TrapDoor => true,
            _ => false,
        }
    }

    pub fn requires_floor_placement(&self) -> bool {
        match self {
            Tower::TrapDoor => true,
            _ => false,
        }
    }

    pub fn fire_sfx(&self) -> Option<Arc<dyn SoundFn>> {
        match self {
            Tower::SpikePit => Some(Arc::new(SoundEffects::spike_fire)),
            Tower::Tesla => Some(Arc::new(SoundEffects::tesla_fire)),
            Tower::Piston => Some(Arc::new(SoundEffects::piston_fire)),
            Tower::Oil => Some(Arc::new(SoundEffects::oil_fire)),
            Tower::Water => Some(Arc::new(SoundEffects::water_fire)),
            _ => None,
        }
    }
}

#[derive(Component, Default)]
pub struct TowerCollision {
    pub slowdown: f32,
    pub damage: isize,
    pub iframe: Timer,
}

impl TowerCollision {
    fn new(slowdown: f32, damage: isize, iframe_time_s: f32) -> Self {
        Self {
            damage,
            slowdown,
            iframe: Timer::from_seconds(iframe_time_s, TimerMode::Repeating),
        }
    }
}

// These towers by themselves will cause damage upon collision
pub fn get_collision(tower: &Tower) -> Option<TowerCollision> {
    match tower {
        Tower::SpikePit => Some(TowerCollision::new(0.0, 15, 0.75)),
        _ => None,
    }
}

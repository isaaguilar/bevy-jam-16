use bevy::prelude::*;
use std::sync::Arc;

use super::status_effects::{StatusEffectTrait, StatusEnum};

#[derive(Clone, Debug, Reflect, PartialEq, Eq)]
pub enum AttackType {
    EntireCell(Vec<AttackEffect>),
    Contact(Vec<AttackEffect>),
    DropsLiquid(LiquidType),
    ModifiesSelf,
}

#[derive(Clone, Debug, Reflect, PartialEq, Eq)]
pub enum AttackEffect {
    Damage(DamageType),
    Push,
    Status(StatusEnum),
}

#[derive(Component, Clone, Debug, Reflect, PartialEq, Eq)]
pub struct Lifetime(pub Timer);

#[derive(Component, Copy, Clone, Debug, Reflect, PartialEq, Eq)]
pub struct Droplet(pub LiquidType);

#[derive(Component, Copy, Clone, Debug, Reflect, PartialEq, Eq)]
pub struct Puddle(pub LiquidType);

#[derive(Copy, Clone, Debug, Reflect, PartialEq, Eq)]
pub enum LiquidType {
    Water,
    Oil,
    Acid,
}

#[derive(Copy, Clone, Debug, Reflect, PartialEq, Eq)]
pub enum DamageType {
    Physical,
    Burning,
    Cold,
    Lightning,
    Chemical,
}

impl DamageType {
    pub fn status_effect(&self) -> StatusEffect {
        match self {
            DamageType::Physical => StatusEffect::Slowed,
            DamageType::Burning => StatusEffect::Burned,
            DamageType::Cold => StatusEffect::Frozen,
            DamageType::Lightning => StatusEffect::Electrified,
            DamageType::Chemical => StatusEffect::Acidic,
        }
    }
}

impl Lifetime {
    pub fn new(duration: f32) -> Self {
        Lifetime(Timer::from_seconds(duration, bevy::time::TimerMode::Once))
    }
}

impl LiquidType {
    pub fn contact_effects(&self) -> Vec<AttackEffect> {
        match self {
            LiquidType::Water => vec![AttackEffect::Status(StatusEnum::Wet)],
            LiquidType::Oil => vec![AttackEffect::Status(StatusEnum::Oiled)],
            LiquidType::Acid => vec![
                AttackEffect::Damage(DamageType::Chemical),
                AttackEffect::Status(StatusEnum::Acidified),
            ],
        }
    }
}

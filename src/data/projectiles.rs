use bevy::{
    color::palettes::css::{AQUA, GREY, LIME, ORANGE, YELLOW},
    prelude::*,
};
use std::sync::Arc;

use super::status_effects::{StatusEffect, StatusEffectTrait, StatusEnum};
use crate::level::resource::CellDirection;

#[derive(Clone, Debug, Reflect, PartialEq)]
pub enum TowerAttackType {
    EntireCell(Vec<AttackSpecification>),
    Contact(Vec<AttackSpecification>),
    DropsLiquid(LiquidType),
    ModifiesSelf,
}

#[derive(Clone, Debug, Reflect, PartialEq)]
pub enum AttackSpecification {
    Damage(DamageType, usize),
    Push(f32),
    Status(StatusEnum),
}

#[derive(Clone, Debug, Reflect, PartialEq)]
pub enum AttackData {
    Damage {
        dmg_type: DamageType,
        strength: usize,
        damage: usize,
    },
    Push {
        direction: CellDirection,
        strength: usize,
        force: f32,
    },
    Status {
        status: StatusEnum,
        strength: usize,
    },
}

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
    pub fn color(&self) -> Color {
        match self {
            DamageType::Physical => GREY.into(),
            DamageType::Burning => ORANGE.into(),
            DamageType::Cold => AQUA.into(),
            DamageType::Lightning => YELLOW.into(),
            DamageType::Chemical => LIME.into(),
        }
    }
}

impl LiquidType {
    pub fn contact_effects(&self) -> Vec<AttackSpecification> {
        match self {
            LiquidType::Water => vec![AttackSpecification::Status(StatusEnum::Wet)],
            LiquidType::Oil => vec![AttackSpecification::Status(StatusEnum::Oiled)],
            LiquidType::Acid => vec![
                AttackSpecification::Damage(DamageType::Chemical, 10),
                AttackSpecification::Status(StatusEnum::Acidified),
            ],
        }
    }
}

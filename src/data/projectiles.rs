use bevy::reflect::Reflect;

use super::StatusEffect;

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
    Status(StatusEffect),
}

#[derive(Copy, Clone, Debug, Reflect, PartialEq, Eq)]
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
}

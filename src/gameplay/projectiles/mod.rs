pub enum AttackType {
    EntireCell,
    Contact,
    DropsLiquid(pub LiquidType),
    VectorField,
    ModifiesSelf,
}

pub enum LiquidType {
    Water,
    Oil,
    Acid,
}

pub enum DamageType {
    Physical,
    Burning,
    Cold,
    Lightning,
}

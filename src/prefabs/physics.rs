use avian2d::prelude::PhysicsLayer;

#[derive(PhysicsLayer, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum GamePhysicsLayer {
    #[default]
    Default, // Layer 0 - the default layer that objects are assigned to. Probably don't use.
    Level,
    Enemy,
    Projectiles,
    Ethereal,
}

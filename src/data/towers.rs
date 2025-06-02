use bevy::prelude::*;

#[derive(Component, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Tower {
    Piston,
    SpikePit,
    Oil,
    TrapDoor,
    Tesla,
    Water,
}

impl Tower {
    pub fn all() -> Vec<Tower> {
        vec![
            Tower::Piston,
            Tower::SpikePit,
            Tower::Oil,
            Tower::TrapDoor,
            Tower::Tesla,
            Tower::Water,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            Tower::Piston => "Piston",
            Tower::SpikePit => "Spike Pit",
            Tower::Oil => "Oil",
            Tower::TrapDoor => "Trap Door",
            Tower::Water => "Water Bucket",
            Tower::Tesla => "Tesla Turret",
        }
    }

    pub fn ui_asset_key(&self) -> &'static str {
        match self {
            Tower::Piston => "icon_piston",
            Tower::SpikePit => "icon_spike_pit",
            Tower::Oil => "icon_oil",
            Tower::TrapDoor => "icon_trapdoor",
            Tower::Water => "icon_water_bucket",
            Tower::Tesla => "icon_tesla",
        }
    }
}

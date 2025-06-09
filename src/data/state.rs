use bevy::prelude::*;

#[derive(Resource, Clone, Copy, Reflect, Debug)]
pub struct PlayerState {
    pub money: i32,
    pub health: i32,
}

impl Default for PlayerState {
    fn default() -> Self {
        PlayerState {
            money: 400,
            health: 25,
        }
    }
}

impl PlayerState {
    pub fn can_afford(&self, cost: i32) -> bool {
        self.money >= cost
    }
}

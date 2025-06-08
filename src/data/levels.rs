use bevy::prelude::*;
use std::collections::VecDeque;

use crate::gameplay::wave_manager::Wave;
use crate::prefabs::enemies::{basic_trooper, chonkus_trooper, turbo_trooper};

pub const MAP_TEXT: &'static str = ">>>
";

pub const MAP_TEXT1: &'static str = "^<<v<
>>^v^
^<<<^
>>>>^
";

pub const MAP_TEXT2: &'static str = "^<<<<
>>>>^
^<<<<
>>>>^
";

pub const MAP_TEXT3: &'static str = "^<<<<<<<<<<<<<
>>>>>>>>>>>>>^";

pub const MAP_TEXT4: &'static str = "^v<<<v<
^<>v^<^
>>^>>>^
";

pub const MAP_TEXT5: &'static str = "^<<<<
>v>v^
^>^>^
";

pub const MAP_TEXT6: &'static str = ">v>^v<
^v^v<^
^v^v>^
^v^<^<
^>>>>^
";

pub const MAP_TEXT7: &'static str = "^>>>>>v
x^v<<<v
x^v>>^v
^<<^<<v
>>>>v^v
^<<<v^v
>>>^>^>
";

#[derive(Resource, Clone)]
pub struct LevelData {
    pub maps: Vec<&'static str>,
    pub enemies: Vec<VecDeque<Wave>>,
}

impl Default for LevelData {
    fn default() -> Self {
        Self {
            maps: vec![
                MAP_TEXT, MAP_TEXT1, MAP_TEXT2, MAP_TEXT3, MAP_TEXT4, MAP_TEXT5, MAP_TEXT6,
                MAP_TEXT7,
            ],
            enemies: vec![test_waves()],
        }
    }
}

pub fn test_waves() -> VecDeque<Wave> {
    vec![
        //
        // Wave 1
        vec![
            (vec![basic_trooper()], 2.),
            (vec![basic_trooper(), turbo_trooper()], 0.),
        ]
        .into(),
    ]
    .into()
}

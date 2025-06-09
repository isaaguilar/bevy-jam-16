use crate::data::Tower;
use bevy::prelude::*;

#[derive(Resource, Clone, Copy, Debug, Hash, Reflect, PartialEq, Eq)]
pub struct CurrentLoadedLevel(pub usize);

#[derive(Resource, Clone, Copy, Debug, Hash, Reflect, PartialEq, Eq)]
pub struct LevelSelect(pub usize);

#[derive(Resource, Clone, Debug, Hash, Reflect, PartialEq, Eq)]
pub struct UnlockedLevels(pub Vec<usize>);

#[derive(Event, Debug, Hash, PartialEq, Eq, Clone, Reflect)]
pub struct GotoNextLevel(pub usize);

// Which direction the enemies need to move in. If we end up adding splitting paths, this won't be
// usable. It's mainly to help get a FWP going.
#[derive(Component, Clone, Copy, Debug, Hash, Reflect, PartialEq, Eq)]
pub enum CellDirection {
    Up,
    Down,
    Left,
    Right,
}

impl From<char> for CellDirection {
    fn from(value: char) -> Self {
        match value {
            '>' => CellDirection::Right,
            '^' => CellDirection::Up,
            '<' => CellDirection::Left,
            'v' => CellDirection::Down,
            'x' | 'X' => CellDirection::Up,
            _ => todo!(),
        }
    }
}

impl Into<Vec2> for CellDirection {
    fn into(self) -> Vec2 {
        match self {
            CellDirection::Up => Vec2::Y,
            CellDirection::Down => Vec2::NEG_Y,
            CellDirection::Left => Vec2::NEG_X,
            CellDirection::Right => Vec2::X,
        }
    }
}

impl CellDirection {
    pub fn vec(self) -> Vec2 {
        self.into()
    }

    pub fn sprite_offset(&self, tower: &Tower) -> Transform {
        match tower {
            Tower::TrapDoor => Transform::from_xyz(0., 0., 0.),

            _ => match self {
                CellDirection::Up => Transform::from_xyz(0., -5., 0.),
                CellDirection::Down => Transform::from_xyz(0., 5., 0.),
                CellDirection::Right => {
                    Transform::from_xyz(-5., 0., 0.).with_scale(Vec3::new(-1., 1., 1.))
                }
                CellDirection::Left => Transform::from_xyz(5., 0., 0.),
            },
        }
    }

    pub fn clockwise(&self) -> Self {
        match self {
            CellDirection::Up => CellDirection::Right,
            CellDirection::Down => CellDirection::Left,
            CellDirection::Left => CellDirection::Up,
            CellDirection::Right => CellDirection::Down,
        }
    }

    pub fn counter_clockwise(&self) -> Self {
        match self {
            CellDirection::Up => CellDirection::Left,
            CellDirection::Down => CellDirection::Right,
            CellDirection::Left => CellDirection::Down,
            CellDirection::Right => CellDirection::Up,
        }
    }

    pub fn flip(&self) -> Self {
        match self {
            CellDirection::Up => CellDirection::Down,
            CellDirection::Down => CellDirection::Up,
            CellDirection::Left => CellDirection::Right,
            CellDirection::Right => CellDirection::Left,
        }
    }
}

// Holds all the information necessary to load a level to the game
#[derive(Clone, Resource, Debug, Default, Reflect)]
pub struct Level {
    pub path: Vec<(Vec2, CellDirection)>,
    pub width: usize,
    pub height: usize,
    pub walls: Vec<Vec<bool>>,
    pub floors: Vec<Vec<bool>>,
}

impl Level {
    pub fn new(
        path: Vec<(Vec2, CellDirection)>,
        width: usize,
        height: usize,
        walls: Vec<Vec<bool>>,
        floors: Vec<Vec<bool>>,
    ) -> Self {
        Self {
            path,
            width,
            height,
            walls,
            floors,
        }
    }

    // Takes a string representing a rectangular grid of <v>^ characters
    pub fn from_str<I: Into<String>>(map_str: I) -> Self {
        let map_str: String = map_str.into();
        let lines: Vec<Vec<CellDirection>> = map_str
            .lines()
            .map(|w| w.chars().map(|v| v.into()).collect())
            .rev()
            .collect();

        let height = lines.len();
        let width = lines[0].len();
        // We create the level with all possible walls and floors, and delete them later
        let mut level = Self::new(
            Vec::new(),
            width,
            height,
            (0..(width + 1))
                .map(|_x| (0..height).map(|_y| true).collect())
                .collect(),
            (0..width)
                .map(|_x| (0..(height + 1)).map(|_y| true).collect())
                .collect(),
        );

        let start_pos = Vec2::new(0., 0.);
        let mut x = start_pos.x as usize;
        let mut y = start_pos.y as usize;
        // This loop moves through the maze in a manner dictated by the cell directions and deletes
        // walls/floors as it goes and adds the cells to the stored level path.
        while x < level.width && y < level.height {
            let current_tile = lines[y][x];
            level
                .path
                .push((Vec2::new(x as f32, y as f32), current_tile));
            match current_tile {
                CellDirection::Up => {
                    level.floors[x][y + 1] = false;
                    level
                        .path
                        .push((Vec2::new(x as f32, y as f32 + 0.5), current_tile));
                    y += 1;
                }
                CellDirection::Down => {
                    level.floors[x][y] = false;
                    level
                        .path
                        .push((Vec2::new(x as f32, y as f32 - 0.5), current_tile));
                    y -= 1;
                }
                CellDirection::Left => {
                    level.walls[x][y] = false;
                    level
                        .path
                        .push((Vec2::new(x as f32 - 0.5, y as f32), current_tile));
                    x -= 1;
                }
                CellDirection::Right => {
                    level.walls[x + 1][y] = false;
                    level
                        .path
                        .push((Vec2::new(x as f32 + 0.5, y as f32), current_tile));
                    x += 1;
                }
            }
        }
        level
    }
}

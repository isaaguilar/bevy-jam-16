//! The title screen that appears after the splash screen.

use bevy::prelude::*;

use crate::{
    level::{
        START_LEVEL,
        resource::{LevelSelect, UnlockedLevels},
    },
    menus::Menu,
    screens::Screen,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Title), open_main_menu);
    app.add_systems(OnExit(Screen::Title), close_menu);
}

fn open_main_menu(
    mut next_menu: ResMut<NextState<Menu>>,
    mut level_select: ResMut<LevelSelect>,
    mut unlocked_levels: ResMut<UnlockedLevels>,
) {
    level_select.0 = START_LEVEL;
    unlocked_levels.0 = vec![START_LEVEL];
    next_menu.set(Menu::Main);
}

fn close_menu(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::None);
}

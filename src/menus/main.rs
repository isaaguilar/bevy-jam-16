//! The main menu (seen on the title screen).

use crate::{menus::Menu, screens::Screen, theme::widget};
use bevy::color::palettes::tailwind;
use bevy::prelude::*;

const TITLE_TEXT: &str = "Zod's Tower Combinator";

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::Main), spawn_main_menu);
}

fn spawn_main_menu(mut commands: Commands) {
    commands.spawn((
        widget::ui_root("Main Menu"),
        GlobalZIndex(2),
        StateScoped(Menu::Main),
        #[cfg(not(target_family = "wasm"))]
        children![
            widget::title(TITLE_TEXT),
            widget::button("Play", open_level_selector_menu),
            widget::button("Settings", open_settings_menu),
            widget::button("Credits", open_credits_menu),
            widget::button("Exit", exit_app),
        ],
        #[cfg(target_family = "wasm")]
        children![
            widget::title(TITLE_TEXT),
            widget::button("Play", open_level_selector_menu),
            widget::button("Settings", open_settings_menu),
            widget::button("Credits", open_credits_menu),
        ],
    ));
    commands.insert_resource(ClearColor(tailwind::SLATE_950.into()));
    commands.spawn(());
}

fn enter_loading_or_gameplay_screen(
    _: Trigger<Pointer<Click>>,
    // resource_handles: Res<ResourceHandles>,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    // if resource_handles.is_all_done() {
    next_screen.set(Screen::Gameplay);
    // } else {
    //     next_screen.set(Screen::Loading);
    // }
}

fn open_settings_menu(_: Trigger<Pointer<Click>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Settings);
}

fn open_credits_menu(_: Trigger<Pointer<Click>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Credits);
}

fn open_level_selector_menu(_: Trigger<Pointer<Click>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::LevelSelector);
}

#[cfg(not(target_family = "wasm"))]
fn exit_app(_: Trigger<Pointer<Click>>, mut app_exit: EventWriter<AppExit>) {
    app_exit.write(AppExit::Success);
}

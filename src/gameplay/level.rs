use bevy::color::palettes::tailwind;
use bevy::prelude::*;
use bevy_composable::{
    app_impl::{ComplexSpawnable, ComponentTreeable},
    wrappers::name,
};

use crate::{
    data::PlayerState,
    demo::enemy_health::EnemyHealth,
    level::{
        components::{EndNode, LevelParent},
        resource::{Level, MAP_TEXT},
    },
    prelude::*,
    screens::Screen,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (despawn_enemy_on_goal)
            .in_set(PausableSystems)
            .run_if(in_state(Screen::Gameplay)),
    );
}

/// A system that spawns the main level.
pub fn spawn_level(mut commands: Commands, mut level: ResMut<Level>) {
    commands.insert_resource(ClearColor(tailwind::SLATE_700.into()));

    *level = Level::from_str(MAP_TEXT);
    commands.compose(
        LevelParent::from_data(&level)
            + name("Level Parent")
            + StateScoped(Screen::Gameplay).store(),
    );
}

pub fn despawn_enemy_on_goal(
    mut commands: Commands,
    mut enemies: Query<(Entity, &Transform), With<EnemyHealth>>,
    goal: Query<&Transform, With<EndNode>>,
    mut game_state: ResMut<PlayerState>,
) {
    if let Ok(goal_pos) = goal.single() {
        let goal_pos = goal_pos.translation.xy();
        for (e, pos) in enemies.iter() {
            if pos.translation.xy().distance(goal_pos) < 7. {
                commands.get_entity(e).unwrap().despawn_recursive();
                game_state.health -= 1;
                println!("Damage Taken!");
            }
        }
    }
}

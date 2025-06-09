use crate::data::PlayerState;
use crate::demo::enemy_health::BountyEarned;
use crate::gameplay::shared_systems::Lifetime;
use crate::gameplay::wave_manager::WaveManager;
use crate::level::resource::{CurrentLoadedLevel, LevelSelect};
use crate::prelude::*;
use crate::theme::handles::LABEL_FONT;
use crate::theme::widget;
use bevy::color::palettes::tailwind;
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(Screen::Gameplay),
        (on_enter_game, update_hud, update_wave_tracker).chain(),
    );
    app.add_systems(Update, update_hud.run_if(resource_changed::<PlayerState>));
    app.add_systems(
        Update,
        update_wave_tracker.run_if(resource_changed::<WaveManager>),
    );
    app.add_systems(Update, animate_bounty_text);
    app.add_observer(bounty_earned);
}

#[derive(Component)]
enum HudElement {
    LevelName,
    Health,
    Money,
    BountyEarned,
}

fn on_enter_game(mut commands: Commands) {
    commands.spawn((
        StateScoped(Screen::Gameplay),
        BackgroundColor(tailwind::INDIGO_300.into()),
        BorderColor(tailwind::INDIGO_100.into()),
        Node {
            width: Val::Percent(100.0),
            display: Display::Flex,
            justify_content: JustifyContent::SpaceBetween,
            padding: UiRect::axes(Val::Px(24.0), Val::Px(8.0)),
            border: UiRect::bottom(Val::Px(4.0)),
            ..default()
        },
        children![
            (
                Node { ..default() },
                children![(widget::ui_font("LEVEL"), HudElement::LevelName)]
            ),
            (
                Node {
                    column_gap: Val::Px(32.0),
                    display: Display::Flex,
                    ..default()
                },
                children![
                    (widget::ui_font("Lives: _"), HudElement::Health),
                    (widget::ui_font("Money: _"), HudElement::Money),
                ]
            ),
        ],
    ));
}

fn update_hud(player_state: Res<PlayerState>, mut hud_elements: Query<(&mut Text, &HudElement)>) {
    for (mut text, element) in hud_elements.iter_mut() {
        match element {
            HudElement::Health => {
                text.0 = format!("Lives: {}", player_state.health);
            }
            HudElement::Money => {
                text.0 = format!("Money: {}", player_state.money);
            }
            _ => {}
        }
    }
}

fn update_wave_tracker(
    wave_manager: Res<WaveManager>,
    mut hud_elements: Query<(&mut Text, &HudElement)>,
    loaded_level: Res<CurrentLoadedLevel>,
) {
    for (mut text, element) in hud_elements.iter_mut() {
        match element {
            HudElement::LevelName => {
                let remaining_waves = wave_manager.remaining_waves();
                text.0 = format!(
                    "LEVEL {} - {remaining_waves} waves remain",
                    loaded_level.0 + 1
                );
            }
            _ => {}
        }
    }
}

fn bounty_earned(trigger: Trigger<BountyEarned>, mut commands: Commands) {
    let earned = trigger.1;

    commands.spawn((
        Lifetime::new(2.0),
        HudElement::BountyEarned,
        Node {
            top: Val::Px(32.0),
            right: Val::Px(50.0),
            position_type: PositionType::Absolute,
            ..default()
        },
        Name::new("Bounty Earned"),
        Text(format!("Bounty {earned}")),
        TextFont::from_font(LABEL_FONT).with_font_size(18.0),
        TextColor(tailwind::INDIGO_300.into()),
    ));
}

fn animate_bounty_text(mut query: Query<(&HudElement, &mut Node)>, time: Res<Time>) {
    for (entity, mut node) in query.iter_mut() {
        match entity {
            HudElement::BountyEarned => {
                let Val::Px(curr) = node.top else {
                    continue;
                };
                let new_top = curr + time.delta_secs() * 25.0;
                node.top = Val::Px(new_top);
            }
            _ => {}
        }
    }
}

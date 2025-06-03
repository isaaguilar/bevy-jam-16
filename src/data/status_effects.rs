use bevy::color::palettes::css::*;
use bevy::prelude::*;

#[derive(Component, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum StatusEffect {
    Wet,
    Burning,
    Frozen,
    Electrified,
    Acidic,
    Oiled,
    Slowed,
    Pushed,
}

impl StatusEffect {
    pub fn name(&self) -> &'static str {
        match self {
            StatusEffect::Wet => "Wet",
            StatusEffect::Burning => "Burning",
            StatusEffect::Frozen => "Frozen",
            StatusEffect::Electrified => "Electrified",
            StatusEffect::Acidic => "Acidic",
            StatusEffect::Oiled => "Oiled",
            StatusEffect::Slowed => "Slowed",
            StatusEffect::Pushed => "Pushed",
        }
    }

    pub fn color(&self) -> Color {
        match self {
            StatusEffect::Wet => BLUE.into(),
            StatusEffect::Burning => RED.into(),
            StatusEffect::Frozen => AQUA.into(),
            StatusEffect::Electrified => YELLOW.into(),
            StatusEffect::Acidic => GREEN.into(),
            StatusEffect::Oiled => BROWN.into(),
            StatusEffect::Slowed => PURPLE.into(),
            StatusEffect::Pushed => ORANGE.into(),
        }
    }

    pub fn ui_text(&self) -> impl Bundle {
        (Text::new(self.name()), TextColor(self.color()))
    }
}

#[derive(Component, Default, Clone, PartialEq, Debug)]
pub struct Ailments {
    pub slowdown: f32,
    pub min_damage: f32,
    pub max_damage: f32,
    pub damage_timer: Timer,
    pub ailment_timer: Timer,
}

impl Ailments {
    fn new(
        slowdown: f32,
        min_damage: f32,
        max_damage: f32,
        ailment_time_s: f32,
        damage_time_s: f32,
    ) -> Self {
        Self {
            slowdown,
            min_damage,
            max_damage,
            ailment_timer: Timer::from_seconds(ailment_time_s, TimerMode::Once),
            damage_timer: Timer::from_seconds(damage_time_s, TimerMode::Repeating),
        }
    }
}

pub fn add_status_effect(effect_type: &str) -> Option<impl Bundle> {
    let effect = match effect_type {
        "Wet" => (StatusEffect::Wet, (Ailments::new(0.0, 0.0, 0.0, 8., 8.))),
        "Burning" => (
            StatusEffect::Burning,
            (Ailments::new(0.0, 0.045, 0.110, 7., 1.5)),
        ),
        "Frozen" => (
            StatusEffect::Frozen,
            (Ailments::new(1.0, 0.023, 0.185, 8., 2.)),
        ),
        "Electrified" => (
            StatusEffect::Electrified,
            (Ailments::new(0.0, 0.026, 0.082, 12., 1.)),
        ),
        "Acidic" => (
            StatusEffect::Acidic,
            (Ailments::new(0.0, 0.100, 0.101, 9., 1.5)),
        ),
        "Oiled" => (
            StatusEffect::Oiled,
            (Ailments::new(0.0, 0.0, 0.0, 15., 15.)),
        ),
        "Slowed" => (
            StatusEffect::Slowed,
            (Ailments::new(0.7, 0.0, 0.0, 10., 10.)),
        ),
        "Pushed" => (
            StatusEffect::Pushed,
            (Ailments::new(1.0, 0.0, 0.0, 0.25, 0.25)),
        ),
        _ => {
            warn!("{effect_type} not a vaild status effect type");
            return None;
        }
    };

    Some(effect)
}

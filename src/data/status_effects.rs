use bevy::color::palettes::css::*;
use bevy::prelude::*;

#[derive(Component, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum StatusEffect {
    Wet,
    Ignited,
    Burned,
    Chilled,
    Frozen,
    Electrified,
    Acidic,
    Oiled,
    Slowed,
}

impl StatusEffect {
    pub fn name(&self) -> &'static str {
        match self {
            StatusEffect::Wet => "Wet",
            StatusEffect::Ignited => "Ignited",
            StatusEffect::Burned => "Burned",
            StatusEffect::Chilled => "Chilled",
            StatusEffect::Frozen => "Frozen",
            StatusEffect::Electrified => "Electrified",
            StatusEffect::Acidic => "Acidic",
            StatusEffect::Oiled => "Oiled",
            StatusEffect::Slowed => "Slowed",
        }
    }

    pub fn color(&self) -> Color {
        match self {
            StatusEffect::Wet => BLUE.into(),
            StatusEffect::Ignited => RED.into(),
            StatusEffect::Burned => ORANGE.into(),
            StatusEffect::Chilled => AQUA.into(),
            StatusEffect::Frozen => AQUA.into(),
            StatusEffect::Electrified => YELLOW.into(),
            StatusEffect::Acidic => GREEN.into(),
            StatusEffect::Oiled => BROWN.into(),
            StatusEffect::Slowed => PURPLE.into(),
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

pub fn get_ailment(status_effect: StatusEffect) -> Ailments {
    match status_effect {
        StatusEffect::Wet => (Ailments::new(0.0, 0.0, 0.0, 8., 8.)),
        StatusEffect::Burned => Ailments::new(0.0, 0.0, 0.0, 2., 2.),
        StatusEffect::Ignited => Ailments::new(0.0, 0.045, 0.110, 7., 1.5),
        StatusEffect::Chilled => Ailments::new(1.0, 0., 0.05, 5., 2.),
        StatusEffect::Frozen => Ailments::new(1.0, 0.023, 0.185, 8., 2.),
        StatusEffect::Electrified => Ailments::new(0.0, 0.026, 0.082, 12., 1.),
        StatusEffect::Acidic => Ailments::new(0.0, 0.100, 0.101, 9., 1.5),
        StatusEffect::Oiled => Ailments::new(0.0, 0.0, 0.0, 15., 15.),
        StatusEffect::Slowed => Ailments::new(0.7, 0.0, 0.0, 10., 10.),
    }
}

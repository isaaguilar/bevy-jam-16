use crate::define_stat;
use bevy::{
    app::App,
    ecs::{component::Component, schedule::SystemSet},
    reflect::{GetTypeRegistration, Reflect, TypePath},
};
use std::marker::PhantomData;

pub trait StatTrait:
    'static + Send + Sync + Reflect + Copy + Sized + TypePath + GetTypeRegistration
{
    fn name() -> &'static str;
}

#[derive(Component, PartialEq, Debug, Reflect, Clone)]
pub struct Stat<T: StatTrait> {
    base: f32,
    pre_flat: Vec<f32>,
    multipliers: Vec<f32>,
    post_flat: Vec<f32>,
    cached_value: f32,
    changed: bool,
    #[reflect(ignore)]
    _phantom: PhantomData<T>,
}

impl<T: StatTrait> Stat<T> {
    pub fn new(base: f32) -> Stat<T> {
        Stat {
            base,
            pre_flat: Vec::new(),
            multipliers: Vec::new(),
            post_flat: Vec::new(),
            cached_value: base,
            changed: false,
            _phantom: PhantomData,
        }
    }

    pub fn current_value(&self) -> f32 {
        self.cached_value
    }

    pub fn current_value_recalculate(&mut self) -> f32 {
        if self.changed {
            self.recalculate();
        }
        self.cached_value
    }

    pub fn premul_bonus(&mut self, val: f32) {
        self.changed = true;
        self.pre_flat.push(val);
    }

    pub fn multiplier(&mut self, val: f32) {
        self.changed = true;
        self.multipliers.push(val);
    }

    pub fn postmul_bonus(&mut self, val: f32) {
        self.changed = true;
        self.post_flat.push(val);
    }

    pub fn reset(&mut self) {
        self.cached_value = self.base;
        self.changed = false;
    }

    pub fn recalculate(&mut self) {
        if self.changed {
            self.cached_value = (self.base + self.pre_flat.iter().sum::<f32>())
                * self.multipliers.iter().fold(1., |a, b| a * b)
                + self.post_flat.iter().sum::<f32>();
            self.changed = false;
        }
    }
}

define_stat!(MoveSpeed, "Move Speed");
define_stat!(DamageMultiplier, "Damage Multiplier");

#[macro_export]
macro_rules! define_stat {
    ( $structname:ident, $name:expr) => {
        #[derive(Component, Copy, Clone, Eq, PartialEq, Hash, Debug, Reflect)]
        pub struct $structname;

        impl StatTrait for $structname {
            fn name() -> &'static str {
                $name
            }
        }
    };
}

pub fn duration_multiplier(strength: usize) -> f32 {
    match strength {
        0 => 0.,
        1 => 1.,
        2 => 1.67,
        3 => 2.5,
        4 => 3.,
        _ => 4.,
    }
}

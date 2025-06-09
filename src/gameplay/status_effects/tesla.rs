use bevy::{
    ecs::{
        entity::Entity,
        event::{EventReader, EventWriter},
        query::With,
        system::{Query, ResMut},
    },
    transform::components::Transform,
};
use bevy_turborand::{DelegatedRng, GlobalRng};

use super::common::{RemoveStatus, TryApplyStatus};
use crate::{
    data::{
        StatusEffect,
        projectiles::DamageType,
        status_effects::{Electrocuted, StatusEnum, Wet, damage_multiplier},
    },
    demo::enemy_health::{DoDamageToEnemy, EnemyHealth, TryDamageToEnemy},
};

pub fn electrocute_on_damage(
    mut damage_events: EventReader<TryDamageToEnemy>,
    mut status_events: EventWriter<TryApplyStatus>,
    wet_enemies: Query<&StatusEffect<Wet>>,
    mut rng: ResMut<GlobalRng>,
) {
    for TryDamageToEnemy {
        damage,
        damage_type,
        enemy,
        strength,
    } in damage_events.read()
    {
        if *damage_type == DamageType::Lightning {
            let wetness_boost = wet_enemies.get(*enemy).map(|w| w.strength).unwrap_or(0);
            let strength = strength + wetness_boost;
            let shock_chance = damage_multiplier(strength + wetness_boost) * 0.1;
            if rng.f32() < shock_chance {
                status_events.write(TryApplyStatus {
                    status: StatusEnum::Electrocuted,
                    enemy: *enemy,
                    strength: 1.max(strength - 1),
                });
            }
        }
    }
}

pub fn damage_after_electrocute(
    mut electro_events: EventReader<RemoveStatus<Electrocuted>>,
    mut damage_events: EventWriter<TryDamageToEnemy>,
    enemies: Query<(Entity, &Transform), With<EnemyHealth>>,
) {
    for RemoveStatus {
        enemy, strength, ..
    } in electro_events.read()
    {
        if let Ok((_, source_pos)) = enemies.get(*enemy) {
            let source_loc = source_pos.translation.clone();
            for (near_enemy, _) in enemies
                .iter()
                .filter(|(e, loc)| loc.translation.distance(source_loc) < 15. && e != enemy)
            {
                damage_events.write(TryDamageToEnemy {
                    damage: 10,
                    strength: *strength,
                    damage_type: DamageType::Lightning,
                    enemy: near_enemy,
                });
                println!("ZANG");
            }
        }
    }
}

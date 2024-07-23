use bevy::prelude::*;

#[derive(Event)]
pub struct DamageEvent {
    pub target: Entity,
    pub source: Entity,
    pub damage: u32
}

#[derive(Event)]
pub struct HealEvent {
    pub target: Entity,
    pub source: Entity,
    pub healing: u32
}

#[derive(Component)]
struct Health {
    hp: u32
}

#[derive(Component)]
struct MaxHealth {
    max_hp: u32
}

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<DamageEvent>()
            .add_systems(Update, (read_damage, read_heals));
    }
}

fn read_heals(
    mut heal_event: EventReader<HealEvent>,
    mut hp_query: Query<(&mut Health, Option<&MaxHealth>)>
) {
    for heal in heal_event.read() {
        let (mut health, max_health) = hp_query.get_mut(heal.target).unwrap();
        health.hp += heal.healing;
        if let Some(max_hp) = max_health {
            if health.hp > max_hp.max_hp {
                health.hp = max_hp.max_hp;
            }
        }
        
    }
}

fn read_damage(
    mut damage_event: EventReader<DamageEvent>,
    mut hp_query: Query<&mut Health>
) {
    for hit in damage_event.read() {
        let mut health = hp_query.get_mut(hit.target).unwrap();
        health.hp = health.hp.saturating_sub(hit.damage);
    }
}
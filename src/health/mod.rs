use bevy::{ecs::system::*, prelude::*};

#[derive(Component)]
pub struct Health {
    pub amount: f32,
    pub max_health: f32,
    pub destroy_on_death: bool,
}

impl Health {
    pub fn new(max_health: f32) -> Health {
        Health {
            amount: max_health,
            max_health,
            destroy_on_death: false,
        }
    }

    pub fn new_destroy_on_death(max_health: f32) -> Health {
        Health {
            amount: max_health,
            max_health,
            destroy_on_death: true,
        }
    }

    pub fn is_dead(&self) -> bool {
        self.amount <= 0.
    }
}

#[derive(Component)]
pub struct HealthHitInvincibilityTime {
    pub invincibility_time: f32,
    pub last_hit_time: Option<f32>,
}

impl HealthHitInvincibilityTime {
    pub fn new(invincibility_time: f32) -> HealthHitInvincibilityTime {
        HealthHitInvincibilityTime {
            invincibility_time,
            last_hit_time: None,
        }
    }
}

#[derive(Event)]
pub struct DamageTaken {
    pub entity: Entity,
    pub amount: f32,
    pub new_hp: f32,
}

impl DamageTaken {
    pub fn new(entity: Entity, amount: f32, new_hp: f32) -> DamageTaken {
        DamageTaken {
            entity,
            amount,
            new_hp,
        }
    }
}

#[derive(Event)]
pub struct Death {
    pub entity: Entity,
}

impl Death {
    pub fn new(entity: Entity) -> Death {
        Death { entity }
    }
}

#[derive(SystemParam)]
pub struct HealthManager<'w, 's> {
    #[doc(hidden)]
    time: Res<'w, Time>,
    #[doc(hidden)]
    healths: Query<
        'w,
        's,
        (
            &'static mut Health,
            Option<&'static mut HealthHitInvincibilityTime>,
        ),
    >,
    #[doc(hidden)]
    damage_writer: EventWriter<'w, DamageTaken>,
    #[doc(hidden)]
    death_writer: EventWriter<'w, Death>,
}

impl<'w, 's> HealthManager<'w, 's> {
    pub fn damage(&mut self, entity: Entity, amount: f32) -> Option<f32> {
        let Ok((mut health, maybe_invincibility)) = self.healths.get_mut(entity) else {
            panic!("Entity has no health component")
        };

        if health.amount <= 0. {
            return None;
        }
        if let Some(mut invincibility) = maybe_invincibility {
            if invincibility.last_hit_time.is_some_and(|last| {
                self.time.elapsed_secs() <= last + invincibility.invincibility_time
            }) {
                return None;
            } else {
                invincibility.last_hit_time = Some(self.time.elapsed_secs());
            }
        }
        health.amount -= amount;
        if health.is_dead() {
            self.death_writer.send(Death::new(entity));
        } else {
            self.damage_writer
                .send(DamageTaken::new(entity, amount, health.amount));
        }
        Some(amount)
    }
}

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DamageTaken>()
            .add_event::<Death>()
            .add_systems(Update, handle_death_events);
    }
}

fn handle_death_events(
    mut commands: Commands,
    health_q: Query<&Health>,
    mut deaths_e: EventReader<Death>,
) {
    for death in deaths_e.read() {
        let Ok(health) = health_q.get(death.entity) else {
            continue;
        };
        if health.destroy_on_death {
            commands.entity(death.entity).try_despawn();
        }
    }
}

use crate::{
    model::BotConfig,
    util::{ext::RngExt, Timer},
};
use bevy::{ecs::component::Component, prelude::Entity};
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg32;
use std::time::Duration;

const REPEAT_SHOOT_CHANCE: f64 = 0.6;

#[derive(Component)]
pub struct Bot {
    pub config: BotConfig,
    pub enemy: Option<Entity>,
    pub teammates: Vec<Entity>,
    pub update_timer: Timer,
    pub voice_timer: Timer,
    pub rng: Pcg32,
    shooting_state: BotShootingState,
    shooting_timer: Timer,
}

impl Bot {
    pub fn new(config: &BotConfig, skill: f32, seed: u64) -> Self {
        let mut rng = Pcg32::seed_from_u64(seed);

        return Self {
            config: config.clone_with(skill, &mut rng),
            enemy: None,
            teammates: Vec::new(),
            update_timer: Timer::default(),
            voice_timer: Timer::default(),
            shooting_state: BotShootingState::Prepare,
            shooting_timer: Timer::default(),
            rng,
        };
    }

    pub fn get_shooting_state(
        &mut self,
        is_weapon_automatic: bool,
        time: Duration,
    ) -> BotShootingState {
        if self.shooting_timer.is_ready_and_enabled(time) {
            let next_state = match self.shooting_state {
                BotShootingState::Prepare => BotShootingState::Shoot,
                BotShootingState::Shoot => BotShootingState::Pause,
                BotShootingState::Pause => {
                    if self.rng.gen_bool(REPEAT_SHOOT_CHANCE) {
                        BotShootingState::Shoot
                    } else {
                        BotShootingState::Prepare
                    }
                }
            };

            self.set_shooting_state(next_state, is_weapon_automatic, time);
        }

        return self.shooting_state;
    }

    pub fn set_shooting_state(
        &mut self,
        state: BotShootingState,
        is_weapon_automatic: bool,
        time: Duration,
    ) {
        let duration = match state {
            BotShootingState::Prepare => self.config.shoot_prepare_duration,
            BotShootingState::Shoot => {
                if is_weapon_automatic {
                    self.config.shoot_burst_duration
                } else {
                    Duration::ZERO // longer time can result ActorAction::Attack changing multiple times
                }
            }
            BotShootingState::Pause => self.config.shoot_interval,
        };

        self.shooting_state = state;
        self.shooting_timer
            .set(time + self.rng.distort_duration(duration));
    }

    pub fn set_shooting_target(&mut self, has_target: bool, time: Duration) {
        let was_target = self.shooting_timer.is_enabled();

        match (was_target, has_target) {
            // target appeared
            (false, true) => {
                self.set_shooting_state(BotShootingState::Prepare, false, time);
            }
            // target disappeared
            (true, false) => {
                self.shooting_state = BotShootingState::Prepare;
                self.shooting_timer.disable();
            }
            _ => {}
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BotShootingState {
    Prepare,
    Shoot,
    Pause,
}

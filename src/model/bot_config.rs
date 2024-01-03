use crate::util::ext::RngExt;
use rand::Rng;
use std::time::Duration;

pub struct BotConfig {
    pub is_silly: bool,
    pub spread: f32,
    pub spread_force: f32,
    pub sprint_distance: f32,
    pub sprint_stamina: f32,
    pub shoot_distance_min: f32,
    pub shoot_distance_max: f32, // TODO: limit by weapon distance and view distance?
    pub angular_deviation: f32,
    pub shoot_prepare_duration: Duration,
    pub shoot_burst_duration: Duration,
    pub shoot_interval: Duration,
}

impl BotConfig {
    pub const HUMAN: &'static Self = &Self {
        is_silly: false,
        spread: 3.0,       // TODO: tweak
        spread_force: 0.4, // TODO: tweak
        sprint_distance: 8.0,
        sprint_stamina: 0.3,
        shoot_distance_min: 4.0,
        shoot_distance_max: 9.0,
        angular_deviation: 0.8, // TODO: tweak
        shoot_prepare_duration: Duration::from_millis(800),
        shoot_burst_duration: Duration::from_millis(400),
        shoot_interval: Duration::from_millis(300),
    };

    pub const ZOMBIE: &'static Self = &Self {
        is_silly: true,
        spread: 3.0,       // TODO: tweak
        spread_force: 0.4, // TODO: tweak
        sprint_distance: 8.0,
        sprint_stamina: 0.3,
        shoot_distance_min: 2.0, // TODO: tweak
        shoot_distance_max: 8.0,
        angular_deviation: 0.16, // TODO: tweak
        shoot_prepare_duration: Duration::from_millis(1500),
        shoot_burst_duration: Duration::from_millis(400),
        shoot_interval: Duration::from_millis(600),
    };

    pub fn clone_with<R: Rng>(&self, skill: f32, r: &mut R) -> Self {
        return Self {
            is_silly: self.is_silly,
            spread: r.distort(self.spread),
            spread_force: r.distort(self.spread_force),
            sprint_distance: r.distort(self.sprint_distance),
            sprint_stamina: r.distort(self.sprint_stamina),
            shoot_distance_min: r.distort(self.shoot_distance_min),
            shoot_distance_max: r.distort(self.shoot_distance_max) * skill,
            angular_deviation: self.angular_deviation / skill,
            shoot_prepare_duration: self.shoot_prepare_duration.div_f32(skill),
            shoot_burst_duration: self.shoot_burst_duration,
            shoot_interval: self.shoot_interval.div_f32(skill),
        };
    }
}

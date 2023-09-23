use crate::{
    model::{ActorActions, ActorActionsExt},
    util::ext::DurationExt,
};
use bevy::ecs::component::Component;
use std::{f32::consts::TAU, time::Duration};

#[derive(Component)]
pub struct Actor {
    pub config: &'static ActorConfig,
    pub skill: f32,
    pub stamina: f32,
    pub actions: ActorActions,
    pub look_at: Option<f32>,
    pub melee_next: Duration,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ActorKind {
    Human,
    Zombie,
}

impl ActorKind {
    pub const fn get_assets_path(&self) -> &'static str {
        return match self {
            Self::Human => "actors/human",
            Self::Zombie => "actors/zombie",
        };
    }
}

pub struct ActorConfig {
    pub kind: ActorKind,
    pub movement_velocity: f32,
    pub rotation_velocity: f32,
    pub sprint_factor: f32,
    pub stamina: Duration,
    pub resistance: f32,
    pub radius: f32,
    pub mass: f32,
    pub melee_damage: f32,
    pub melee_distance: f32,
    pub melee_distance_angular: f32,
    pub melee_interval: Duration,
    pub pain_threshold: f32,
}

impl Actor {
    pub const ARMS_LENGTH_1: f32 = 0.546875;
    pub const ARMS_LENGTH_2: f32 = 0.34375;

    pub const fn new(config: &'static ActorConfig, skill: f32) -> Self {
        return Self {
            config,
            skill,
            stamina: 1.0,
            actions: ActorActions::EMPTY,
            look_at: None,
            melee_next: Duration::ZERO,
        };
    }

    pub fn reset_actions(&mut self) {
        self.actions = ActorActions::EMPTY;
        self.look_at = None;
    }

    pub fn update_stamina(&mut self, delta: f32) {
        let mut change = self.config.stamina.delta(delta);

        if self.actions.is_moving() {
            if self.actions.is_sprinting() {
                // spend stamina while sprinting
                change = -change;
            } else {
                // slower stamina gain while just moving
                change *= self.stamina / 2.0;
            }
        }

        self.stamina = (self.stamina + change).clamp(0.0, 1.0);
    }
}

impl ActorConfig {
    const HUMAN_RESISTANCE: f32 = 9.0;

    pub const HUMAN: Self = Self {
        kind: ActorKind::Human,
        movement_velocity: 2.5,
        rotation_velocity: 3.5,
        sprint_factor: 2.0,
        stamina: Duration::from_secs(16),
        resistance: Self::HUMAN_RESISTANCE,
        radius: 0.25,
        mass: 85.0,
        melee_damage: Self::HUMAN_RESISTANCE / 16.0, // 16 hits to kill human
        melee_distance: 0.7,
        melee_distance_angular: TAU / 5.0,
        melee_interval: Duration::from_millis(600),
        pain_threshold: 0.02,
    };

    pub const ZOMBIE: Self = Self {
        kind: ActorKind::Zombie,
        movement_velocity: Self::HUMAN.movement_velocity * 0.4,
        rotation_velocity: Self::HUMAN.rotation_velocity * 0.4,
        sprint_factor: 1.8,
        stamina: Duration::from_secs(10),
        resistance: Self::HUMAN.resistance * 0.6,
        radius: 0.21,
        mass: 70.0,
        melee_damage: Self::HUMAN.resistance / 10.0, // 10 hits to kill human
        melee_distance: Self::HUMAN.melee_distance,
        melee_distance_angular: Self::HUMAN.melee_distance_angular,
        melee_interval: Self::HUMAN.melee_interval,
        pain_threshold: 0.08,
    };

    pub fn get_image_path(&self, mut suffix: u8) -> String {
        suffix = match self.kind {
            ActorKind::Human => {
                suffix.clamp(1, 2) // since player has only 1 and 2
            }
            ActorKind::Zombie => {
                0 // since zombie only has 0
            }
        };

        return format!("{}/image_{}.png", self.kind.get_assets_path(), suffix);
    }
}

#[derive(Component)]
pub struct ActorWeaponSprite;

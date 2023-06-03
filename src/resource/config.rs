use anyhow::{Context, Result};
use bevy::{
    ecs::system::Resource,
    window::{PresentMode, WindowMode},
};
use serde::Deserialize;

// TODO: logging config

#[derive(Clone, Deserialize, Debug, Resource)]
pub struct Config {
    pub display: DisplayConfig,
    pub audio: AudioConfig,
    pub controls: ControlsConfig,
    pub misc: MiscConfig,
}

#[derive(Clone, Deserialize, Debug)]
pub struct DisplayConfig {
    pub fullscreen: bool,
    pub window_size_x: f32,
    pub window_size_y: f32,
    pub v_sync: bool,
}

#[derive(Clone, Deserialize, Debug)]
pub struct AudioConfig {
    pub sources: usize,
}

#[derive(Clone, Deserialize, Debug)]
pub struct ControlsConfig {
    pub mouse_sensitivity: f32,
}

#[derive(Clone, Deserialize, Debug)]
pub struct MiscConfig {
    pub laser_sight: bool,
    pub grab_cursor: bool,
    pub debug: bool,
}

impl Config {
    // TODO: simplify
    pub fn load_from(path: &str) -> Result<Self> {
        let global_context = || format!("Path: {}", path);

        let config = config::Config::builder()
            .add_source(config::File::with_name(path))
            .build()
            .context("Failed to load config from file")
            .with_context(global_context)?
            .try_deserialize::<Self>()
            .context("Failed to deserialize config")
            .with_context(global_context)?;

        return Ok(config);
    }
}

impl DisplayConfig {
    pub fn mode(&self) -> WindowMode {
        if self.fullscreen {
            return WindowMode::Fullscreen;
        } else {
            return WindowMode::Windowed;
        }
    }

    pub fn present_mode(&self) -> PresentMode {
        if self.v_sync {
            return PresentMode::AutoVsync;
        } else {
            return PresentMode::AutoNoVsync;
        }
    }
}

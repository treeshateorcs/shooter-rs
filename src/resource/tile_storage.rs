use crate::{
    data::{PIXELS_PER_METER, PIXELS_PER_METER_U8},
    util::math::{trunc_by},
};
use bevy::prelude::{Handle, Image, Resource, Vec2};

// https://discord.com/channels/691052431525675048/1115635786751164561/1115635786751164561
// https://discord.com/channels/691052431525675048/1104047542855024761/1104047542855024761

#[derive(Resource)]
pub struct TileStorage {
    // TODO: check inner size
    // TODO: maybe use fixed size vec
    images: Vec<Option<Handle<Image>>>,
}

impl TileStorage {
    pub const BLOCK_SIZE_HALF: u16 = 4;
    pub const BLOCK_SIZE: u16 = Self::BLOCK_SIZE_HALF * 2;
    pub const BLOCK_SIZE_PX: u16 = Self::BLOCK_SIZE * PIXELS_PER_METER_U8 as u16;

    pub const CHUNK_CAPACITY_SQUARED: u16 = 16;
    pub const CHUNK_CAPACITY: u16 = Self::CHUNK_CAPACITY_SQUARED * Self::CHUNK_CAPACITY_SQUARED;
    pub const CHUNK_SIZE: u16 = Self::BLOCK_SIZE * Self::CHUNK_CAPACITY_SQUARED;
    pub const CHUNK_SIZE_PX: u16 = Self::CHUNK_SIZE * PIXELS_PER_METER_U8 as u16;

    pub fn new() -> Self {
        return Self {
            images: Vec::with_capacity(usize::from(Self::CHUNK_CAPACITY)),
        };
    }

    pub fn pixel_position(position: Vec2) -> Vec2 {
        return position % f32::from(Self::BLOCK_SIZE) * PIXELS_PER_METER;
    }

    pub fn normalize_position(mut position: Vec2) -> Vec2 {
        let size = f32::from(Self::BLOCK_SIZE);
        position.x = trunc_by(position.x, size);
        position.y = trunc_by(position.y, size);
        return position;
    }

    // TODO: always return something
    // TODO: shift
    fn position_to_index(position: Vec2) -> Option<usize> {
        if position.x < 0.0 || position.x >= f32::from(Self::CHUNK_SIZE) {
            return None;
        }

        if position.y < 0.0 || position.y >= f32::from(Self::CHUNK_SIZE) {
            return None;
        }

        let size = f32::from(Self::BLOCK_SIZE);
        let x = (position.x / size).floor() as usize;
        let y = (position.y / size).floor() as usize * usize::from(Self::BLOCK_SIZE);
        return Some(x + y);
    }

    // TODO: test
    fn position_to_index_v2(position: Vec2) -> Option<usize> {
        let chunk_size_half = f32::from(Self::CHUNK_SIZE) / 2.0;

        if position.x <= -chunk_size_half || chunk_size_half <= position.x {
            return None;
        }

        if position.y <= -chunk_size_half || chunk_size_half <= position.y {
            return None;
        }

        let size = f32::from(Self::BLOCK_SIZE);
        let x = ((position.x + chunk_size_half) / size).trunc() as usize;
        let y = ((position.y + chunk_size_half) / size).trunc() as usize * usize::from(Self::CHUNK_CAPACITY_SQUARED);
        return Some(x + y);
    }

    pub fn get_tile(&self, position: Vec2) -> Option<Handle<Image>> {
        return Self::position_to_index_v2(position)
            .and_then(|i| self.images.get(i))
            .and_then(|i| i.as_ref())
            .cloned();
    }

    pub fn set_tile(&mut self, image: Handle<Image>, position: Vec2) {
        if let Some(i) = Self::position_to_index_v2(position) {
            if i < self.images.len() {
                if cfg!(debug_assertions) && self.images[i].is_some() {
                    panic!("Tile at index {} is already set", i);
                }

                self.images[i] = Some(image);
            } else {
                for _ in 0..(i - self.images.len()) {
                    self.images.push(None);
                }

                self.images.push(Some(image));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::f32::consts::E;
    use std::fmt::Debug;
    use super::*;

    const SIZE: Vec2 = Vec2::new(7.99, 7.99);
    const BLOCK_SIZE_HALF: f32 = TileStorage::BLOCK_SIZE_HALF as f32;

    fn v(x: f32, y: f32) -> Vec2 {
        return Vec2::new(x, y);
    }

    #[test]
    fn test_constants() {
        assert_eq!(TileStorage::BLOCK_SIZE, 8);
        assert_eq!(TileStorage::BLOCK_SIZE_PX, 256);
        assert_eq!(TileStorage::CHUNK_SIZE, 128);
        assert_eq!(TileStorage::CHUNK_SIZE_PX, 4096);
    }

    #[test]
    fn test_pixel_position() {
        assert_eq!(TileStorage::pixel_position(v(0.0, 0.0)), v(0.0, 0.0));
        assert_eq!(TileStorage::pixel_position(v(1.0, 0.0)), v(32.0, 0.0));
        assert_eq!(TileStorage::pixel_position(v(0.0, 1.0)), v(0.0, 32.0));
        assert_eq!(TileStorage::pixel_position(v(1.0, 1.0)), v(32.0, 32.0));

        assert_eq!(TileStorage::pixel_position(v(7.0, 7.0)), v(224.0, 224.0));
        assert_eq!(TileStorage::pixel_position(v(8.0, 7.0)), v(0.0, 224.0));
        assert_eq!(TileStorage::pixel_position(v(7.0, 8.0)), v(224.0, 0.0));
    }

    #[test]
    fn test_normalize_position() {
        let p = v(0.0, 0.0);
        assert_eq!(TileStorage::normalize_position(p), p, "Zero");
        assert_eq!(TileStorage::normalize_position(p + SIZE), p, "Zero");

        let p = v(8.0, 8.0);
        assert_eq!(TileStorage::normalize_position(p), p, "Positive");
        assert_eq!(TileStorage::normalize_position(p + SIZE), p, "Positive");

        let p = v(-7.99, -7.99);
        assert_eq!(TileStorage::normalize_position(p), Vec2::ZERO, "Negative");
        assert_eq!(TileStorage::normalize_position(p + SIZE), Vec2::ZERO, "Negative");

        let p = v(-15.99, -15.99);
        assert_eq!(TileStorage::normalize_position(p), v(-8.0, -8.0), "More negative");
        assert_eq!(TileStorage::normalize_position(p + SIZE), v(-8.0, -8.0), "More negative");
    }

    #[test]
    fn test_position_to_index() {
        let p = Vec2::new(0.0, 0.0);
        assert_eq!(TileStorage::position_to_index(p), Some(0), "First");
        assert_eq!(TileStorage::position_to_index(p + SIZE), Some(0), "First");

        let p = Vec2::new(8.0, 0.0);
        assert_eq!(TileStorage::position_to_index(p), Some(1), "Second by X");
        assert_eq!(
            TileStorage::position_to_index(p + SIZE),
            Some(1),
            "Second by X"
        );

        let p = Vec2::new(8.0, 8.0);
        assert_eq!(
            TileStorage::position_to_index(p),
            Some(9),
            "Second by X and Y"
        );
        assert_eq!(
            TileStorage::position_to_index(p + SIZE),
            Some(9),
            "Second by X and Y"
        );

        let p = Vec2::new(56.0, 56.0);
        assert_eq!(TileStorage::position_to_index(p), Some(63), "Last");
        assert_eq!(TileStorage::position_to_index(p + SIZE), Some(63), "Last");

        let p = Vec2::new(128.0, 128.0);
        assert_eq!(TileStorage::position_to_index(p), None, "Outside");
        assert_eq!(TileStorage::position_to_index(p + SIZE), None, "Outside");

        let p = Vec2::new(-0.01, -0.01);
        assert_eq!(TileStorage::position_to_index(p), None, "Outside");
        assert_eq!(
            TileStorage::position_to_index(p + SIZE),
            Some(0),
            "Now inside (first)"
        );
    }

    // macro_rules! assert_position {
    //     ($center:expr, $expected:expr) => {
    //         assert_eq!(TileStorage::position_to_index_v2($center), $expected);
    //     };
    // }

    fn assert_position_fn<T: Debug + PartialEq + Copy, F: Fn(Vec2) -> T>(f: &F, p: Vec2, expected: T) {
        assert_eq!(f(p), expected);
    }

    // TODO: use
    fn assert_position_fn_with_shifts<T: Debug + PartialEq + Copy, F: Fn(Vec2) -> T>(f: &F, p: Vec2, expected: T) {
        let shift = BLOCK_SIZE_HALF - 0.001;
        assert_position_fn(f, p, expected);
        assert_position_fn(f, p + v(-shift, -shift), expected);
        assert_position_fn(f, p + v(-shift,  shift), expected);
        assert_position_fn(f, p + v( shift,  shift), expected);
        assert_position_fn(f, p + v( shift, -shift), expected);
    }

    #[test]
    fn test_position_to_index_v2() {
        for x in [-64.0, 0.0, 64.0] {
            for y in [-64.0, 0.0, 64.0] {
                if x == 0.0 && y == 0.0 {
                    continue;
                }

                assert_eq!(
                    TileStorage::position_to_index_v2(Vec2::new(x, y)),
                    None,
                    "Outside ({}, {})",
                    x,
                    y,
                );
            }
        }

        // first
        let p = Vec2::new(-64.0 + BLOCK_SIZE_HALF, -64.0 + BLOCK_SIZE_HALF);
        assert_position_fn_with_shifts(&TileStorage::position_to_index_v2, p, Some(0));

        // second by x

        // last by x
        let p = Vec2::new(64.0 - BLOCK_SIZE_HALF, -64.0 + BLOCK_SIZE_HALF);
        assert_position_fn_with_shifts(&TileStorage::position_to_index_v2, p, Some(15));

        let p = Vec2::new(64.0 - BLOCK_SIZE_HALF, 64.0 - BLOCK_SIZE_HALF);
        assert_position_fn_with_shifts(&TileStorage::position_to_index_v2, p, Some(255));
    }
}

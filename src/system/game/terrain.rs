use crate::{component::Terrain, util::math::round_by};
use bevy::{
    ecs::system::Query,
    prelude::{Camera, Transform, With, Without},
};

pub fn terrain(
    cameras: Query<&Transform, With<Camera>>,
    mut terrains: Query<&mut Transform, (With<Terrain>, Without<Camera>)>,
) {
    let count = Terrain::get_count();
    let shift = count * Terrain::SIZE_HALF - Terrain::SIZE_HALF;
    let mut x = 0;
    let mut y = 0;

    if let Some(camera) = cameras.iter().next() {
        let size = Terrain::SIZE as f32;
        x = round_by(camera.translation.x, size) as i32 - shift;
        y = round_by(camera.translation.y, size) as i32 - shift;
    }

    for (i, mut terrain) in terrains.iter_mut().enumerate() {
        let i = i as i32;
        terrain.translation.x = (i % count * Terrain::SIZE + x) as f32;
        terrain.translation.y = (i / count * Terrain::SIZE + y) as f32;
    }
}

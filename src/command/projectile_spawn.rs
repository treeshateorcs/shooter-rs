use crate::{
    component::{Projectile, ProjectileConfig},
    data::LAYER_PROJECTILE,
    material::ProjectileMaterial,
    model::TransformLite,
    resource::Cache,
    util::ext::Vec2Ext,
};
use bevy::{
    asset::Assets,
    ecs::system::Command,
    math::Vec3,
    prelude::{Entity, Time, Transform, Vec2, World},
    sprite::MaterialMesh2dBundle,
};

pub struct ProjectileSpawn {
    pub config: &'static ProjectileConfig,
    pub transform: TransformLite,
    pub velocity: f32,
    pub shooter: Option<Entity>,
}

impl Command for ProjectileSpawn {
    fn apply(self, world: &mut World) {
        let cache = world.resource::<Cache>();

        let Some(image) = cache.dummy_image.clone() else {
            log::warn!("Failed to spawn a projectile. The dummy image isn't initialized");
            return;
        };

        let Some(mesh) = cache.dummy_mesh.clone() else {
            log::warn!("Failed to spawn a projectile. The dummy mesh isn't initialized");
            return;
        };

        let projectile = Projectile::new(
            self.config,
            world.resource::<Time>().elapsed(),
            self.transform.translation,
            Vec2::from_length(self.velocity, self.transform.direction),
            self.shooter,
        );

        let material = world
            .resource_mut::<Assets<ProjectileMaterial>>()
            .add(ProjectileMaterial { image });

        world
            .spawn(MaterialMesh2dBundle {
                transform: Transform {
                    translation: self.transform.translation.extend(LAYER_PROJECTILE),
                    scale: Vec3::new(0.0, 0.0, 1.0),
                    ..Transform::default()
                },
                mesh: mesh.into(),
                material,
                ..Default::default()
            })
            .insert(projectile);
    }
}

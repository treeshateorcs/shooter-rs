use crate::{
    component::{Actor, ActorConfig, ActorKind, Breath, Collision, Footsteps, Health, Inertia},
    data::LAYER_ACTOR,
    model::TransformLite,
};
use bevy::{
    ecs::system::Command,
    prelude::{AssetServer, Entity, SpriteBundle, World},
};

pub struct ActorSet {
    pub entity: Entity,
    pub config: &'static ActorConfig,
    pub skill: f32,
    pub transform: TransformLite,
}

impl Command for ActorSet {
    fn apply(self, world: &mut World) {
        let texture_path = self.config.get_image_path(0);
        let texture = world
            .resource::<AssetServer>()
            .get_handle(texture_path)
            .unwrap_or_default();

        let mut entity_mut = world.entity_mut(self.entity);

        entity_mut
            .insert(SpriteBundle {
                transform: self.transform.as_transform(LAYER_ACTOR),
                texture,
                ..Default::default()
            })
            .insert(Collision {
                radius: self.config.radius,
            })
            .insert(Inertia::new(self.config.mass))
            .insert(Actor::new(self.config, self.skill))
            .insert(Health::new(self.config.health * self.skill))
            .insert(Footsteps::default());

        if let ActorKind::Human = self.config.kind {
            entity_mut.insert(Breath::default());
        }
    }
}

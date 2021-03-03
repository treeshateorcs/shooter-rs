use crate::components::Actor;
use crate::components::Interpolation;
use crate::resources::EntityMap;
use crate::resources::Message;
use crate::resources::MessageReceiver;
use crate::resources::MessageResource;
use amethyst::core::transform::Transform;
use amethyst::derive::SystemDesc;
use amethyst::ecs::prelude::Join;
use amethyst::ecs::prelude::ReadStorage;
use amethyst::ecs::prelude::System;
use amethyst::ecs::prelude::SystemData;
use amethyst::ecs::Entities;
use amethyst::ecs::ReadExpect;
use amethyst::ecs::Write;
use std::collections::HashMap;
use std::f32::consts::TAU;
use std::time::Duration;
use std::time::Instant;

#[allow(clippy::integer_division)]
pub const INTERVAL: Duration = Duration::from_millis(1000 / 25);

#[derive(SystemDesc)]
pub struct TransformSyncSystem {
    last_sync: Instant,
    cache: HashMap<u16, Cached>,
}

#[derive(PartialEq)]
struct Cached {
    x: f32,
    y: f32,
    direction: f32,
}

impl TransformSyncSystem {
    pub fn new() -> Self {
        return Self {
            last_sync: Instant::now(),
            cache: HashMap::new(),
        };
    }
}

impl<'a> System<'a> for TransformSyncSystem {
    type SystemData = (
        Entities<'a>,
        ReadExpect<'a, EntityMap>,
        ReadStorage<'a, Actor>,
        ReadStorage<'a, Interpolation>,
        ReadStorage<'a, Transform>,
        Write<'a, MessageResource>,
    );

    fn run(
        &mut self,
        (entities, entity_map, actors, interpolations, transforms, mut messages): Self::SystemData,
    ) {
        if self.last_sync.elapsed() < INTERVAL {
            return;
        }

        let mut clean_cache = HashMap::with_capacity(self.cache.capacity());
        let query = (&entities, &actors, (&interpolations).maybe(), &transforms).join();

        for (entity, _, interpolation, transform) in query {
            if let Some(external_id) = entity_map.get_external_id(entity) {
                let mut current = Cached {
                    x: transform.translation().x,
                    y: transform.translation().y,
                    direction: transform.euler_angles().2,
                };

                if let Some(interpolation) = interpolation {
                    current.x += interpolation.offset_x;
                    current.y += interpolation.offset_y;
                    current.direction = (current.direction - interpolation.offset_direction) % TAU;
                }

                if self.cache.get(&external_id).map_or(true, |c| c != &current) {
                    messages.push((
                        MessageReceiver::Every,
                        Message::TransformSync {
                            id: 0,
                            external_id,
                            x: current.x,
                            y: current.y,
                            direction: current.direction,
                        },
                    ));

                    clean_cache.insert(external_id, current);
                }
            }
        }

        std::mem::swap(&mut self.cache, &mut clean_cache);
        self.last_sync = Instant::now();
    }
}

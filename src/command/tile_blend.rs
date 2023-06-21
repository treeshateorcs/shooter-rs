use crate::{
    data::{LAYER_BLUFF, TRANSFORM_SCALE},
    resource::TileStorage,
    util::{create_blank_image, ext::ImageExt},
};
use bevy::{
    ecs::system::Command,
    prelude::{AssetServer, Assets, Handle, Image, SpriteBundle, Transform, Vec2, World, Vec3},
    sprite::{Anchor, Sprite},
};
use derive_more::Constructor;

#[derive(Constructor)]
pub struct TileBlend {
    pub position: Vec2,
}

impl TileBlend {
    fn get_source(&self, world: &mut World, handle: &Handle<Image>) -> Option<ImageSimple> {
        return world.resource::<Assets<Image>>().get(handle).map(|image| {
            ImageSimple::new(
                image.data.clone(),
                image.texture_descriptor.size.width,
                image.texture_descriptor.size.height,
            )
        });
    }

    fn get_target(&self, world: &mut World) -> Handle<Image> {
        if let Some(handle) = world.resource::<TileStorage>().get_tile(self.position) {
            return handle;
        }

        let handle = create_blank_image(
            u32::from(TileStorage::BLOCK_SIZE_PX),
            u32::from(TileStorage::BLOCK_SIZE_PX),
            &mut world.resource_mut::<Assets<Image>>(),
        );

        if let Some(image) = world.resource_mut::<Assets<Image>>().get_mut(&handle) {
            image.add_border((255, 0, 0, 255));
        }

        world
            .resource_mut::<TileStorage>()
            .set_tile(handle.clone(), self.position);

        world.spawn(SpriteBundle {
            sprite: Sprite {
                anchor: Anchor::TopLeft,
                ..Default::default()
            },
            transform: Transform {
                translation: TileStorage::normalize_position(self.position).extend(LAYER_BLUFF),
                scale: TRANSFORM_SCALE,
                ..Default::default()
            },
            texture: handle.clone(),
            ..Default::default()
        });

        return handle;
    }
}

impl Command for TileBlend {
    fn write(self, world: &mut World) {
        let source_handle = world
            .resource::<AssetServer>()
            .get_handle("actors/zombie/image_0.png");

        let source = if let Some(source) = self.get_source(world, &source_handle) {
            source
        } else {
            // TODO: warn
            return;
        };

        let target_handle = self.get_target(world);

        if let Some(target) = world
            .resource_mut::<Assets<Image>>()
            .get_mut(&target_handle)
        {
            let mut px = TileStorage::pixel_position(self.position);
            // px.x -= source.size_x as f32 / 2.0;
            // px.y -= source.size_y as f32 / 2.0;

            // let mut px = Vec2::ZERO;
            // px.x -= source.size_x as f32 / 2.0;
            // px.y -= source.size_y as f32 / 2.0;

            source.for_each_pixel(|x, y, color| {
                let mut p = Vec2::new(x as f32, y as f32);
                p.x += px.x;
                p.y -= px.y; // TODO: fix this. it's strange that Y mus be negative
                target.set_pixel(p.x as u32, p.y as u32, color)
            });
        }
    }
}

#[derive(Constructor)]
struct ImageSimple {
    data: Vec<u8>,
    size_x: u32,
    size_y: u32,
}

impl ImageExt for ImageSimple {
    fn blank(_: u32, _: u32) -> Self {
        unimplemented!(); // it's ok
    }

    fn data(&self) -> &[u8] {
        return &self.data;
    }

    fn data_mut(&mut self) -> &mut [u8] {
        return &mut self.data;
    }

    fn size_x(&self) -> u32 {
        return self.size_x;
    }

    fn size_y(&self) -> u32 {
        return self.size_y;
    }
}

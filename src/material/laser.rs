use bevy::{
    asset::Asset,
    prelude::{Handle, Image},
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};

#[derive(Debug, Clone, Asset, TypePath, AsBindGroup)]
pub struct LaserMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub image: Handle<Image>,
}

impl Material2d for LaserMaterial {
    fn fragment_shader() -> ShaderRef {
        return "shader/laser.wgsl".into();
    }
}

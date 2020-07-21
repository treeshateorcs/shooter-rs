use crate::components::actor::Actor;
use crate::components::player::Player;
use crate::states::menu::home::Home;
use crate::utils;
use amethyst::assets::AssetStorage;
use amethyst::assets::Loader;
use amethyst::core::math::Point3;
use amethyst::core::math::Vector3;
use amethyst::core::shrev::EventChannel;
use amethyst::core::transform::Transform;
use amethyst::core::Parent;
use amethyst::ecs::prelude::World;
use amethyst::ecs::Entity;
use amethyst::input::is_key_down;
use amethyst::input::InputEvent;
use amethyst::prelude::*;
use amethyst::renderer::sprite::SpriteSheetHandle;
use amethyst::renderer::Camera;
use amethyst::renderer::ImageFormat;
use amethyst::renderer::SpriteRender;
use amethyst::renderer::SpriteSheet;
use amethyst::renderer::SpriteSheetFormat;
use amethyst::renderer::Texture;
use amethyst::tiles::MortonEncoder;
use amethyst::tiles::Tile;
use amethyst::tiles::TileMap;
use amethyst::winit::VirtualKeyCode;

const VIEWPORT: f32 = 150.0; // TODO: Do not hard-code

#[derive(Debug)]
pub enum GameEvent {
    GameStart,
    GameEnd,
}

pub struct Game {
    root: Option<Entity>,
}

impl Game {
    pub fn new() -> Self {
        return Self { root: None };
    }
}

impl SimpleState for Game {
    fn on_start(&mut self, mut data: StateData<GameData>) {
        utils::set_cursor_visibility(false, &mut data.world);

        let actor_renderer = SpriteRender {
            // TODO: Simplify sprite loading, avoid using sprite sheets
            sprite_sheet: load_sprite_sheet(
                data.world,
                "actors/human/image.png",
                "actors/human/image.ron",
            ),
            sprite_number: 0,
        };

        let root = data.world.create_entity().build();

        create_actor(data.world, 50.0, 0.0, false, actor_renderer.clone(), root);
        create_actor(data.world, -50.0, 0.0, false, actor_renderer.clone(), root);
        create_actor(data.world, 0.0, 50.0, false, actor_renderer.clone(), root);
        create_actor(data.world, 0.0, -50.0, false, actor_renderer.clone(), root);

        let actor_main = create_actor(data.world, 0.0, 0.0, true, actor_renderer, root);

        create_camera(data.world, actor_main);
        create_ground(data.world, root);

        utils::input::reset_mouse_delta();

        data.world
            .write_resource::<EventChannel<GameEvent>>()
            .single_write(GameEvent::GameStart);

        self.root = Some(root);
    }

    fn on_stop(&mut self, data: StateData<GameData>) {
        if let Some(root) = self.root.take() {
            // TODO: Do not panic, write warning to the log instead
            data.world
                .delete_entity(root)
                .expect("Failed to delete the root entity. Was it already removed?");
        }

        data.world
            .write_resource::<EventChannel<GameEvent>>()
            .single_write(GameEvent::GameEnd);
    }

    fn on_resume(&mut self, mut data: StateData<GameData>) {
        utils::set_cursor_visibility(false, &mut data.world);
    }

    fn handle_event(&mut self, _data: StateData<GameData>, event: StateEvent) -> SimpleTrans {
        match event {
            StateEvent::Window(event) => {
                if is_key_down(&event, VirtualKeyCode::Escape) {
                    return Trans::Push(Box::new(Home::new(false)));
                }
            }
            StateEvent::Input(event) => {
                if let InputEvent::MouseMoved { delta_x: delta, .. } = event {
                    #[allow(clippy::cast_possible_truncation)]
                    utils::input::add_mouse_delta(delta as i16);
                }
            }
            _ => {}
        }

        return Trans::None;
    }
}

#[derive(Default, Clone)]
pub struct GroundTile;

impl Tile for GroundTile {
    fn sprite(&self, _: Point3<u32>, _: &World) -> Option<usize> {
        return Some(1);
    }
}

fn create_actor(
    world: &mut World,
    x: f32,
    y: f32,
    is_player: bool,
    renderer: SpriteRender,
    root: Entity,
) -> Entity {
    let mut transform = Transform::default();
    transform.set_translation_xyz(x, y, 0.0);
    transform.set_rotation_2d(utils::math::PI_0_5);

    let mut actor = world
        .create_entity()
        .with(renderer)
        .with(Actor::new())
        .with(transform)
        .with(Parent { entity: root });

    if is_player {
        actor = actor.with(Player::new());
    }

    return actor.build();
}

fn create_camera(world: &mut World, player: Entity) -> Entity {
    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0, 0.0, 1.0);
    transform.set_rotation_2d(utils::math::PI_1_5);

    return world
        .create_entity()
        .with(Camera::standard_2d(VIEWPORT, VIEWPORT))
        .with(transform)
        .with(Parent { entity: player })
        .build();
}

fn create_ground(world: &mut World, root: Entity) -> Entity {
    let map = TileMap::<GroundTile, MortonEncoder>::new(
        Vector3::new(2, 2, 1),
        Vector3::new(128, 128, 1),
        Some(load_sprite_sheet(
            world,
            "ground/grass.png",
            "ground/grass.ron",
        )),
    );

    return world
        .create_entity()
        .with(map)
        .with(Transform::default())
        .with(Parent { entity: root })
        .build();
}

fn load_sprite_sheet(world: &mut World, png_path: &str, ron_path: &str) -> SpriteSheetHandle {
    return world.read_resource::<Loader>().load(
        ron_path,
        SpriteSheetFormat(world.read_resource::<Loader>().load(
            png_path,
            ImageFormat::default(),
            (),
            &world.read_resource::<AssetStorage<Texture>>(),
        )),
        (),
        &world.read_resource::<AssetStorage<SpriteSheet>>(),
    );
}

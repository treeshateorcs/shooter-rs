use amethyst::ecs::Component;
use amethyst::ecs::NullStorage;

#[derive(Default)]
pub struct Player;

impl Component for Player {
    type Storage = NullStorage<Self>;
}

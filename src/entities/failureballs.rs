use crate::component::graphics::Sprite;
use crate::component::physics::{Acceleration, Gravity, Position, Velocity};
use derive::Event;
use engine::entities::entity::{entity, Entities};
use engine::events::dispatcher::Dispatcher;
use engine::events::event::Events;

#[derive(Event)]
pub struct SpawnFailureBall {
    pub sprite: String,
    pub dx: f64,
    pub position: (f64, f64),
}

pub fn register(dispatcher: &mut Dispatcher) {
    dispatcher.register(spawn_failure_ball);
}

fn spawn_failure_ball(SpawnFailureBall { sprite, dx, position: (x, y) }: &SpawnFailureBall, world: &mut Entities, _events: &mut Events) {
    world.spawn(entity()
            .with(Gravity())
            .with(Sprite::sprite_from_string(sprite.clone(), 5, false))
            .with(Acceleration(0.0, 0.0))
            .with(Position(*x, *y))
            .with(Velocity(*dx, 0.0))
        );
}

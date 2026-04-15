use crate::component::collisions::{Collided, Pickup};
use crate::component::graphics::Sprite;
use crate::component::physics::Position;
use derive::{Constant, Event};
use engine::entities::entity::{entity, Entities};
use engine::events::dispatcher::Dispatcher;
use engine::events::event::Events;
use engine::events::spawner::Spawner;
use engine::shapes::shape::Shape;
use crate::component::lifecycle::Destroy;

#[derive(Event)]
pub struct SpawnKey(f64, f64);

#[derive(Constant, Clone)]
pub struct Key();

#[derive(Event)]
pub struct Unlock();

pub fn register(dispatcher: &mut Dispatcher, spawner: &mut Spawner) {
    dispatcher.register(spawn_key);
    dispatcher.register(pickup_key);

    spawner.register("Key", |spawn, events| {
        events.fire(SpawnKey(spawn.x, spawn.y))
    });
}

fn spawn_key(&SpawnKey(x, y): &SpawnKey, world: &mut Entities, _events: &mut Events) {
    world.spawn(
        entity()
            .with(Key())
            .with(Pickup())
            .with(Sprite::sprite("key", 5))
            .with(Position(x, y))
            .with(Shape::bbox(0.0, 0.0, 12.0, 12.0))
    );
}


fn pickup_key(Collided(first, second): &Collided, world: &mut Entities, events: &mut Events) {
    world.apply_to(first, |Key()| {
        events.fire(Unlock());
        events.fire(Destroy(*first));
    });
    world.apply_to(second, |Key()| {
        events.fire(Unlock());
        events.fire(Destroy(*second));
    });
}

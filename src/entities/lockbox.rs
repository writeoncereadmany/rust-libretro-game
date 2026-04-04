use crate::component::graphics::Sprite;
use crate::component::lifecycle::Destroy;
use crate::component::physics::Position;
use crate::entities::key::Unlock;
use crate::entities::map::CollisionType;
use derive::{Constant, Event};
use engine::entities::entity::{entity, Entities, EntityId, Id};
use engine::events::dispatcher::Dispatcher;
use engine::events::event::Events;
use engine::events::spawner::Spawner;
use engine::shapes::shape::Shape;

#[derive(Event)]
pub struct SpawnLockbox(f64, f64);

#[derive(Constant, Clone)]
pub struct Lockbox();

pub fn register(dispatcher: &mut Dispatcher, spawner: &mut Spawner) {
    dispatcher.register(spawn_lockbox);
    dispatcher.register(unlock);

    spawner.register("Lockbox", |spawn, events| {
        events.fire(SpawnLockbox(spawn.x, spawn.y))
    });
}

pub fn spawn_lockbox(&SpawnLockbox(x, y): &SpawnLockbox, world: &mut Entities, _events: &mut Events) {
    world.spawn(entity()
        .with(Lockbox())
        .with(Sprite::sprite("lockbox", 5))
        .with(Shape::bbox(0.0, 0.0, 12.0, 12.0))
        .with(CollisionType::WALL)
        .with(Position(x, y))
    );
}
pub fn unlock(_: &Unlock, world: &mut Entities, events: &mut Events) {
    world.apply(|(Lockbox(), Id(id)) | events.fire(Destroy(id)));
}
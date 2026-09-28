use crate::component::graphics::Sprite;
use crate::component::lifecycle::Destroy;
use crate::component::physics::Position;
use crate::entities::key::Unlock;
use crate::entities::map::CollisionType;
use derive::{Constant, Event, system, spawn};
use engine::entities::entity::{entity, Entities, Id};
use engine::events::event::Events;
use engine::events::spawner::Spawn;
use engine::shapes::shape::Shape;

#[derive(Event)]
pub struct SpawnLockbox(f64, f64);

#[derive(Constant, Clone)]
pub struct Lockbox();

#[spawn("Lockbox")]
fn spawn_lockbox_from_map(spawn: Spawn, events: &mut Events) {
    events.fire(SpawnLockbox(spawn.x, spawn.y));
}

#[system]
pub fn spawn_lockbox(&SpawnLockbox(x, y): &SpawnLockbox, world: &mut Entities, _events: &mut Events) {
    world.spawn(entity()
        .with(Lockbox())
        .with(Sprite::sprite("lockbox", 5))
        .with(Shape::bbox(0.0, 0.0, 12.0, 12.0))
        .with(CollisionType::WALL)
        .with(Position(x, y))
    );
}
#[system]
pub fn unlock(_: &Unlock, world: &mut Entities, events: &mut Events) {
    world.apply(|(Lockbox(), Id(id)) | events.fire(Destroy(id)));
}
use crate::component::graphics::Sprite;
use crate::component::lifecycle::Destroy;
use crate::component::physics::{Acceleration, Gravity, Position, Velocity};
use crate::entities::key::Unlock;
use derive::{Constant, Event};
use engine::entities::entity::{entity, Entities, EntityId, Id};
use engine::events::dispatcher::Dispatcher;
use engine::events::event::Events;
use engine::events::spawner::Spawner;
use engine::shapes::shape::Shape;
use crate::component::collisions::{Actor, Collided, Pickup};
use crate::entities::coin::{Coin, PickupCoin, SpawnSparkle};
use crate::game::game::Score;

#[derive(Event)]
pub struct SpawnChest(f64, f64);

#[derive(Event)]
pub struct SpawnOpenChest(f64, f64);

#[derive(Event)]
pub struct SpawnRuby(f64, f64);

#[derive(Event)]
pub struct PickupRuby(EntityId);

#[derive(Constant, Clone)]
pub struct Chest();

#[derive(Constant, Clone)]
pub struct Ruby();

pub fn register(dispatcher: &mut Dispatcher, spawner: &mut Spawner) {
    dispatcher.register(spawn_chest);
    dispatcher.register(spawn_open_chest);
    dispatcher.register(spawn_ruby);
    dispatcher.register(unlock);
    dispatcher.register(pickup_ruby);
    dispatcher.register(collect_ruby);

    spawner.register("Chest", |spawn, events| {
        events.fire(SpawnChest(spawn.x, spawn.y))
    });
}

pub fn spawn_chest(&SpawnChest(x, y): &SpawnChest, world: &mut Entities, _events: &mut Events) {
    world.spawn(entity()
        .with(Chest())
        .with(Sprite::sprite("chest_closed", 3))
        .with(Position(x, y))
    );
}

pub fn spawn_open_chest(&SpawnOpenChest(x, y): &SpawnOpenChest, world: &mut Entities, _events: &mut Events) {
    world.spawn(entity()
        .with(Sprite::sprite("chest_open", 3))
        .with(Position(x, y))
    );
}

pub fn spawn_ruby(&SpawnOpenChest(x, y): &SpawnOpenChest, world: &mut Entities, _events: &mut Events) {
    world.spawn(entity()
        .with(Ruby())
        .with(Sprite::sprite("ruby", 5))
        .with(Position(x, y))
        .with(Velocity(0.0, 300.0))
        .with(Acceleration(0.0, 0.0))
        .with(Shape::bbox(0.0, 0.0, 12.0, 12.0))
        .with(Gravity())
        .with(Actor())
        .with(Pickup())
    );
}

pub fn unlock(_: &Unlock, world: &mut Entities, events: &mut Events) {
    world.apply(|(Chest(), Position(x, y), Id(id)) | {
        events.fire(Destroy(id));
        events.fire(SpawnOpenChest(x, y));
        events.fire(SpawnRuby(x, y));
    });
}

fn pickup_ruby(Collided(first, second): &Collided, world: &mut Entities, events: &mut Events) {
    world.apply_to(first, |Ruby()| events.fire(PickupRuby(*first)));
    world.apply_to(second, |Ruby()| events.fire(PickupRuby(*second)));
}

fn collect_ruby(PickupRuby(ruby): &PickupRuby, world: &mut Entities, events: &mut Events) {
    if let Some(Position(x, y)) = world.delete(ruby) {
        events.fire(Score(100));
    }
}
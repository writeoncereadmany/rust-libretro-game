use std::time::Duration;
use crate::component::collisions::Collided;
use crate::component::graphics::{Animation, Sprite};
use crate::component::lifecycle::Destroy;
use crate::component::physics::Position;
use crate::entities::map::CollisionType;
use derive::{Constant, Event};
use engine::entities::entity::{entity, Entities, EntityId};
use engine::events::dispatcher::Dispatcher;
use engine::events::event::Events;
use engine::events::spawner::Spawner;
use engine::shapes::shape::Shape;
use crate::component::time::{Period, Phase};

const CRUMBLER_LIFESPAN: f64 = 0.5;

#[derive(Event)]
pub struct SpawnCrumbler(f64, f64);

#[derive(Event)]
pub struct SpawnCrumbling(f64, f64);

#[derive(Constant, Clone)]
pub struct Crumbler();

pub fn register(dispatcher: &mut Dispatcher, spawner: &mut Spawner) {
    dispatcher.register(spawn_crumbler);
    dispatcher.register(spawn_crumbling);
    dispatcher.register(crumble);

    spawner.register("Crumbler", |spawn, events| {
        events.fire(SpawnCrumbler(spawn.x, spawn.y))
    });
}

pub fn spawn_crumbler(&SpawnCrumbler(x, y): &SpawnCrumbler, world: &mut Entities, _events: &mut Events) {
    world.spawn(entity()
        .with(Crumbler())
        .with(Sprite::sprite("crumbler", 5))
        .with(Shape::bbox(0.0, 0.0, 12.0, 12.0))
        .with(CollisionType::WALL)
        .with(Position(x, y))
    );
}

pub fn spawn_crumbling(&SpawnCrumbling(x, y): &SpawnCrumbling, world: &mut Entities, events: &mut Events) {
    let crumbling_id = world.spawn(entity()
        .with(Sprite::sprite("crumbling_1", 5))
        .with(Shape::bbox(0.0, 0.0, 12.0, 12.0))
        .with(Animation{ sprites: vec!("crumbling_1", "crumbling_2", "crumbling_3"), layer: 5 })
        .with(Phase(0.0))
        .with(Period(CRUMBLER_LIFESPAN))
        .with(CollisionType::WALL)
        .with(Position(x, y))
    );
    events.schedule("Game", Duration::from_secs_f64(CRUMBLER_LIFESPAN), Destroy(crumbling_id));
}

pub fn crumble(Collided(entity_1, entity_2, _): &Collided, world: &mut Entities, events: &mut Events) {
    crumble_entity(entity_1, world, events);
    crumble_entity(entity_2, world, events);
}

pub fn crumble_entity(entity_id: &EntityId, world: &mut Entities, events: &mut Events) {
    world.apply_to(entity_id, |(Crumbler(), Position(x, y))| {
        events.fire(Destroy(*entity_id));
        events.fire(SpawnCrumbling(x, y));
    });
}
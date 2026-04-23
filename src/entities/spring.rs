use std::time::Duration;
use crate::component::collisions::{Collided, Interactable};
use crate::component::graphics::{Animation, Sprite};
use crate::component::physics::Position;
use crate::entities::hero::Hero;
use derive::{Constant, Event};
use engine::entities::entity::{entity, Entities, EntityId};
use engine::events::dispatcher::Dispatcher;
use engine::events::event::Events;
use engine::events::spawner::Spawner;
use engine::shapes::shape::Shape;
use crate::component::lifecycle::Destroy;
use crate::component::time::{Period, Phase};

#[derive(Constant, Clone)]
pub struct Spring();

#[derive(Event)]
pub struct Sprung(pub EntityId);

#[derive(Event)]
pub struct SpawnSpring(f64, f64);

#[derive(Event)]
pub struct SpawnStretchySpring(f64, f64);

pub fn register(dispatcher: &mut Dispatcher, spawner: &mut Spawner) {
    dispatcher.register(spawn_spring);
    dispatcher.register(spawn_stretchy_spring);
    dispatcher.register(collide_spring);

    spawner.register("Spring", |spawn, events| events.fire(SpawnSpring(spawn.x, spawn.y)));
}

fn spawn_spring(&SpawnSpring(x, y): &SpawnSpring, world: &mut Entities, _events: &mut Events) {
    world.spawn(entity()
        .with(Spring())
        .with(Interactable())
        .with(Sprite::sprite("spring_down", 3))
        .with(Position(x, y))
        .with(Shape::bbox(0.15, 0.0, 0.6, 0.2))
    );
}

fn spawn_stretchy_spring(&SpawnStretchySpring(x, y): &SpawnStretchySpring, world: &mut Entities, events: &mut Events) {
    let stretch_id = world.spawn(entity()
        .with(Sprite::sprite("spring_up", 3))
        .with(Phase(0.0))
        .with(Period(0.7))
        .with(Animation{
            sprites: vec![
                "spring_mid",
                "spring_up",
                "spring_up",
                "spring_up",
                "spring_up",
                "spring_up",
                "spring_up",
                "spring_up",
                "spring_mid",
                "spring_mid"],
            layer: 3})
        .with(Position(x, y))
    );
    events.schedule("Game", Duration::from_millis(700), Destroy(stretch_id));
    events.schedule("Game", Duration::from_millis(700), SpawnSpring(x, y));
}

fn collide_spring(Collided(first, second, _): &Collided, world: &mut Entities, events: &mut Events) {
    world.apply_to_pair(first, second, |Hero(), (Spring(), Position(x, y))| {
        activate_spring(first, second, events, x, y);
    });
    world.apply_to_pair(second, first, |Hero(), (Spring(), Position(x, y))| {
        activate_spring(second, first, events, x, y);
    });
}

fn activate_spring(first: &EntityId, second: &EntityId, events: &mut Events, x: f64, y: f64) {
    events.fire(Sprung(*first));
    events.fire(Destroy(*second));
    events.fire(SpawnStretchySpring(x, y))
}

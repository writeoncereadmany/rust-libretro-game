use crate::component::graphics::Sprite;
use crate::component::physics::Position;
use derive::Event;
use engine::entities::entity::{entity, Entities};
use engine::events::dispatcher::Dispatcher;
use engine::events::event::Events;
use engine::events::spawner::Spawner;

#[derive(Event)]
struct SpawnHero(f64, f64);

pub fn register(dispatcher: &mut Dispatcher, spawner: &mut Spawner) {
    dispatcher.register(spawn_hero);

    spawner.register("Hero", |spawn, events| {
        events.fire(SpawnHero(spawn.x as f64, spawn.y as f64))
    });
}

fn spawn_hero(&SpawnHero(x, y): &SpawnHero, world: &mut Entities, _events: &mut Events) {
    world.spawn(
        entity()
            .with(Sprite("panda_stand"))
            .with(Position(x, y)),
    );
}
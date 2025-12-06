use crate::assets::map::Spawn;
use crate::component::graphics::{Animation, Phase, Sprite};
use crate::component::physics::Position;
use derive::Event;
use engine::entities::entity::{Entities, entity};
use engine::events::dispatcher::Dispatcher;
use engine::events::event::Events;
use engine::events::spawner::Spawner;

#[derive(Event)]
pub struct SpawnCoin(f64, f64);

pub fn register(dispatcher: &mut Dispatcher, spawner: &mut Spawner<Spawn>) {
    dispatcher.register(spawn_coin);

    spawner.register("Coin", |spawn, events| {
        events.fire(SpawnCoin(spawn.x as f64, spawn.y as f64))
    });
}

fn spawn_coin(&SpawnCoin(x, y): &SpawnCoin, world: &mut Entities, events: &mut Events) {
    world.spawn(
        entity()
            .with(Animation {
                sprites: vec!["coin_1", "coin_2", "coin_3", "coin_4"],
                period: 0.5,
            })
            .with(Phase((-0.005 * x + 0.015 * y) % 1.0))
            .with(Sprite("coin_1"))
            .with(Position(x, y)),
    );
}

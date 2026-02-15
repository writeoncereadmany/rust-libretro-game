use crate::component::graphics::{Animation, Phase, Sprite};
use crate::component::physics::Position;
use derive::{Constant, Event};
use engine::entities::entity::{Entities, entity, EntityId};
use engine::events::dispatcher::Dispatcher;
use engine::events::event::Events;
use engine::events::spawner::Spawner;
use engine::shapes::shape::Shape;
use crate::component::collisions::{Collision, Pickup};
use crate::component::lifecycle::Destroy;

#[derive(Event)]
pub struct SpawnCoin(f64, f64);

#[derive(Constant, Clone)]
pub struct Coin;

pub fn register(dispatcher: &mut Dispatcher, spawner: &mut Spawner) {
    dispatcher.register(spawn_coin);
    dispatcher.register(pickup_coin);

    spawner.register("Coin", |spawn, events| {
        events.fire(SpawnCoin(spawn.x as f64, spawn.y as f64))
    });
}

fn spawn_coin(&SpawnCoin(x, y): &SpawnCoin, world: &mut Entities, _events: &mut Events) {
    world.spawn(
        entity()
            .with(Coin)
            .with(Pickup)
            .with(Animation {
                sprites: vec!["coin_1", "coin_2", "coin_3", "coin_4"],
                period: 0.5,
            })
            .with(Phase((-0.005 * x + 0.015 * y) % 1.0))
            .with(Sprite("coin_1"))
            .with(Position(x, y))
            .with(Shape::circle((6.0, -6.0), 4.0))
    );
}

fn pickup_coin(Collision(_hero, coin): &Collision, world: &mut Entities, events: &mut Events) {
    world.apply_to(coin, |(Coin)| events.fire(Destroy(*coin)));
}
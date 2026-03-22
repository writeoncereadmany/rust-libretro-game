use crate::component::collisions::{Collided, Pickup};
use crate::component::graphics::{Animation, Sprite};
use crate::component::lifecycle::Destroy;
use crate::component::physics::Position;
use crate::game::game::Score;
use derive::{Constant, Event};
use engine::entities::entity::{entity, Entities, EntityId};
use engine::events::dispatcher::Dispatcher;
use engine::events::event::Events;
use engine::events::spawner::Spawner;
use engine::shapes::shape::Shape;
use std::time::Duration;
use crate::component::time::{Period, Phase};

#[derive(Event)]
pub struct SpawnCoin(f64, f64);

#[derive(Event)]
pub struct SpawnSparkle(f64, f64);

#[derive(Event)]
pub struct PickupCoin(EntityId);

#[derive(Constant, Clone)]
pub struct Coin();

pub fn register(dispatcher: &mut Dispatcher, spawner: &mut Spawner) {
    dispatcher.register(spawn_coin);
    dispatcher.register(spawn_sparkle);
    dispatcher.register(pickup_coin);
    dispatcher.register(collect_coin);

    spawner.register("Coin", |spawn, events| {
        events.fire(SpawnCoin(spawn.x, spawn.y))
    });
}

fn spawn_coin(&SpawnCoin(x, y): &SpawnCoin, world: &mut Entities, _events: &mut Events) {
    world.spawn(
        entity()
            .with(Coin())
            .with(Pickup)
            .with(Animation {
                sprites: vec!["coin_1", "coin_2", "coin_3", "coin_4"],
                layer: 5,
            })
            .with(Period(0.5))
            .with(Phase((-0.005 * x + 0.015 * y) % 1.0))
            .with(Sprite::sprite("coin_1", 5))
            .with(Position(x, y))
            .with(Shape::circle((6.0, 6.0), 4.0))
    );
}

fn spawn_sparkle(&SpawnSparkle(x, y): &SpawnSparkle, world: &mut Entities, events: &mut Events) {
    let entity_id = world.spawn(
        entity()
            .with(Animation {
                sprites: vec!["sparkle_small", "sparkle_big", "sparkle_small"],
                layer: 5,
            })
            .with(Period(0.35))
            .with(Phase(0.0))
            .with(Sprite::sprite("sparkle_small", 5))
            .with(Position(x, y))
    );
    events.schedule(Duration::from_secs_f64(0.35), Destroy(entity_id));
}

fn pickup_coin(Collided(first, second): &Collided, world: &mut Entities, events: &mut Events) {
    world.apply_to(first, |Coin()| events.fire(PickupCoin(*first)));
    world.apply_to(second, |Coin()| events.fire(PickupCoin(*second)));
}

fn collect_coin(PickupCoin(coin): &PickupCoin, world: &mut Entities, events: &mut Events) {
    if let Some(Position(x, y)) = world.delete(coin) {
        events.fire(SpawnSparkle(x, y));
        events.fire(Score(5));
    }
}
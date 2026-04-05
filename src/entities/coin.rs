use crate::component::collisions::{Collided, Pickup};
use crate::component::graphics::{Animation, Sprite};
use crate::component::physics::Position;
use crate::component::time::{Period, Phase};
use crate::entities::sparkle::SpawnSparkle;
use crate::game::game::Score;
use derive::{Constant, Event};
use engine::entities::entity::{entity, Component, Entities, EntityId, Id};
use engine::events::dispatcher::Dispatcher;
use engine::events::event::Events;
use engine::events::spawner::Spawner;
use engine::shapes::shape::Shape;
use crate::component::lifecycle::Destroy;

#[derive(Event)]
pub struct SpawnCoin(f64, f64, bool);

#[derive(Event)]
pub struct SpawnBell(f64, f64);

#[derive(Event)]
pub struct SpawnCoinRespawn(f64, f64);

#[derive(Event)]
pub struct RespawnGhostCoins();

#[derive(Event)]
pub struct PickupCoin(EntityId);

#[derive(Event)]
pub struct PickupGhostCoin(EntityId);

#[derive(Constant, Clone)]
pub struct Bell();

#[derive(Constant, Clone)]
pub struct Coin();

#[derive(Constant, Clone)]
pub struct GhostCoin();

#[derive(Constant, Clone)]
pub struct CoinRespawn();

pub fn register(dispatcher: &mut Dispatcher, spawner: &mut Spawner) {
    dispatcher.register(spawn_coin);
    dispatcher.register(spawn_bell);
    dispatcher.register(spawn_coin_respawn);
    dispatcher.register(respawn_ghost_coins);
    dispatcher.register(pickup_coin);
    dispatcher.register(collect_coin);
    dispatcher.register(collect_ghost_coin);

    spawner.register("Coin", |spawn, events| events.fire(SpawnCoin(spawn.x, spawn.y, false)));
    spawner.register("Bell", |spawn, events| events.fire(SpawnBell(spawn.x, spawn.y)));
}

fn spawn_coin(&SpawnCoin(x, y, is_ghost): &SpawnCoin, world: &mut Entities, _events: &mut Events) {
    if is_ghost {
        spawn_coin_inner(world, x, y, GhostCoin(), vec!["silver_coin_1", "silver_coin_2", "silver_coin_3", "silver_coin_4"]);
    } else {
        spawn_coin_inner(world, x, y, Coin(), vec!["coin_1", "coin_2", "coin_3", "coin_4"]);
    }
}

fn spawn_bell(&SpawnBell(x, y): &SpawnBell, world: &mut Entities, _events: &mut Events) {
    world.spawn(
        entity()
            .with(Bell())
            .with(Pickup())
            .with(Sprite::sprite("bell", 5))
            .with(Position(x, y))
            .with(Shape::bbox(0.0, 0.0, 12.0, 12.0))
    );
}

fn spawn_coin_inner<T: Component>(world: &mut Entities, x: f64, y: f64, marker: T, sprites: Vec<&'static str>) {
    world.spawn(
        entity()
            .with(marker)
            .with(Pickup())
            .with(Animation {
                sprites,
                layer: 5,
            })
            .with(Period(0.5))
            .with(Phase((-0.005 * x + 0.015 * y) % 1.0))
            .with(Sprite::sprite("error", 5))
            .with(Position(x, y))
            .with(Shape::circle((6.0, 6.0), 4.0))
    );
}

fn spawn_coin_respawn(&SpawnCoinRespawn(x, y): &SpawnCoinRespawn, world: &mut Entities, _events: &mut Events) {
    world.spawn(
        entity()
            .with(CoinRespawn())
            .with(Position(x, y))
    );
}

fn pickup_coin(Collided(first, second): &Collided, world: &mut Entities, events: &mut Events) {
    world.apply_to(first, |Coin()| events.fire(PickupCoin(*first)));
    world.apply_to(second, |Coin()| events.fire(PickupCoin(*second)));

    world.apply_to(first, |GhostCoin()| events.fire(PickupGhostCoin(*first)));
    world.apply_to(second, |GhostCoin()| events.fire(PickupGhostCoin(*second)));

    world.apply_to(first, |(Bell(), Id(id))| {
        events.fire(Destroy(id));
        events.fire(RespawnGhostCoins());
    });
    world.apply_to(second, |(Bell(), Id(id))| {
        events.fire(Destroy(id));
        events.fire(RespawnGhostCoins());
    });
}

fn collect_coin(PickupCoin(coin): &PickupCoin, world: &mut Entities, events: &mut Events) {
    if let Some(Position(x, y)) = world.delete(coin) {
        events.fire(SpawnSparkle(x, y));
        events.fire(SpawnCoinRespawn(x, y));
        events.fire(Score(5));
    }
}

fn collect_ghost_coin(PickupGhostCoin(coin): &PickupGhostCoin, world: &mut Entities, events: &mut Events) {
    if let Some(Position(x, y)) = world.delete(coin) {
        events.fire(SpawnSparkle(x, y));
        events.fire(SpawnCoinRespawn(x, y));
        events.fire(Score(5));
    }
}

fn respawn_ghost_coins(_: &RespawnGhostCoins, world: &mut Entities, events: &mut Events) {
    world.apply(|(CoinRespawn(), Id(id), Position(x, y))| {
        events.fire(Destroy(id));
        events.fire(SpawnCoin(x, y, true));
    });
}
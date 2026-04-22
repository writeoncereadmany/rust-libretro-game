use crate::component::collisions::{Collided, Interactable};
use crate::component::graphics::Sprite;
use crate::component::physics::Position;
use crate::entities::coin::RespawnGhostCoins;
use crate::entities::key::Unlock;
use crate::game::game::{BuyBonus, BuyMetamultiplier, IncreaseMultiplier, Score};
use derive::{Constant, Event};
use engine::entities::entity::{entity, Entities, EntityId};
use engine::events::dispatcher::Dispatcher;
use engine::events::event::Events;
use engine::events::spawner::Spawner;
use engine::shapes::shape::Shape;
// Fruit score increasingly for each piece eaten,
// (10, 20, 30, 40)
// and then on collecting the last fruit you get a different bonus:

// Apple (ruby): score 100 points
// Cherry (???): add one to multiplier
// Banana (bell): respawn all coins in the stage
// Watermelon (???): if mult at max, cash it in for +1 metamultiplier, otherwise 100 points
// Grapes (???): raise the bonus exit flag
// Strawberry (???): if mult at max, cash it in for 10,000 pts, otherwise 100 points
// Kiwi (key): unlock all lockboxes and chests
// Orange (???): ???

#[derive(Constant, Clone, Copy)]
struct Fruit();

#[derive(Constant, Clone, Copy)]
enum FruitType {
    Apple,
    Banana,
    Cherry,
    Watermelon,
    Grapes,
    Strawberry,
    Kiwi,
    Orange
}

#[derive(Event)]
struct SpawnFruit(f64, f64, FruitType);

#[derive(Event)]
struct PickupFruit(EntityId);

pub fn register(dispatcher: &mut Dispatcher, spawner: &mut Spawner) {
    dispatcher.register(spawn_fruit);
    dispatcher.register(pickup_fruit);
    dispatcher.register(collect_fruit);

    spawner.register("Apple", |spawn, events| events.fire(SpawnFruit(spawn.x, spawn.y, FruitType::Apple)));
    spawner.register("Banana", |spawn, events| events.fire(SpawnFruit(spawn.x, spawn.y, FruitType::Banana)));
    spawner.register("Cherry", |spawn, events| events.fire(SpawnFruit(spawn.x, spawn.y, FruitType::Cherry)));
    spawner.register("Watermelon", |spawn, events| events.fire(SpawnFruit(spawn.x, spawn.y, FruitType::Watermelon)));
    spawner.register("Grapes", |spawn, events| events.fire(SpawnFruit(spawn.x, spawn.y, FruitType::Grapes)));
    spawner.register("Strawberry", |spawn, events| events.fire(SpawnFruit(spawn.x, spawn.y, FruitType::Strawberry)));
    spawner.register("Kiwi", |spawn, events| events.fire(SpawnFruit(spawn.x, spawn.y, FruitType::Kiwi)));
    spawner.register("Orange", |spawn, events| events.fire(SpawnFruit(spawn.x, spawn.y, FruitType::Orange)));
}

fn spawn_fruit(&SpawnFruit(x, y, fruit): &SpawnFruit, world: &mut Entities, _events: &mut Events) {
    world.spawn(
        entity()
            .with(Fruit())
            .with(fruit)
            .with(Interactable())
            .with(Sprite::sprite(sprite_for(fruit), 5))
            .with(Position(x, y))
            .with(Shape::bbox(0.0, 0.0, 12.0, 12.0))
    );
}

fn pickup_fruit(Collided(first, second): &Collided, world: &mut Entities, events: &mut Events) {
    world.apply_to(first, |Fruit()| events.fire(PickupFruit(*first)));
    world.apply_to(second, |Fruit()| events.fire(PickupFruit(*second)));
}

fn collect_fruit(PickupFruit(id): &PickupFruit, world: &mut Entities, events: &mut Events) {
    if let Some(fruit_type) = world.delete(id) {
        match world.collect::<Fruit>().len() {
            0 => match fruit_type {
                FruitType::Apple => events.fire(Score(100)),
                FruitType::Banana => events.fire(RespawnGhostCoins()),
                FruitType::Cherry => events.fire(IncreaseMultiplier()),
                FruitType::Watermelon => events.fire(BuyMetamultiplier()),
                FruitType::Grapes => {}
                FruitType::Strawberry => events.fire(BuyBonus()),
                FruitType::Kiwi => events.fire(Unlock()),
                FruitType::Orange => {}
            },
            1 => events.fire(Score(40)),
            2 => events.fire(Score(30)),
            3 => events.fire(Score(20)),
            _otherwise => events.fire(Score(10)),
        }
    }
}

fn sprite_for(fruit_type: FruitType) -> &'static str {
    match fruit_type {
        FruitType::Apple => "apple",
        FruitType::Banana => "banana",
        FruitType::Cherry => "cherry",
        FruitType::Watermelon => "watermelon",
        FruitType::Grapes => "grapes",
        FruitType::Strawberry => "strawberry",
        FruitType::Kiwi => "kiwi",
        FruitType::Orange => "orange"
    }
}
use crate::component::collisions::{Collision, Pickup};
use crate::component::graphics::{Animation, Phase, Sprite};
use crate::component::physics::Position;
use crate::game::game::StartLevel;
use derive::{Constant, Event};
use engine::entities::entity::{entity, Entities, EntityId};
use engine::events::dispatcher::Dispatcher;
use engine::events::event::Events;
use engine::events::spawner::{Spawn, Spawner};
use engine::shapes::shape::Shape;
use tiled::PropertyValue;

#[derive(Event)]
pub struct SpawnFlag(f64, f64, String);

#[derive(Event)]
pub struct PickupFlag(EntityId);

#[derive(Constant, Clone)]
pub struct Flag();

#[derive(Constant, Clone)]
pub struct NextLevel(String);

pub fn register(dispatcher: &mut Dispatcher, spawner: &mut Spawner) {
    dispatcher.register(spawn_flag);
    dispatcher.register(pickup_flag);
    dispatcher.register(collect_flag);

    spawner.register("Flag", |spawn, events| {
        events.fire(SpawnFlag(spawn.x, spawn.y, destination(&spawn).unwrap_or("start".to_string())))
    });
}

fn destination(spawn: &Spawn) -> Option<String> {
    if let Some(dest) = spawn.object.properties.get("destination") {
        match dest {
            PropertyValue::StringValue(map) => Some(map.clone()),
            _otherwise => None
        }
    } else {
        None
    }
}


fn spawn_flag(SpawnFlag(x, y, dest): &SpawnFlag, world: &mut Entities, _events: &mut Events) {
    world.spawn(
        entity()
            .with(Flag())
            .with(Pickup)
            .with(Animation {
                sprites: vec!["flag_blue_1", "flag_blue_2"],
                layer: 5,
                period: 0.2,
            })
            .with(Phase(0.0))
            .with(Sprite("flag_blue_1", 5))
            .with(Position(*x, *y))
            .with(NextLevel(dest.clone()))
            .with(Shape::bbox(0.0, 0.0, 12.0, 12.0))
    );

    world.spawn(
        entity()
            .with(Sprite("flagpole_silver", 3))
            .with(Position(*x, *y))
    );
}


fn pickup_flag(Collision(first, second): &Collision, world: &mut Entities, events: &mut Events) {
    world.apply_to(first, |Flag()| events.fire(PickupFlag(*first)));
    world.apply_to(second, |Flag()| events.fire(PickupFlag(*second)));
}

fn collect_flag(PickupFlag(flag): &PickupFlag, world: &mut Entities, events: &mut Events) {
    if let Some(NextLevel(destination)) = world.delete(flag) {
        events.fire(StartLevel(destination));
    }
}

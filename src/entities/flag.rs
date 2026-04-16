use crate::component::collisions::{Collided, Pickup};
use crate::component::graphics::{Animation, Sprite};
use crate::component::physics::Position;
use crate::component::time::{Period, Phase};
use crate::entities::radial::SpawnRadials;
use crate::game::game::CompleteLevel;
use derive::{Constant, Event};
use engine::entities::entity::{entity, Entities, EntityId};
use engine::events::dispatcher::Dispatcher;
use engine::events::event::Events;
use engine::events::spawner::{Spawn, Spawner};
use engine::shapes::shape::Shape;
use std::time::Duration;
use tiled::PropertyValue;

#[derive(Event)]
pub struct SpawnFlag(f64, f64, String);

#[derive(Event)]
pub struct SpawnFlagpole(f64, f64);

#[derive(Event)]
pub struct PickupFlag(EntityId);

#[derive(Constant, Clone)]
pub struct Flag();

#[derive(Constant, Clone)]
pub struct NextLevel(String);

pub fn register(dispatcher: &mut Dispatcher, spawner: &mut Spawner) {
    dispatcher.register(spawn_flag);
    dispatcher.register(spawn_flagpole);
    dispatcher.register(pickup_flag);
    dispatcher.register(collect_flag);

    spawner.register("Flag", |spawn, events| {
        events.fire(SpawnRadials(spawn.x, spawn.y, vec!["ball_blue"], 8));
        events.schedule("Game", Duration::from_secs_f64(2.4), SpawnFlag(spawn.x, spawn.y, destination(&spawn).unwrap_or("start".to_string())));
        events.fire(SpawnFlagpole(spawn.x, spawn.y,));
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
            .with(Pickup())
            .with(Animation {
                sprites: vec!["flag_blue_1", "flag_blue_2"],
                layer: 5,
            })
            .with(Period(0.2))
            .with(Phase(0.0))
            .with(Sprite::sprite("flag_blue_1", 5))
            .with(Position(*x, *y))
            .with(NextLevel(dest.clone()))
            .with(Shape::bbox(0.0, 0.0, 12.0, 12.0))
    );
}

fn spawn_flagpole(SpawnFlagpole(x, y): &SpawnFlagpole, world: &mut Entities, _events: &mut Events) {
    world.spawn(
        entity()
            .with(Sprite::sprite("flagpole_silver", 3))
            .with(Position(*x, *y))
    );
}

fn pickup_flag(Collided(first, second): &Collided, world: &mut Entities, events: &mut Events) {
    world.apply_to(first, |Flag()| events.fire(PickupFlag(*first)));
    world.apply_to(second, |Flag()| events.fire(PickupFlag(*second)));
}

fn collect_flag(PickupFlag(flag): &PickupFlag, world: &mut Entities, events: &mut Events) {
    if let Some(NextLevel(destination)) = world.delete(flag) {
        events.fire(CompleteLevel(destination));
    }
}

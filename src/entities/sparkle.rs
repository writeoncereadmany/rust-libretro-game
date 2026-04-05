use crate::component::graphics::{Animation, Sprite};
use crate::component::lifecycle::Destroy;
use crate::component::physics::Position;
use crate::component::time::{Period, Phase};
use derive::Event;
use engine::entities::entity::{entity, Entities};
use engine::events::dispatcher::Dispatcher;
use engine::events::event::Events;
use engine::events::spawner::Spawner;
use std::time::Duration;

#[derive(Event)]
pub struct SpawnSparkle(pub f64, pub f64);

pub fn register(dispatcher: &mut Dispatcher, _spawner: &mut Spawner) {
    dispatcher.register(spawn_sparkle);
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


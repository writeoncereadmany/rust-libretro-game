use derive::Event;
use engine::entities::entity::{Entities, EntityId};
use engine::events::dispatcher::Dispatcher;
use engine::events::event::Events;

#[derive(Event)]
pub struct Destroy(pub EntityId);

pub fn register(dispatcher: &mut Dispatcher) {
    dispatcher.register(destroy);
}

fn destroy(Destroy(id): &Destroy, world: &mut Entities, _events: &mut Events) {
    world.delete::<()>(id);
}
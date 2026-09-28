use derive::{Event, system};
use engine::entities::entity::{Entities, EntityId};
use engine::events::event::Events;

#[derive(Event)]
pub struct Destroy(pub EntityId);

#[system]
fn destroy(Destroy(id): &Destroy, world: &mut Entities, _events: &mut Events) {
    world.delete::<()>(id);
}
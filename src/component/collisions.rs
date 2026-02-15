use crate::component::physics::{Position, Translation};
use derive::{Constant, Event};
use engine::entities::entity::{Entities, EntityId};
use engine::entities::entity::Id;
use engine::events::dispatcher::Dispatcher;
use engine::events::event::Events;
use engine::shapes::shape::Shape;

#[derive(Event)]
pub struct CheckCollisions;

#[derive(Event)]
pub struct Collision(pub EntityId, pub EntityId);

#[derive(Constant, Clone)]
pub struct Actor;

#[derive(Constant, Clone)]
pub struct Pickup;

pub fn register(dispatcher: &mut Dispatcher) {
    dispatcher.register(handle_collisions);
}

pub fn handle_collisions(_ : &CheckCollisions, world: &mut Entities, events: &mut Events)
{
    world.for_each_pair(|(Actor, Id(hero_id), hero_shape, hero_position, hero_translation), 
                         (Pickup, Id(pickup_id), pickup_shape, pickup_position, pickup_translation)|
        {
            if collides(hero_shape, hero_position, hero_translation, pickup_shape, pickup_position, pickup_translation)
            {
                events.fire(Collision(*hero_id, *pickup_id));
            }
        }
    )
}

fn collides(moving: &Shape, &Position(mx, my): &Position, &Translation(mtx, mty): &Translation,
            other: &Shape, &Position(ox, oy): &Position, other_translation: &Option<Translation>) -> bool
{
    let moving = moving.translate(&(mx, my));
    let other = other.translate(&(ox, oy));
    if let Some(Translation(otx, oty)) = other_translation {
        moving.intersects_moving(&other, &(mtx - otx, mty - oty))
    }
    else {  
        moving.intersects_moving(&other, &(mtx, mty))
    }
}
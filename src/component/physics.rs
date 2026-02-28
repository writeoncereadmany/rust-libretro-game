use std::time::Duration;
use engine::entities::entity;
use derive::Variable;
use engine::entities::entity::Entities;
use engine::events::dispatcher::Dispatcher;
use engine::events::event::Events;
use crate::component::collisions::{CheckCollisions, ResolveCollisions};

#[derive(Clone, Variable)]
pub struct Position(pub f64, pub f64);

#[derive(Clone, Variable)]
pub struct Velocity(pub f64, pub f64);

#[derive(Clone, Variable)]
pub struct Acceleration(pub f64, pub f64);

#[derive(Clone, Variable)]
pub struct Translation(pub f64, pub f64);

pub fn register(dispatcher: &mut Dispatcher) {
    dispatcher.register(integrate);
    dispatcher.register(resolve_collisions);
}

pub fn integrate(dt: &Duration, world: &mut Entities, events: &mut Events) {
    let dt = dt.as_secs_f64();
    world.apply(|(Acceleration(ddx, ddy), Velocity(dx, dy))| Velocity (dx + (ddx * dt), dy + (ddy * dt)));
    world.apply(|(Velocity(dx, dy))| Translation (dx * dt, dy * dt));
    events.fire(CheckCollisions);
}

pub fn resolve_collisions(_ : &ResolveCollisions, world: &mut Entities, _events: &mut Events) {
    world.apply(|(Position(x, y), Translation(tx, ty))| { Position(x + tx, y + ty)})
}
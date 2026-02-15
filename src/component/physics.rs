use std::time::Duration;
use engine::entities::entity;
use derive::Variable;
use engine::entities::entity::Entities;
use engine::events::dispatcher::Dispatcher;
use engine::events::event::Events;
use crate::component::collisions::CheckCollisions;

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
}

pub fn integrate(dt: &Duration, world: &mut Entities, events: &mut Events) {
    let dt = dt.as_secs_f64();
    world.apply(|(Acceleration(ddx, ddy), Velocity(dx, dy))| Velocity (dx + (ddx * dt), dy + (ddy * dt)));
    world.apply(|(Velocity(dx, dy))| Translation (dx * dt, dy * dt));
    // temporary - eventually we'll have collision detection apply first
    world.apply(|(Translation(dx, dy), Position(x, y))| Position(x + dx, y + dy));
    events.fire(CheckCollisions);
}
use crate::component::collisions::{CheckCollisions, ResolveCollisions};
use derive::{Event, Variable};
use engine::entities::entity::Entities;
use engine::events::dispatcher::Dispatcher;
use engine::events::event::Events;
use std::time::Duration;

const QUANTIZATION_FACTOR: f64 = 1024.0;

#[derive(Clone, Variable)]
pub struct Position(pub f64, pub f64);

#[derive(Clone, Variable)]
pub struct Velocity(pub f64, pub f64);

#[derive(Clone, Variable)]
pub struct Acceleration(pub f64, pub f64);

#[derive(Clone, Variable)]
pub struct Translation(pub f64, pub f64);

#[derive(Clone, Event)]
pub struct QuantizeEvent();

pub fn register(dispatcher: &mut Dispatcher) {
    dispatcher.register(integrate);
    dispatcher.register(resolve_collisions);
    dispatcher.register(quantize)
}

pub fn integrate(dt: &Duration, world: &mut Entities, events: &mut Events) {
    let dt = dt.as_secs_f64();
    world.apply(|(Acceleration(ddx, ddy), Velocity(dx, dy))| Velocity (dx + (ddx * dt), dy + (ddy * dt)));
    world.apply(|(Velocity(dx, dy))| Translation (dx * dt, dy * dt));
    events.fire(CheckCollisions);
}

pub fn resolve_collisions(_ : &ResolveCollisions, world: &mut Entities, events: &mut Events) {
    world.apply(|(Position(x, y), Translation(tx, ty))| { Position(x + tx, y + ty)});
    events.fire(QuantizeEvent());
}

fn quantize(_ : &QuantizeEvent, world: &mut Entities, _events: &mut Events) {
    world.apply(|Position(x, y)| Position(quant(x), quant(y)));
}

fn quant(x: f64) -> f64 {
    (x * QUANTIZATION_FACTOR).round() / QUANTIZATION_FACTOR
}
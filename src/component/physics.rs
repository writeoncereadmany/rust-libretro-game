use crate::component::collisions::{CheckCollisions, ResolveCollisions};
use derive::{Constant, Variable};
use engine::entities::entity::Entities;
use engine::events::dispatcher::Dispatcher;
use engine::events::event::Events;
use std::time::Duration;

const QUANTIZATION_FACTOR: f64 = 1024.0;

const GRAVITY: f64 = 1200.0;

#[derive(Clone, Variable)]
pub struct Position(pub f64, pub f64);

#[derive(Clone, Variable)]
pub struct Velocity(pub f64, pub f64);

#[derive(Clone, Variable)]
pub struct Acceleration(pub f64, pub f64);

#[derive(Clone, Variable)]
pub struct Gravity();

#[derive(Clone, Variable)]
pub struct Translation(pub f64, pub f64);

#[derive(Clone, Constant)]
pub struct VelocityCap(pub f64, pub f64);

pub fn register(dispatcher: &mut Dispatcher) {
    dispatcher.register(integrate);
    dispatcher.register(resolve_collisions);
}

fn integrate(dt: &Duration, world: &mut Entities, events: &mut Events) {
    let dt = dt.as_secs_f64();
    world.apply(|(Gravity(), Acceleration(ddx, ddy))| Acceleration(ddx, ddy - GRAVITY));
    world.apply(|(Acceleration(ddx, ddy), Velocity(dx, dy))| (Acceleration(0.0, 0.0), Velocity (dx + (ddx * dt), dy + (ddy * dt))));
    world.apply(|(Velocity(dx, dy), VelocityCap(max_dx, max_dy))| Velocity(dx.clamp(-max_dx, max_dx), dy.clamp(-max_dy, max_dy)));
    world.apply(|Velocity(dx, dy)| Translation (dx * dt, dy * dt));
    events.fire(CheckCollisions);
}

fn resolve_collisions(_ : &ResolveCollisions, world: &mut Entities, events: &mut Events) {
    world.apply(|(Position(x, y), Translation(tx, ty))| { Position(x + tx, y + ty)});
}
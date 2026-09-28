use derive::{Variable, system};
use engine::entities::entity::Entities;
use engine::events::event::Events;
use std::time::Duration;

#[derive(Clone, Variable)]
pub struct Phase(pub f64);

#[derive(Clone, Variable)]
pub struct Period(pub f64);

#[derive(Clone, Variable)]
pub struct Age(pub f64);

#[system]
fn update_time(dt: &Duration, world: &mut Entities, _events: &mut Events) {
    world.apply(|(Phase(phase), Period(period))| {
        Phase((phase + dt.as_secs_f64() / period) % 1.0)
    });
    
    world.apply(|Age(age)| Age(age + dt.as_secs_f64()));
}
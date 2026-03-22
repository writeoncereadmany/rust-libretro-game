use crate::app::application::AfterUpdate;
use crate::component::graphics::Sprite;
use crate::component::lifecycle::Destroy;
use crate::component::physics::Position;
use crate::component::time::{Age, Period, Phase};
use derive::{Constant, Event, Variable};
use engine::entities::entity::{entity, Entities};
use engine::events::dispatcher::Dispatcher;
use engine::events::event::Events;
use engine::events::spawner::Spawner;
use std::f64::consts::PI;
use std::time::Duration;

#[derive(Clone, Variable)]
struct Radius(f64);

#[derive(Clone, Constant)]
struct AngleOffset(f64);

#[derive(Clone, Variable)]
struct Angle(f64);

#[derive(Clone, Variable)]
struct Center(f64, f64);

#[derive(Event, Clone)]
pub struct SpawnRadials(pub f64, pub f64, pub Vec<&'static str>, pub i32);


pub fn register(dispatcher: &mut Dispatcher, _spawner: &mut Spawner) {
    dispatcher.register(spawn_radials);
    dispatcher.register(radial_events);
}

fn spawn_radials(SpawnRadials(x, y, sprites, elements): &SpawnRadials, entities: &mut Entities, events: &mut Events)
{
    for i in 0..*elements {
        spawn_radial(*x, *y, sprites[i as usize % sprites.len()], i as f64 * (2.0 * PI) / *elements as f64, entities, events);
    }
}

fn spawn_radial(x: f64, y: f64, sprite: &'static str, theta: f64, entities: &mut Entities, events: &mut Events) {
    let radial_id = entities.spawn(entity()
        .with(Center(x, y))
        .with(Sprite::sprite(sprite, 20))
        .with(Period(0.6))
        .with(Phase(0.0))
        .with(AngleOffset(theta))
        .with(Age(0.0))
    );

    events.schedule(Duration::from_millis(2800), Destroy(radial_id));
}

fn radial_events(_event: &AfterUpdate, entities: &mut Entities, _events: &mut Events)
{
    entities.apply(|(Phase(phase), AngleOffset(theta))| Angle((phase * 2.0 * PI) + theta));
    entities.apply(update_radius);
    entities.apply(update_period);
    entities.apply(|(Center(x, y), Angle(theta), Radius(r))| Position(x + (r * f64::sin(theta)), y + (r * f64::cos(theta))));
}

fn update_radius(Age(age): Age) -> Radius {
    if age < 0.8 {
        let through_phase = 1.0 - (age / 0.8);
        Radius(36.0 + (240.0 * through_phase))
    } else if age < 2.2 {
        Radius(36.0)
    } else {
        let through_phase = 1.0 - (age - 2.2) / 0.2;
        Radius(36.0 * through_phase)
    }
}

fn update_period(Age(age): Age) -> Period {
    if age < 0.8 {
        Period(1.2)
    } else if age < 1.8 {
        let through_phase = (age - 0.8) / 1.0;
        Period(1.2 - through_phase)
    } else {
        Period(1000.0)
    }
}
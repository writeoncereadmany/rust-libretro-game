use std::time::Duration;
use derive::{system, Event, Variable};
use engine::entities::entity::{entity, Entities};
use engine::events::event::Events;
use engine::renderer::spritefont::{Alignment, HorizontalAlignment::CENTER, VerticalAlignment::MIDDLE};
use crate::component::graphics::Sprite::Text;
use crate::component::physics::{Position, Velocity};
use crate::game::game::{CompleteLevel, Score};

#[derive (Clone, Variable)]
struct TimeBonus(f64);

#[derive(Event)]
pub struct SpawnTimeBonus(pub f64);

#[derive(Event)]
struct DisplayTimeBonus(u32);

#[system]
fn spawn_time_bonus(SpawnTimeBonus(time): &SpawnTimeBonus, world: &mut Entities, events: &mut Events) {
    world.spawn(entity()
        .with(TimeBonus(*time))
    );
}

#[system]
fn spawn_display_time_bonus(DisplayTimeBonus(time): &DisplayTimeBonus, world: &mut Entities, events: &mut Events) {
    world.spawn(entity()
        .with(Text(
            format!("Time bonus: {}!", time),
            "Spritefont_Medium".to_string(),
            Alignment::aligned(CENTER, MIDDLE),
            20))
        .with(Position(192.0, 92.0))
        .with(Velocity(0.0, 20.0))
    );
}

#[system]
fn decrement_bonus(dt: &Duration, world: &mut Entities, _events: &mut Events) {
    world.apply(|TimeBonus(t)| TimeBonus(t - dt.as_secs_f64()))
}

#[system]
fn apply_bonus(_complete: &CompleteLevel, world: &mut Entities, events: &mut Events) {
    world.apply(|TimeBonus(t)| {
        let bonus = (t * 2.0) as u32 * 5;
        events.fire(DisplayTimeBonus(bonus));
        events.fire(Score(bonus));
    })
}
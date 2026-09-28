use std::time::Duration;
use crate::component::collisions::{Collided, Interactable};
use crate::component::graphics::{Animation, Sprite};
use crate::component::lifecycle::Destroy;
use crate::component::physics::Position;
use crate::component::time::{Period, Phase};
use crate::entities::hero::{Hero, Jump};
use derive::{Constant, Event, system, spawn};
use engine::entities::entity::{entity, Entities, EntityId};
use engine::events::event::Events;
use engine::events::spawner::Spawn;
use engine::shapes::shape::Shape;
use engine::shapes::vec2d::Vec2d;

#[derive(Constant, Clone)]
struct Bubble();

#[derive(Event)]
pub struct SpawnBubble(f64, f64);

#[derive(Event)]
pub struct SpawnBubbles(pub f64, pub f64);

#[derive(Event)]
pub struct SpawnBubblePop(f64, f64);

#[spawn("Bubble")]
fn spawn_bubble_from_map(spawn: Spawn, events: &mut Events) {
    events.fire(SpawnBubble(spawn.x, spawn.y));
}

#[system]
fn spawn_bubble(&SpawnBubble(x, y): &SpawnBubble, world: &mut Entities, _events: &mut Events) {
    world.spawn(entity()
        .with(Bubble())
        .with(Position(x, y))
        .with(Interactable())
        .with(Sprite::sprite("bubble_1", 6))
        .with(Period(1.0))
        .with(Phase(0.0))
        .with(Animation {
            sprites: vec!["bubble_1", "bubble_2", "bubble_1", "bubble_3"],
            layer: 6
        })
        .with(Shape::bbox(0.0, 0.0, 12.0, 12.0))
    );
}

#[system]
fn spawn_pop(&SpawnBubblePop(x, y): &SpawnBubblePop, world: &mut Entities, events: &mut Events) {
    let pop_id = world.spawn(entity()
        .with(Position(x, y))
        .with(Sprite::sprite("bubble_1", 6))
        .with(Period(0.5))
        .with(Phase(0.0))
        .with(Animation {
            sprites: vec!["bubble_pop_1", "bubble_pop_2", "bubble_pop_3"],
            layer: 6
        })
    );
    events.schedule("Game", Duration::from_millis(500), Destroy(pop_id));
}

#[system]
fn spawn_bubbles(&SpawnBubbles(x, y): &SpawnBubbles, world: &mut Entities, events: &mut Events) {
    let pop_id = world.spawn(entity()
        .with(Position(x, y))
        .with(Sprite::sprite("bubble_pop_2", 6))
        .with(Period(0.3))
        .with(Phase(0.0))
        .with(Animation {
            sprites: vec!["bubble_pop_2", "bubble_pop_3"],
            layer: 6
        })
    );
    events.schedule("Game", Duration::from_millis(300), Destroy(pop_id));
}

#[system]
fn collide_bubble(Collided(first, second, push): &Collided, world: &mut Entities, events: &mut Events) {
    world.apply_to_pair(first, second, |Hero(), (Bubble(), Position(x, y))| {
        collide_with_bubble(second, events, x, y, push);
    });
    world.apply_to_pair(second, first, |Hero(), (Bubble(), Position(x, y))| {
        collide_with_bubble(first, events, x, y, &push.scale(&-1.0));
    });
}

fn collide_with_bubble(second: &EntityId, events: &mut Events, x: f64, y: f64, &(_, py): &(f64, f64)) {
    events.fire(Destroy(*second));
    events.fire(SpawnBubblePop(x, y));
    if py > 0.0 {
        events.fire(Jump());
    }
}
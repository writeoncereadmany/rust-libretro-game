use std::time::Duration;
use derive::{spawn, system};
use engine::entities::entity::{entity, Entities, Id};
use engine::events::event::Events;
use engine::events::spawner::Spawn;
use engine::shapes::shape::Shape;
use crate::component::collisions::{Actor, Splash};
use crate::component::graphics::Sprite;
use crate::component::lifecycle::Destroy;
use crate::component::physics::{Acceleration, Gravity, Position, Velocity, VelocityCap};
use crate::entities::bubble::SpawnBubbles;
use crate::entities::hero::{Bubbles, DirectionFacing, Hero, HeroState, MovementIntent, SpawnHero, SpawnRadialAndDelayedHero, SpawnShade};
use crate::entities::radial::SpawnRadials;
use crate::game::game::{Character, CompleteLevel, Failed, Options};

#[spawn("Hero")]
fn hero(spawn: Spawn, events: &mut Events) {
    events.fire(SpawnRadialAndDelayedHero(spawn.x, spawn.y));
}

#[system]
fn spawn_radial_and_delayed_hero(
    &SpawnRadialAndDelayedHero(x, y): &SpawnRadialAndDelayedHero,
    world: &mut Entities,
    events: &mut Events,
) {
    let options: Vec<Options> = world.collect();
    let character = options
        .first()
        .map(|options| options.character.clone())
        .unwrap_or(Character::Bluu);
    match character {
        Character::Bluu => events.fire(SpawnRadials(x, y, vec!["ball_blue", "ball_white"], 8)),
        Character::Redd => events.fire(SpawnRadials(x, y, vec!["ball_brown", "ball_white"], 8)),
    }

    events.schedule("Game", Duration::from_secs_f64(2.4), SpawnHero(x, y));
}

#[system]
fn spawn_hero(&SpawnHero(x, y): &SpawnHero, world: &mut Entities, _events: &mut Events) {
    let options: Vec<Options> = world.collect();

    let character = options
        .first()
        .map(|options| options.character.clone())
        .unwrap_or(Character::Bluu);

    world.spawn(
        entity()
            .with(Hero())
            .with(HeroState::Grounded)
            .with(character)
            .with(DirectionFacing::RIGHT)
            .with(MovementIntent::NEUTRAL)
            .with(Gravity())
            .with(Actor())
            .with(Sprite::sprite("panda_stand", 10))
            .with(Shape::bbox(0.0, 0.0, 12.0, 11.75))
            .with(Acceleration(0.0, 0.0))
            .with(Velocity(0.0, 0.0))
            .with(VelocityCap(200.0, f64::INFINITY))
            .with(Position(x, y)),
    );
}

#[system]
fn create_shade_on_victory(_: &CompleteLevel, world: &mut Entities, events: &mut Events) {
    world.apply(|(Hero(), Id(id), sprite, pos)| {
        events.fire(SpawnShade(sprite, pos));
        events.fire(Destroy(id))
    })
}

#[system]
fn create_shade_on_failure(_: &Failed, world: &mut Entities, events: &mut Events) {
    world.apply(|(Hero(), Id(id), sprite, pos)| {
        events.fire(SpawnShade(sprite, pos));
        events.fire(Destroy(id))
    })
}

#[system]
fn spawn_shade(SpawnShade(sprite, pos): &SpawnShade, world: &mut Entities, _events: &mut Events) {
    world.spawn(entity().with(sprite.clone()).with(pos.clone()));
}

#[system]
fn on_splash(&Splash { id, dy, .. }: &Splash, world: &mut Entities, _events: &mut Events) {
    world.apply_to(&id, |Hero()| {
        if dy < -150.0 {
            Some(Bubbles(0.25))
        } else {
            None
        }
    });
}

#[system]
fn bubbles(dt: &Duration, world: &mut Entities, events: &mut Events) {
    world.apply(|(Bubbles(before), Position(x, y))| {
        let after = before - dt.as_secs_f64();
        if before > 0.2 && after < 0.2 {
            events.fire(SpawnBubbles(x, y));
        }
        if before > 0.1 && after < 0.1 {
            events.fire(SpawnBubbles(x, y));
        }
        if before > 0.0 && after < 0.0 {
            events.fire(SpawnBubbles(x, y));
        }
        if after < 0.0 {
            None
        } else {
            Some(Bubbles(after))
        }
    });
}
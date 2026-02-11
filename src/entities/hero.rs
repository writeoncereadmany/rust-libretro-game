use std::time::Duration;
use rust_libretro::sys::{RETRO_DEVICE_ID_JOYPAD_L, RETRO_DEVICE_ID_JOYPAD_LEFT, RETRO_DEVICE_ID_JOYPAD_RIGHT};
use rust_libretro::types::JoypadState;
use crate::component::graphics::Sprite;
use crate::component::physics::{Acceleration, Position, Velocity};
use derive::{Constant, Event, Variable};
use engine::entities::entity::{entity, Entities};
use engine::events::dispatcher::Dispatcher;
use engine::events::event::Events;
use engine::events::input::InputState;
use engine::events::spawner::Spawner;

const ACCEL:f64 = 100.0;
const DECEL:f64 = 250.0;


#[derive(Constant, Clone)]
struct Hero;

#[derive(Variable, Clone)]
enum DirectionIntent {
    LEFT,
    RIGHT,
    NONE
}

#[derive(Event)]
struct SpawnHero(f64, f64);

pub fn register(dispatcher: &mut Dispatcher, spawner: &mut Spawner) {
    dispatcher.register(spawn_hero);
    dispatcher.register(listen_to_input);
    dispatcher.register(move_hero);

    spawner.register("Hero", |spawn, events| {
        events.fire(SpawnHero(spawn.x as f64, spawn.y as f64))
    });
}

fn spawn_hero(&SpawnHero(x, y): &SpawnHero, world: &mut Entities, _events: &mut Events) {
    world.spawn(
        entity()
            .with(Hero)
            .with(DirectionIntent::NONE)
            .with(Sprite("panda_stand"))
            .with(Velocity(0.0, 0.0))
            .with(Position(x, y)),
    );
}

fn listen_to_input(&InputState(joypad): &InputState, world: &mut Entities, _events: &mut Events) {
    world.apply(|(Hero)|
        match (joypad.contains(JoypadState::LEFT), joypad.contains(JoypadState::RIGHT)) {
            (true, false) => DirectionIntent::LEFT,
            (false, true) => DirectionIntent::RIGHT,
            _otherwise => DirectionIntent::NONE
        })
}

fn move_hero(dt: &Duration, world: &mut Entities, _events: &mut Events) {
    world.apply(|(Hero, directionIntent, Velocity(dx, dy))| {
        match directionIntent {
            DirectionIntent::LEFT => Acceleration(-ACCEL, 0.0),
            DirectionIntent::RIGHT => Acceleration(ACCEL, 0.0),
            DirectionIntent::NONE => Acceleration(dx.signum() * -DECEL, 0.0)
        }
    })
}
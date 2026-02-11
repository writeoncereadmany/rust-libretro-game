use rust_libretro::sys::{RETRO_DEVICE_ID_JOYPAD_L, RETRO_DEVICE_ID_JOYPAD_LEFT, RETRO_DEVICE_ID_JOYPAD_RIGHT};
use rust_libretro::types::JoypadState;
use crate::component::graphics::Sprite;
use crate::component::physics::Position;
use derive::{Constant, Event};
use engine::entities::entity::{entity, Entities};
use engine::events::dispatcher::Dispatcher;
use engine::events::event::Events;
use engine::events::input::InputState;
use engine::events::spawner::Spawner;

#[derive(Constant, Clone)]
struct Hero;

#[derive(Event)]
struct SpawnHero(f64, f64);

pub fn register(dispatcher: &mut Dispatcher, spawner: &mut Spawner) {
    dispatcher.register(spawn_hero);
    dispatcher.register(move_hero);

    spawner.register("Hero", |spawn, events| {
        events.fire(SpawnHero(spawn.x as f64, spawn.y as f64))
    });
}

fn spawn_hero(&SpawnHero(x, y): &SpawnHero, world: &mut Entities, _events: &mut Events) {
    world.spawn(
        entity()
            .with(Hero)
            .with(Sprite("panda_stand"))
            .with(Position(x, y)),
    );
}

fn move_hero(&InputState(joypad): &InputState, world: &mut Entities, _events: &mut Events) {
    world.apply(|(Hero, Position(x, y))|
        match (joypad.contains(JoypadState::LEFT), joypad.contains(JoypadState::RIGHT)) {
            (false, true) => Position(x + 1.0, y),
            (true, false) => Position(x - 1.0, y),
            _otherwise => Position(x, y)
        })
}
use crate::component::graphics::Sprite;
use crate::component::physics::{Position, Velocity};
use derive::{Constant, Event, Variable};
use engine::entities::entity::{entity, Entities};
use engine::events::dispatcher::Dispatcher;
use engine::events::event::Events;
use engine::events::input::InputState;
use engine::events::spawner::Spawner;
use rust_libretro::types::JoypadState;
use engine::shapes::shape::Shape;
use crate::component::collisions::Actor;

const ACCEL:f64 = 100.0;
const DECEL:f64 = 250.0;
const VEL:f64 = 50.0;


#[derive(Constant, Clone)]
struct Hero;

#[derive(Event)]
struct SpawnHero(f64, f64);

pub fn register(dispatcher: &mut Dispatcher, spawner: &mut Spawner) {
    dispatcher.register(spawn_hero);
    dispatcher.register(listen_to_input);

    spawner.register("Hero", |spawn, events| {
        events.fire(SpawnHero(spawn.x as f64, spawn.y as f64))
    });
}

fn spawn_hero(&SpawnHero(x, y): &SpawnHero, world: &mut Entities, _events: &mut Events) {
    world.spawn(
        entity()
            .with(Hero)
            .with(Actor)
            .with(Sprite("panda_stand"))
            .with(Shape::bbox(0.0, -12.0, 12.0, 12.0))
            .with(Velocity(0.0, 0.0))
            .with(Position(x, y)),
    );
}

fn listen_to_input(&InputState(joypad): &InputState, world: &mut Entities, _events: &mut Events) {
    world.apply(|Hero|
        {
            let dx = match (joypad.contains(JoypadState::LEFT), joypad.contains(JoypadState::RIGHT)) {
                (true, false) => -VEL,
                (false, true) => VEL,
                _otherwise => 0.0
            };
            let dy = match (joypad.contains(JoypadState::UP), joypad.contains(JoypadState::DOWN)) {
                (true, false) => -VEL,
                (false, true) => VEL,
                _otherwise => 0.0
            };
            Velocity(dx, dy)
        }
    )
}

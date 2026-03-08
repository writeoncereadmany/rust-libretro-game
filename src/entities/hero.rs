use crate::component::graphics::{Sprite};
use crate::component::physics::{Acceleration, Gravity, Position, Velocity};
use derive::{Constant, Event, Variable};
use engine::entities::entity::{entity, Entities};
use engine::events::dispatcher::Dispatcher;
use engine::events::event::Events;
use engine::events::input::{ButtonPressed, InputState};
use engine::events::spawner::Spawner;
use rust_libretro::types::JoypadState;
use engine::shapes::shape::Shape;
use crate::component::collisions::Actor;

const VEL:f64 = 100.0;


#[derive(Constant, Clone)]
struct Hero();

#[derive(Event)]
struct SpawnHero(f64, f64);

pub fn register(dispatcher: &mut Dispatcher, spawner: &mut Spawner) {
    dispatcher.register(spawn_hero);
    dispatcher.register(listen_to_input_state);
    dispatcher.register(listen_to_button_press);

    spawner.register("Hero", |spawn, events| {
        events.fire(SpawnHero(spawn.x, spawn.y))
    });
}

fn spawn_hero(&SpawnHero(x, y): &SpawnHero, world: &mut Entities, _events: &mut Events) {
    world.spawn(
        entity()
            .with(Hero())
            .with(Gravity())
            .with(Actor)
            .with(Sprite("panda_stand", 10))
            .with(Shape::bbox(0.0, 0.0, 12.0, 12.0))
            .with(Acceleration(0.0, 0.0))
            .with(Velocity(0.0, 0.0))
            .with(Position(x, y)),
    );
}

fn listen_to_input_state(&InputState(joypad): &InputState, world: &mut Entities, _events: &mut Events) {
    world.apply(|(Hero(), Velocity(_dx, dy))|
        {
            let dx = match (joypad.contains(JoypadState::LEFT), joypad.contains(JoypadState::RIGHT)) {
                (true, false) => -VEL,
                (false, true) => VEL,
                _otherwise => 0.0
            };
            Velocity(dx, dy)
        }
    )
}

fn listen_to_button_press(&ButtonPressed(button): &ButtonPressed, world: &mut Entities, _events: &mut Events) {
    world.apply(|(Hero(), Velocity(dx, dy))|
        {
            match button {
                JoypadState::A => Velocity(dx, 300.0),
                _otherwise => Velocity(dx, dy),
            }
        }
    )
}

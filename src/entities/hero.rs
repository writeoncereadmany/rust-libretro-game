use crate::app::application::AfterUpdate;
use crate::component::collisions::{Actor, Push};
use crate::component::graphics::Sprite;
use crate::component::physics::{Acceleration, Gravity, Position, Velocity};
use derive::{Constant, Event, Variable};
use engine::entities::entity::{Entities, entity};
use engine::events::dispatcher::Dispatcher;
use engine::events::event::Events;
use engine::events::input::{ButtonPressed, InputState};
use engine::events::spawner::Spawner;
use engine::shapes::shape::Shape;
use rust_libretro::types::JoypadState;

const VEL: f64 = 100.0;

#[derive(Constant, Clone)]
struct Hero();

#[derive(Event)]
struct SpawnHero(f64, f64);

#[derive(Variable, Clone)]
enum HeroState {
    GROUNDED,
    AIRBORNE,
}

#[derive(Variable, Clone)]
enum DirectionFacing {
    LEFT,
    RIGHT,
}

pub fn register(dispatcher: &mut Dispatcher, spawner: &mut Spawner) {
    dispatcher.register(spawn_hero);
    dispatcher.register(listen_to_input_state);
    dispatcher.register(listen_to_button_press);
    dispatcher.register(on_push);
    dispatcher.register(update_sprite);

    spawner.register("Hero", |spawn, events| {
        events.fire(SpawnHero(spawn.x, spawn.y))
    });
}

fn spawn_hero(&SpawnHero(x, y): &SpawnHero, world: &mut Entities, _events: &mut Events) {
    world.spawn(
        entity()
            .with(Hero())
            .with(HeroState::GROUNDED)
            .with(DirectionFacing::RIGHT)
            .with(Gravity())
            .with(Actor)
            .with(Sprite::sprite("panda_stand", 10))
            .with(Shape::bbox(0.0, 0.0, 12.0, 12.0))
            .with(Acceleration(0.0, 0.0))
            .with(Velocity(0.0, 0.0))
            .with(Position(x, y)),
    );
}

fn listen_to_input_state(
    &InputState(joypad): &InputState,
    world: &mut Entities,
    _events: &mut Events,
) {
    world.apply(|(Hero(), Velocity(_dx, dy))| {
        let dx = match (
            joypad.contains(JoypadState::LEFT),
            joypad.contains(JoypadState::RIGHT),
        ) {
            (true, false) => -VEL,
            (false, true) => VEL,
            _otherwise => 0.0,
        };
        Velocity(dx, dy)
    })
}

fn listen_to_button_press(
    &ButtonPressed(button): &ButtonPressed,
    world: &mut Entities,
    _events: &mut Events,
) {
    world.apply(|(Hero(), hero_state, Velocity(dx, dy))| match button {
        JoypadState::A => match hero_state {
            HeroState::GROUNDED => Velocity(dx, 325.0),
            _otherwise => Velocity(dx, dy)
            }
        _otherwise => Velocity(dx, dy),
    })
}

fn on_push(Push(entity_id, (_px, py)): &Push, world: &mut Entities, _events: &mut Events) {
    world.apply_to(entity_id, |Hero()| {
        if py > &0.0 {
            HeroState::GROUNDED
        } else {
            HeroState::AIRBORNE
        }
    });
}

fn update_sprite(_update: &AfterUpdate, world: &mut Entities, events: &mut Events) {
    world.apply(|(Hero(), facing, Velocity(dx, _))| {
        if dx > 0.0 {
            DirectionFacing::RIGHT
        } else if dx < 0.0 {
            DirectionFacing::LEFT
        } else {
            facing
        }
    });
    world.apply(
        |(Hero(), status, facing, Position(x, _y), Velocity(dx, dy))| match status {
            HeroState::GROUNDED => {
                if dx == 0.0 {
                    Sprite::sprite_ex("panda_stand", 10, flip(facing))
                } else {
                    let frame = (x as i32 / 8) % 4;
                    match frame {
                        0 => Sprite::sprite_ex("panda_run_1", 10, flip(facing)),
                        1 => Sprite::sprite_ex("panda_run_2", 10, flip(facing)),
                        2 => Sprite::sprite_ex("panda_run_3", 10, flip(facing)),
                        3 => Sprite::sprite_ex("panda_run_2", 10, flip(facing)),
                        _ => Sprite::sprite("error", 10),
                    }
                }
            },
            HeroState::AIRBORNE => {
                if dy > 0.0 {
                    Sprite::sprite_ex("panda_ascend", 10, flip(facing))
                } else {
                    Sprite::sprite_ex("panda_descend", 10, flip(facing))
                }
            }
        },
    );
}

fn flip(facing: DirectionFacing) -> bool {
    match facing {
        DirectionFacing::LEFT => true,
        DirectionFacing::RIGHT => false
    }
}

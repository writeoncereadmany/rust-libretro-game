use std::time::Duration;
use crate::app::application::{AfterUpdate, BeforeUpdate};
use crate::component::collisions::{Actor, Push};
use crate::component::graphics::Sprite;
use crate::component::physics::{Acceleration, Gravity, Position, Velocity, VelocityCap};
use derive::{Constant, Event, Variable};
use engine::entities::entity::{Entities, entity};
use engine::events::dispatcher::Dispatcher;
use engine::events::event::Events;
use engine::events::input::{ButtonPressed, InputState};
use engine::events::spawner::Spawner;
use engine::shapes::shape::Shape;
use rust_libretro::types::JoypadState;
use crate::entities::radial::SpawnRadials;

const RUN_ACCEL: f64 = 500.0;
const SKID_ACCEL: f64 = 800.0;
const SLOW_ACCEL: f64 = 250.0;
const STATIC_FRICTION_THRESHOLD: f64 = 5.0;
const ASCENT_DURATION: f64 = 0.15;
const POST_JUMP_ACCEL: f64 = 1500.0;
const WALL_STICK: f64 = 100.0;

#[derive(Constant, Clone)]
struct Hero();

#[derive(Variable, Clone)]
struct AscentRemaining(f64);

#[derive(Variable, Clone)]
enum HeroState {
    Grounded,
    Airborne,
    WallDragLeft,
    WallDragRight
}

#[derive(Variable, Clone)]
enum DirectionFacing {
    LEFT,
    RIGHT,
}

#[derive(Variable, Clone, Debug)]
enum MovementIntent {
    LEFT,
    NEUTRAL,
    RIGHT
}

#[derive(Event)]
struct SpawnHero(f64, f64);

#[derive(Event)]
struct SpawnRadialAndDelayedHero(f64, f64);

#[derive(Event)]
struct Jump();

#[derive(Event)]
struct WallJump(DirectionFacing);

pub fn register(dispatcher: &mut Dispatcher, spawner: &mut Spawner) {
    dispatcher.register(spawn_hero);
    dispatcher.register(spawn_radial_and_delayed_hero);
    dispatcher.register(listen_to_input_state);
    dispatcher.register(listen_to_button_press);
    dispatcher.register(jump);
    dispatcher.register(wall_jump);
    dispatcher.register(post_jump);
    dispatcher.register(check_static_friction);
    dispatcher.register(apply_movement);
    dispatcher.register(on_push);
    dispatcher.register(clamp_to_screen);
    dispatcher.register(update_sprite);

    spawner.register("Hero", |spawn, events| {
        events.fire(SpawnRadialAndDelayedHero(spawn.x, spawn.y))
    });
}

fn spawn_radial_and_delayed_hero(&SpawnRadialAndDelayedHero(x, y): &SpawnRadialAndDelayedHero, _world: &mut Entities, events: &mut Events) {
    events.fire(SpawnRadials(x, y, vec!["ball_blue", "ball_white"], 8));
    events.schedule(Duration::from_secs_f64(2.4), SpawnHero(x, y));
}

fn spawn_hero(&SpawnHero(x, y): &SpawnHero, world: &mut Entities, _events: &mut Events) {
    world.spawn(
        entity()
            .with(Hero())
            .with(HeroState::Grounded)
            .with(DirectionFacing::RIGHT)
            .with(MovementIntent::NEUTRAL)
            .with(Gravity())
            .with(Actor)
            .with(Sprite::sprite("panda_stand", 10))
            .with(Shape::bbox(0.0, 0.0, 12.0, 12.0))
            .with(Acceleration(0.0, 0.0))
            .with(Velocity(0.0, 0.0))
            .with(VelocityCap(200.0, f64::INFINITY))
            .with(Position(x, y)),
    );
}

fn listen_to_input_state(
    &InputState(joypad): &InputState,
    world: &mut Entities,
    _events: &mut Events,
) {
    world.apply(|Hero()| {
        match (
            joypad.contains(JoypadState::LEFT),
            joypad.contains(JoypadState::RIGHT),
        ) {
            (true, false) => MovementIntent::LEFT,
            (false, true) => MovementIntent::RIGHT,
            _otherwise => MovementIntent::NEUTRAL,
        }
    });
    world.apply(|(Hero(), asc@AscentRemaining(_))| {
       if joypad.contains(JoypadState::A) { Some(asc) } else { None }
    });
}

fn listen_to_button_press(
    &ButtonPressed(button): &ButtonPressed,
    world: &mut Entities,
    events: &mut Events,
) {
    world.apply(|(Hero(), hero_state)| match button {
        JoypadState::A => match hero_state {
            HeroState::Grounded => events.fire(Jump()),
            HeroState::WallDragLeft => events.fire(WallJump(DirectionFacing::RIGHT)),
            HeroState::WallDragRight => events.fire(WallJump(DirectionFacing::LEFT)),
            _otherwise => (),
            }
        _otherwise => (),
    })
}

fn jump(
    _: &Jump,
    world: &mut Entities,
    _events: &mut Events,
) {
    world.apply(|(Hero(), Velocity(dx, _dy))| {
        (Velocity(dx, 150.0), AscentRemaining(ASCENT_DURATION))
    })
}

fn wall_jump(
    WallJump(facing): &WallJump,
    world: &mut Entities,
    _events: &mut Events,
) {
    world.apply(|(Hero(), Velocity(dx, _dy))| {
        match facing {
            DirectionFacing::LEFT => (Velocity(-200.0, 150.0), AscentRemaining(ASCENT_DURATION)),
            DirectionFacing::RIGHT => (Velocity(200.0, 150.0), AscentRemaining(ASCENT_DURATION))
        }
    })
}

fn post_jump(
    dt: &Duration,
    world: &mut Entities,
    _events: &mut Events,
) {
    world.apply(|(Hero(), AscentRemaining(at), acc@Acceleration(ddx, ddy))| {
        if at > 0.0 {
            (Some(AscentRemaining(at - dt.as_secs_f64())), Acceleration(ddx, ddy + POST_JUMP_ACCEL))
        } else {
            (None, acc)
        }
    })
}


fn check_static_friction(_: &BeforeUpdate, world: &mut Entities, _events: &mut Events) {
    world.apply(|(Hero(), movement_intent, Velocity(dx, dy))| {
        match movement_intent {
            MovementIntent::NEUTRAL => {
                if dx.abs() < STATIC_FRICTION_THRESHOLD {
                    Velocity(0.0, dy)
                } else {
                    Velocity(dx, dy)
                }
            }
            _otherwise => Velocity(dx, dy)
        }
    });
}

fn apply_movement(_: &BeforeUpdate, world: &mut Entities, _events: &mut Events)
{
    world.apply(
        |(Hero(), movement_intent, hero_state, Acceleration(ddx, ddy), Velocity(dx, _))|
            {
                let h_accel = match (hero_state, movement_intent) {
                    (HeroState::WallDragLeft, MovementIntent::LEFT) => {
                        -RUN_ACCEL
                    },
                    (HeroState::WallDragLeft, MovementIntent::RIGHT) => {
                        -WALL_STICK
                    },
                    (HeroState::WallDragLeft, MovementIntent::NEUTRAL) => {
                        RUN_ACCEL
                    },
                    (HeroState::WallDragRight, MovementIntent::LEFT) => {
                        -RUN_ACCEL
                    },
                    (HeroState::WallDragRight, MovementIntent::RIGHT) => {
                        WALL_STICK
                    },
                    (HeroState::WallDragRight, MovementIntent::NEUTRAL) => {
                        RUN_ACCEL
                    },
                    (_otherwise, MovementIntent::LEFT) =>
                                if dx > 0.0 { -SKID_ACCEL } else { -RUN_ACCEL },
                    (_otherwise, MovementIntent::RIGHT) =>
                                if dx < 0.0 { SKID_ACCEL } else { RUN_ACCEL },
                    (_otherwise, MovementIntent::NEUTRAL) =>
                                if dx > 0.0 { -SLOW_ACCEL } else if dx < 0.0 { SLOW_ACCEL } else { 0.0 },
                    };
                Acceleration(ddx + h_accel, ddy)
            }
    );
}

fn on_push(Push(entity_id, (px, py)): &Push, world: &mut Entities, _events: &mut Events) {
    world.apply_to(entity_id, |Hero()| {
        if py > &0.0 {
            HeroState::Grounded
        } else if px < &0.0 {
            HeroState::WallDragRight
        } else if px > &0.0 {
            HeroState::WallDragLeft
        } else {
            HeroState::Airborne
        }
    });
}

fn clamp_to_screen(_: &AfterUpdate, world: &mut Entities, _events: &mut Events) {
    world.apply(|(Hero(), pos@Position(x, y), vel@Velocity(_, dy))| {
        if x < 0.0 || x > 324.0 {
            (Position(x.clamp(0.0, 324.0), y), Velocity(0.0, dy))
        } else {
            (pos, vel)
        }
    })
}

fn update_sprite(_update: &AfterUpdate, world: &mut Entities, _events: &mut Events) {
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
        |(Hero(), status, facing, movement_intent, Position(x, _y), Velocity(dx, dy))| match status {
            HeroState::Grounded => {
                if dx == 0.0 {
                    Sprite::sprite_ex("panda_stand", 10, flip(&facing))
                } else {
                    if turning(&facing, &movement_intent) {
                        Sprite::sprite_ex("panda_skid", 10, flip(&facing))
                    }
                    else {
                        let frame = (x as i32 / 8) % 4;
                        match frame {
                            0 => Sprite::sprite_ex("panda_run_1", 10, flip(&facing)),
                            1 => Sprite::sprite_ex("panda_run_2", 10, flip(&facing)),
                            2 => Sprite::sprite_ex("panda_run_3", 10, flip(&facing)),
                            3 => Sprite::sprite_ex("panda_run_2", 10, flip(&facing)),
                            _ => Sprite::sprite("error", 10)
                        }
                    }
                }
            },
            HeroState::Airborne => {
                if dy > 0.0 {
                    Sprite::sprite_ex("panda_ascend", 10, flip(&facing))
                } else {
                    Sprite::sprite_ex("panda_descend", 10, flip(&facing))
                }
            },
            HeroState::WallDragLeft => {
                Sprite::sprite_ex("panda_wallslide", 10, false)
            },
            HeroState::WallDragRight => {
                Sprite::sprite_ex("panda_wallslide", 10, true)
            }
        },
    );
}

fn turning(facing: &DirectionFacing, movement_intent: &MovementIntent) -> bool {
    match (movement_intent, facing) {
        (MovementIntent::LEFT, DirectionFacing::RIGHT) => true,
        (MovementIntent::RIGHT, DirectionFacing::LEFT) => true,
        _otherwise => false
    }
}

fn flip(facing: &DirectionFacing) -> bool {
    match facing {
        DirectionFacing::LEFT => true,
        DirectionFacing::RIGHT => false
    }
}
use crate::app::pandamonium::{AfterUpdate, BeforeUpdate};
use crate::component::physics::{Acceleration, Position, Velocity};
use crate::entities::hero::{DirectionFacing, Hero, HeroState, MovementIntent};
use crate::entities::spring::Sprung;
use crate::game::game::Failed;
use derive::{system, Event, Variable};
use engine::entities::entity::Entities;
use engine::events::event::Events;
use engine::events::input::{ButtonPressed, InputState};
use rust_libretro::types::JoypadState;
use std::time::Duration;
use crate::entities::bubble::Bounce;

const RUN_ACCEL: f64 = 500.0;
const SKID_ACCEL: f64 = 1200.0;
const SLOW_ACCEL: f64 = 500.0;
const STATIC_FRICTION_THRESHOLD: f64 = 5.0;
const ASCENT_DURATION: f64 = 0.15;
const POST_JUMP_ACCEL: f64 = 1500.0;
const WALL_STICK: f64 = 100.0;
const BUOYANCY: f64 = 2400.0;
const WALL_DRAG_COEFFICIENT: f64 = -10.0;
const COYOTE_TIME: f64 = 0.07;

#[derive(Variable, Clone)]
struct AscentRemaining(f64);

#[derive(Variable, Clone)]
struct PostJump(f64, f64);

#[derive(Variable, Clone)]
struct CoyoteTime(HeroState, f64);

#[derive(Event)]
pub struct Jump();

#[derive(Event)]
pub struct Swim();

#[derive(Event)]
struct WallJump(DirectionFacing);

#[system]
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
    world.apply(|(Hero(), asc @ AscentRemaining(_))| {
        if joypad.contains(JoypadState::A) {
            Some(asc)
        } else {
            None
        }
    });
}

#[system]
fn listen_to_button_press(
    &ButtonPressed(button): &ButtonPressed,
    world: &mut Entities,
    events: &mut Events,
) {
    world.apply(|(Hero(), hero_state, maybe_ct)| match button {
        JoypadState::A => match (hero_state, maybe_ct) {
            (HeroState::Grounded, _) | (_, Some(CoyoteTime(HeroState::Grounded, _))) => {
                events.fire(Jump())
            }
            (HeroState::WallDragLeft, _) | (_, Some(CoyoteTime(HeroState::WallDragLeft, _))) => {
                events.fire(WallJump(DirectionFacing::RIGHT))
            }
            (HeroState::WallDragRight, _) | (_, Some(CoyoteTime(HeroState::WallDragRight, _))) => {
                events.fire(WallJump(DirectionFacing::LEFT))
            }
            (HeroState::Submerged, _) => events.fire(Swim()),
            _otherwise => (),
        },
        _otherwise => (),
    })
}

#[system]
fn jump(_: &Jump, world: &mut Entities, _events: &mut Events) {
    world.apply(|(Hero(), Velocity(dx, _dy))| {
        (
            Velocity(dx, 150.0),
            AscentRemaining(ASCENT_DURATION),
            PostJump(0.0, POST_JUMP_ACCEL),
            HeroState::Airborne,
            None::<CoyoteTime>,
        )
    })
}

#[system]
fn bounce(_: &Bounce, world: &mut Entities, _events: &mut Events) {
    world.apply(|(Hero(), Velocity(dx, _dy))| {
        (
            Velocity(dx, 150.0),
            AscentRemaining(ASCENT_DURATION),
            PostJump(0.0, POST_JUMP_ACCEL),
            HeroState::Airborne,
            None::<CoyoteTime>,
        )
    })
}

#[system]
fn swim(_: &Swim, world: &mut Entities, _events: &mut Events) {
    world.apply(|(Hero(), Velocity(dx, dy))| {
        (
            Velocity(dx, (dy - 150.0).clamp(-200.0, 0.0)),
            AscentRemaining(ASCENT_DURATION),
            PostJump(0.0, -POST_JUMP_ACCEL),
        )
    })
}

#[system]
fn sprung(Sprung(id): &Sprung, world: &mut Entities, _events: &mut Events) {
    world.apply_to(id, |(Hero(), Velocity(dx, _dy))| {
        (
            Velocity(dx, 500.0),
            AscentRemaining(0.0),
            HeroState::Airborne,
            None::<CoyoteTime>,
        )
    })
}

#[system]
fn wall_jump(WallJump(facing): &WallJump, world: &mut Entities, _events: &mut Events) {
    world.apply(|(Hero(), Velocity(_dx, _dy))| match facing {
        DirectionFacing::LEFT => (
            Velocity(-200.0, 150.0),
            AscentRemaining(ASCENT_DURATION),
            PostJump(-POST_JUMP_ACCEL, POST_JUMP_ACCEL),
            HeroState::Airborne,
            None::<CoyoteTime>,
        ),
        DirectionFacing::RIGHT => (
            Velocity(200.0, 150.0),
            AscentRemaining(ASCENT_DURATION),
            PostJump(POST_JUMP_ACCEL, POST_JUMP_ACCEL),
            HeroState::Airborne,
            None::<CoyoteTime>,
        ),
    })
}

#[system]
fn post_jump(dt: &Duration, world: &mut Entities, _events: &mut Events) {
    world.apply(
        |(Hero(), AscentRemaining(at), PostJump(pddx, pddy), acc @ Acceleration(ddx, ddy))| {
            if at > 0.0 {
                (
                    Some(AscentRemaining(at - dt.as_secs_f64())),
                    Acceleration(ddx + pddx, ddy + pddy),
                )
            } else {
                (None, acc)
            }
        },
    )
}

#[system]
fn coyote_time(dt: &Duration, world: &mut Entities, _events: &mut Events) {
    world.apply(|(hero_state, maybe_ct)| match hero_state {
        HeroState::Grounded => Some(CoyoteTime(HeroState::Grounded, COYOTE_TIME)),
        HeroState::WallDragRight => Some(CoyoteTime(HeroState::WallDragRight, COYOTE_TIME)),
        HeroState::WallDragLeft => Some(CoyoteTime(HeroState::WallDragLeft, COYOTE_TIME)),
        _ => {
            if let Some(CoyoteTime(current_state, ct)) = maybe_ct {
                let new_ct = ct - dt.as_secs_f64();
                if new_ct <= 0.0 {
                    None
                } else {
                    Some(CoyoteTime(current_state, new_ct))
                }
            } else {
                None
            }
        }
    });
}

#[system]
fn check_static_friction(_: &BeforeUpdate, world: &mut Entities, _events: &mut Events) {
    world.apply(
        |(Hero(), movement_intent, Velocity(dx, dy), Acceleration(ddx, ddy))| match movement_intent {
            MovementIntent::NEUTRAL => {
                if dx.abs() < STATIC_FRICTION_THRESHOLD {
                    (Velocity(0.0, dy), Acceleration(0.0, ddy))
                } else {
                    (Velocity(dx, dy), Acceleration(ddx, ddy))
                }
            }
            _otherwise => (Velocity(dx, dy), Acceleration(ddx, ddy))
        },
    );
}

#[system]
fn apply_movement(_: &BeforeUpdate, world: &mut Entities, _events: &mut Events) {
    world.apply(
        |(Hero(), movement_intent, hero_state, Acceleration(ddx, ddy), Velocity(dx, dy))| {
            let h_accel = match (&hero_state, movement_intent) {
                (HeroState::WallDragLeft, MovementIntent::LEFT) => -RUN_ACCEL,
                (HeroState::WallDragLeft, MovementIntent::RIGHT) => -WALL_STICK,
                (HeroState::WallDragLeft, MovementIntent::NEUTRAL) => RUN_ACCEL,
                (HeroState::WallDragRight, MovementIntent::LEFT) => -RUN_ACCEL,
                (HeroState::WallDragRight, MovementIntent::RIGHT) => WALL_STICK,
                (HeroState::WallDragRight, MovementIntent::NEUTRAL) => RUN_ACCEL,
                (_otherwise, MovementIntent::LEFT) => {
                    if dx > 0.0 {
                        -SKID_ACCEL
                    } else {
                        -RUN_ACCEL
                    }
                }
                (_otherwise, MovementIntent::RIGHT) => {
                    if dx < 0.0 {
                        SKID_ACCEL
                    } else {
                        RUN_ACCEL
                    }
                }
                (_otherwise, MovementIntent::NEUTRAL) => {
                    if dx > 0.0 {
                        -SLOW_ACCEL
                    } else if dx < 0.0 {
                        SLOW_ACCEL
                    } else {
                        0.0
                    }
                }
            };

            let y_accel = match &hero_state {
                HeroState::WallDragLeft | HeroState::WallDragRight => {
                    dy.min(0.0) * WALL_DRAG_COEFFICIENT
                }
                _otherwise => 0.0,
            };
            Acceleration(ddx + h_accel, ddy + y_accel)
        },
    );
}

#[system]
fn clamp_to_screen(_: &AfterUpdate, world: &mut Entities, events: &mut Events) {
    world.apply(|(Hero(), pos @ Position(x, y), vel @ Velocity(_, dy))| {
        if y < -12.0 {
            events.fire(Failed());
        }

        if !(0.0..=348.0).contains(&x) {
            (Position(x.clamp(0.0, 348.0), y), Velocity(0.0, dy))
        } else {
            (pos, vel)
        }
    })
}

#[system]
fn buoyancy(_: &BeforeUpdate, world: &mut Entities, _events: &mut Events) {
    world.apply(
        |(Hero(), hero_state, Acceleration(ddx, ddy))| match hero_state {
            HeroState::Submerged => Acceleration(ddx, ddy + BUOYANCY),
            _otherwise => Acceleration(ddx, ddy),
        },
    );
}
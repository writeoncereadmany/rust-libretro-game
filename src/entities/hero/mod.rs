use crate::component::graphics::Sprite;
use crate::component::physics::Position;
use derive::{Constant, Event, Variable};

mod movement;
mod graphics;
pub mod collisions;
pub mod spawns;



#[derive(Constant, Clone)]
pub struct Hero();

#[derive(Variable, Clone)]
enum HeroState {
    Grounded,
    Airborne,
    WallDragLeft,
    WallDragRight,
    Submerged,
}

#[derive(Variable, Clone)]
enum DirectionFacing {
    LEFT,
    RIGHT,
}

#[derive(Variable, Clone)]
enum MovementIntent {
    LEFT,
    NEUTRAL,
    RIGHT,
}

#[derive(Variable, Clone)]
pub struct Bubbles(f64);

#[derive(Event)]
struct SpawnHero(f64, f64);

#[derive(Event)]
struct SpawnRadialAndDelayedHero(f64, f64);

#[derive(Event)]
struct SpawnShade(Sprite, Position);







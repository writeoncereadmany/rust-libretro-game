use engine::assets::map::Map;
use engine::events::event::Events;
use engine::events::spawner::Spawner;

pub mod coin;
mod hero;
mod flag;
pub mod map;
pub mod radial;
mod crumbler;
mod lockbox;
mod key;
mod chest;
mod sparkle;
mod fruit;
pub mod spring;
pub mod bubble;
mod splash;
pub mod failureballs;

pub fn load_map(map: &Map, spawner: &Spawner, events: &mut Events) {
    map::load_map(map, spawner, events)
}
use engine::events::dispatcher::Dispatcher;
use engine::events::event::Events;
use engine::events::spawner::Spawner;

mod coin;
mod hero;
mod flag;
pub mod map;
mod radial;
mod crumbler;
mod lockbox;
mod key;

pub fn register(dispatcher: &mut Dispatcher, spawner: &mut Spawner) {
    coin::register(dispatcher, spawner);
    crumbler::register(dispatcher, spawner);
    flag::register(dispatcher, spawner);
    hero::register(dispatcher, spawner);
    key::register(dispatcher, spawner);
    lockbox::register(dispatcher, spawner);
    map::register(dispatcher, spawner);
    radial::register(dispatcher, spawner);
}

pub fn load_map(map: &tiled::Map, spawner: &Spawner, events: &mut Events) {
    map::load_map(map, spawner, events)
}
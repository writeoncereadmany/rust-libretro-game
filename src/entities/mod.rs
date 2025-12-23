use engine::events::dispatcher::Dispatcher;
use engine::events::event::Events;
use engine::events::spawner::Spawner;

mod coin;
mod map;

pub fn register(dispatcher: &mut Dispatcher, spawner: &mut Spawner) {
    coin::register(dispatcher, spawner);
}

pub fn load_map(map: &tiled::Map, spawner: &Spawner, events: &mut Events) {
    map::load_map(map, spawner, events)
}
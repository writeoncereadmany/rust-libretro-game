use engine::events::dispatcher::Dispatcher;
use engine::events::spawner::Spawner;
use crate::assets::map::Spawn;

mod coin;

pub fn register(dispatcher: &mut Dispatcher, spawner: &mut Spawner<Spawn>) {
    coin::register(dispatcher, spawner);
}
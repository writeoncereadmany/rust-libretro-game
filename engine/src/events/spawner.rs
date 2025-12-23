use crate::events::event::Events;
use std::collections::HashMap;
use tiled::Object;

pub struct Spawner {
    spawns: HashMap<String, fn(&tiled::Object, &mut Events)>
}

impl Spawner {
    pub fn new() -> Self {
        Spawner { spawns: HashMap::new() }
    }

    pub fn spawn(&self, object: &tiled::Object, events: &mut Events) {
        if let Some(user_type) = get_user_type(object) {
            self.spawns.get(&user_type).map(|f| f(object, events));
        }
    }

    pub fn register(&mut self, name: &str, spawner: fn(&tiled::Object, &mut Events)) {
        self.spawns.insert(name.to_string(), spawner);
    }
}

fn get_user_type(object: &Object) -> Option<String> {
    object.get_tile()?.get_tile()?.user_type.clone()
}
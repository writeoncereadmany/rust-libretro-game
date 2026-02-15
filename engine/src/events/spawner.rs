use crate::events::event::Events;
use std::collections::HashMap;
use tiled::Object;

pub struct Spawner {
    spawns: HashMap<String, fn(Spawn, &mut Events)>
}

pub struct Spawn<'a> {
    pub x: f64,
    pub y: f64,
    pub object: &'a Object<'a>
}

impl Spawner {
    pub fn new() -> Self {
        Spawner { spawns: HashMap::new() }
    }

    pub fn spawn(&self, object: &Object, events: &mut Events) {
        if let Some(user_type) = get_user_type(object) {
            let spawn = Spawn { x: object.x as f64, y: (19*12) as f64 - object.y as f64, object };
            self.spawns.get(&user_type).map(|f| f(spawn, events));
        }
    }

    pub fn register(&mut self, name: &str, spawner: fn(Spawn, &mut Events)) {
        self.spawns.insert(name.to_string(), spawner);
    }
}

fn get_user_type(object: &Object) -> Option<String> {
    object.get_tile()?.get_tile()?.user_type.clone()
}
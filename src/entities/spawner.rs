use crate::events::event::Events;
use std::collections::HashMap;

pub struct Spawner<T> {
    spawns: HashMap<String, fn(&T, &mut Events)>
}

impl <T> Spawner<T> {
    pub fn new() -> Self {
        Spawner { spawns: HashMap::new() }
    }

    pub fn spawn(&self, name: &str, object: &T, events: &mut Events) {
        self.spawns.get(name).map(|f| f(object, events));
    }

    pub fn register(&mut self, name: &str, spawner: fn(&T, &mut Events)) {
        self.spawns.insert(name.to_string(), spawner);
    }
}
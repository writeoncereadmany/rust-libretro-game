use std::collections::HashMap;
use crate::entities::entity::Entities;

pub struct Spawner<T> {
    spawns: HashMap<String, fn(&T, &mut Entities)>
}

impl <T> Spawner<T> {
    pub fn new() -> Self {
        Spawner { spawns: HashMap::new() }
    }

    pub fn spawn(&self, name: &str, object: &T, world: &mut Entities) {
        self.spawns.get(name).map(|f| f(object, world));
    }

    pub fn register(&mut self, name: &str, spawner: fn(&T, &mut Entities)) {
        self.spawns.insert(name.to_string(), spawner);
    }
}
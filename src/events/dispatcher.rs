use std::collections::HashMap;
use std::any::{Any, TypeId};
use super::event::{EventTrait, Events};
use crate::entities::entity::Entities;

pub struct Dispatcher {
    functions: HashMap<TypeId, Box<dyn Any>>
}

impl Dispatcher {
    pub fn new() -> Self {
        Dispatcher { functions: HashMap::new() }
    }

    pub fn register<Event: EventTrait + 'static>(&mut self, f: fn(&Event, &mut Entities, &mut Events)) {
        self.functions.entry(TypeId::of::<Event>())
            .or_insert(Box::new(Vec::<fn(&Event, &mut Entities, &mut Events)>::new()))
            .downcast_mut::<Vec<fn(&Event, &mut Entities, &mut Events)>>()
            .map(|fs| fs.push(f));
    }

    pub fn dispatch<Event: EventTrait + 'static>(&self, event: &Event, world: &mut Entities, events: &mut Events) {
        if let Some(functions) = self.functions.get(&TypeId::of::<Event>())
            .map(|fs| fs.downcast_ref::<Vec<fn(&Event, &mut Entities, &mut Events)>>())
            .flatten() {
            for function in functions {
                function(event, world, events);
            }
        }
    }
}
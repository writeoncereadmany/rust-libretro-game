use crate::entities::entity::Entities;
use crate::events::dispatcher::Dispatcher;
use crate::events::timer::{Timer, TimerId};
use std::any::Any;
use std::collections::{HashMap, VecDeque};
use std::time::Duration;

pub trait EventTrait {
    fn as_any(&self) -> &dyn Any;

    fn dispatch(&self, dispatcher: &Dispatcher, world: &mut Entities, events: &mut Events);
}

impl EventTrait for Duration {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn dispatch(&self, dispatcher: &Dispatcher, world: &mut Entities, events: &mut Events) {
        dispatcher.dispatch(self, world, events);
    }
}

pub struct Event(Box<dyn EventTrait>);

impl Event {
    pub fn new<E: EventTrait + 'static>(event: E) -> Self {
        Event(Box::new(event))
    }

    pub fn unwrap<E: EventTrait + 'static>(&self) -> Option<&E> {
        let Event(event) = self;
        event.as_any().downcast_ref()
    }

    pub fn apply<E: EventTrait + 'static, O>(&self, f: impl FnMut(&E) -> O) -> Option<O> {
        self.unwrap().map(f)
    }

    pub fn dispatch(&self, dispatcher: &Dispatcher, world: &mut Entities, events: &mut Events) {
        let Event(event) = self;
        event.dispatch(dispatcher, world, events);
    }
}

pub struct Events {
    events: VecDeque<Event>,
    timers: HashMap<&'static str, Timer>,
}

impl Events {
    pub fn new() -> Self {
        Events {
            events: VecDeque::new(),
            timers: HashMap::new(),
        }
    }

    pub fn schedule<E: EventTrait + 'static>(
        &mut self,
        timer_name: &'static str,
        fires_in: Duration,
        event: E,
    ) -> TimerId {
        if !self.timers.contains_key(timer_name) {
            self.timers.insert(timer_name, Timer::new());
        }
        self.timers.get_mut(timer_name).unwrap().schedule(fires_in, Event::new(event))
    }
    
    pub fn cancel(&mut self, timer_name: &'static str, timer_id: &TimerId) {
        self.timers.get_mut(timer_name).map(|t| t.cancel(timer_id));
    }

    pub fn clear_schedule(&mut self, timer_name: &'static str) {
        self.timers.get_mut(timer_name).map(|t| t.clear_schedule());
    }

    pub fn elapse(&mut self, timer_name: &'static str, dt: Duration) {
        if let Some(timer) = &mut self.timers.get_mut(timer_name) {
            timer.elapse(&dt, &mut self.events);
        };
    }

    pub fn fire<E: EventTrait + 'static>(&mut self, event: E) {
        self.events.push_back(Event::new(event));
    }

    pub fn fire_event(&mut self, event: Event) {
        self.events.push_back(event);
    }

    pub fn pop(&mut self) -> Option<Event> {
        self.events.pop_front()
    }
}

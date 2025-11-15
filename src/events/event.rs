use std::any::Any;
use std::collections::VecDeque;
use std::time::Duration;
use crate::entities::entity::Entities;
use crate::events::dispatcher::Dispatcher;
use crate::events::timer::Timer;

pub trait EventTrait {
    fn as_any(&self) -> &dyn Any;

    fn dispatch(&self, dispatcher: &super::dispatcher::Dispatcher, world: &mut crate::entities::entity::Entities, events: &mut Events);
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

    pub fn apply<E:EventTrait + 'static, O>(&self, f: impl FnMut(&E) -> O) -> Option<O> {
        self.unwrap().map(f)
    }
}

pub struct Events {
    events: VecDeque<Event>,
    timer: Timer
}

impl Events {

    pub fn new() -> Self {
        Events { events: VecDeque::new(), timer: Timer::new() }
    }

    pub fn schedule<E: EventTrait + 'static>(&mut self, fires_in: Duration, event: E) {
        self.timer.schedule(fires_in, Event::new(event));
    }

    pub fn clear_schedule(&mut self) {
        self.timer.clear_schedule();
    }

    pub fn elapse(&mut self, dt: Duration) {
        let timer = &mut self.timer;
        timer.elapse(&dt, &mut self.events);
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
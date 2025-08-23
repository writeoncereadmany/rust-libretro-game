use std::any::Any;
use std::collections::VecDeque;
use std::time::Duration;

pub trait EventTrait {
    fn as_any(&self) -> &dyn Any;
}

impl EventTrait for Duration {
    fn as_any(&self) -> &dyn Any {
        self
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
    events: VecDeque<Event>
}

impl Events {

    pub fn new() -> Self {
        Events { events: VecDeque::new() }
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
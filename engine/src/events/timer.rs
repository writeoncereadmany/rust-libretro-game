use crate::events::event::Event;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, VecDeque};
use std::time::{Duration, Instant};

struct TimerEvent {
    fires_at: Instant,
    event: Event
}

impl TimerEvent {
    fn from(instant: Instant, fire_in: Duration, event: Event) -> Self {
        TimerEvent {
            fires_at: instant + fire_in,
            event
        }
    }
}

impl Eq for TimerEvent {
}

impl PartialEq for TimerEvent {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}

impl Ord for TimerEvent {
    fn cmp(&self, other: &Self) -> Ordering {
        other.fires_at.cmp(&self.fires_at)
    }
}

impl PartialOrd for TimerEvent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Timer {
    current_time: Instant,
    scheduled_events: BinaryHeap<TimerEvent>
}

impl Timer {
    pub fn new() -> Self {
        Timer { current_time: Instant::now(), scheduled_events: BinaryHeap::new() }
    }

    pub fn schedule(&mut self, fires_in: Duration, event: Event) {
        self.scheduled_events.push(TimerEvent::from(self.current_time, fires_in, event));
    }

    pub fn clear_schedule(&mut self) {
        self.scheduled_events.clear();
    }

    pub fn elapse(&mut self, dt: &Duration, events: &mut VecDeque<Event>) {
        self.current_time += *dt;
        while self.has_pending_events() {
            if let Some(TimerEvent { event, .. }) = self.scheduled_events.pop() {
                events.push_back(event);
            } else {
                break;
            }
        }
    }

    fn has_pending_events(&self) -> bool {
        if let Some(next) = self.scheduled_events.peek() {
            next.fires_at < self.current_time
        }
        else {
            false
        }
    }
}

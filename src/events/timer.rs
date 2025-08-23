use derive::Event;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, VecDeque};
use std::time::{Duration, Instant};
use crate::events::event::{Event, EventTrait, Events};

#[derive(Event)]
struct ScheduleEvent {
    fire_in: Duration,
    event: fn() -> Event
}

struct TimerEvent {
    fires_at: Instant,
    event: Event
}

impl TimerEvent {
    fn from(instant: Instant, schedule: &ScheduleEvent) -> Self {
        TimerEvent {
            fires_at: instant + schedule.fire_in,
            event: (schedule.event)()
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

    pub fn on_event(&mut self, event: &Event) {
        event.apply(|schedule| self.scheduled_events.push(TimerEvent::from(self.current_time, schedule)));
    }

    pub fn clear_schedule(&mut self) {
        self.scheduled_events.clear();
    }

    pub fn elapse(&mut self, dt: &Duration, events: &mut Events) {
        self.current_time += *dt;
        while self.has_pending_events() {
            if let Some(TimerEvent { event, .. }) = self.scheduled_events.pop() {
                events.fire_event(event);
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

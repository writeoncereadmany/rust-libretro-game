use std::any::Any;
use rust_libretro::types::JoypadState;
use crate::entities::entity::Entities;
use crate::events::dispatcher::Dispatcher;
use crate::events::event::{EventTrait, Events};

pub struct ButtonPressed(pub JoypadState);

impl EventTrait for ButtonPressed {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn dispatch(&self, dispatcher: &Dispatcher, world: &mut Entities, events: &mut Events) {
        dispatcher.dispatch(self, world, events);
    }
}

pub struct ButtonReleased(pub JoypadState);

impl EventTrait for ButtonReleased {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn dispatch(&self, dispatcher: &Dispatcher, world: &mut Entities, events: &mut Events) {
        dispatcher.dispatch(self, world, events);
    }
}


pub struct InputState(pub JoypadState);

impl EventTrait for InputState {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn dispatch(&self, dispatcher: &Dispatcher, world: &mut Entities, events: &mut Events) {
        dispatcher.dispatch(self, world, events);
    }
}


pub fn fire_input_events(current_input: JoypadState, old_input: JoypadState, events: &mut Events) {
    events.fire(InputState(current_input));
    fire_presses_and_releases(JoypadState::LEFT, current_input, old_input, events);
    fire_presses_and_releases(JoypadState::RIGHT, current_input, old_input, events);
    fire_presses_and_releases(JoypadState::A, current_input, old_input, events);
    fire_presses_and_releases(JoypadState::START, current_input, old_input, events);
}

fn fire_presses_and_releases(
    button: JoypadState,
    current_input: JoypadState,
    old_input: JoypadState,
    events: &mut Events,
) {
    if current_input.contains(button) && !old_input.contains(button) {
        events.fire(ButtonPressed(button));
    } else if old_input.contains(button) && !current_input.contains(button) {
        events.fire(ButtonReleased(button))
    }
}
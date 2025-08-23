use derive::Event;
use rust_libretro::types::JoypadState;
use crate::events::event::Events;

#[derive(Event)]
pub struct ButtonPressed(pub JoypadState);

#[derive(Event)]
pub struct ButtonReleased(pub JoypadState);

#[derive(Event)]
pub struct InputState(pub JoypadState);

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
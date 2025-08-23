use crate::assets::assets::Assets;
use crate::events::event::{Event, Events};
use crate::events::input::fire_input_events;
use crate::renderer::renderer::Renderer;
use crate::screens::screen::Screen;
use crate::screens::title::TitleScreen;
use rust_libretro::contexts::AudioContext;
use rust_libretro::types::JoypadState;
use std::sync::Arc;
use std::time::Duration;
use derive::Event;
use crate::screens::game::{Game, StartLevel};

pub struct Application {
    assets: Arc<Assets>,
    previous_joypad_state: JoypadState,
    screen: Box<dyn Screen>,
}

#[derive(Event)]
pub struct StartGame;

#[derive(Event)]
pub struct GameOver;

impl Application {
    pub fn new(assets: Assets) -> Self {
        let assets = Arc::new(assets);
        Application {
            assets: assets.clone(),
            previous_joypad_state: JoypadState::empty(),
            screen: Box::new(TitleScreen::new(&assets)),
        }
    }

    pub fn update(&mut self, input: JoypadState, delta_time: u64) {
        let mut events = Events::new();

        events.fire(Duration::from_micros(delta_time));
        fire_input_events(input, self.previous_joypad_state, &mut events);

        while let Some(event) = events.pop() {
            self.on_event(&event, &mut events);
            self.screen.on_event(&event, &mut events);
        }

        self.previous_joypad_state = input;
    }

    pub fn draw(&mut self, renderer: &mut Renderer) {
        self.screen.draw(renderer);
    }

    pub fn play(&mut self, ctx: &mut AudioContext) {}

    fn on_event(&mut self, event: &Event, events: &mut Events) {
        event.apply(|StartGame| {
            self.screen = Box::new(Game::new(&self.assets));
            events.fire(StartLevel("start".to_string()));
        });
        event.apply(|GameOver| {
            self.screen = Box::new(TitleScreen::new(&self.assets))
        });
    }
}

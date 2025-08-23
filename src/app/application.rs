use crate::assets::assets::Assets;
use crate::events::event::{Event, Events};
use crate::renderer::renderer::Renderer;
use crate::screens::screen::Screen;
use crate::screens::title::TitleScreen;
use rust_libretro::contexts::AudioContext;
use rust_libretro::types::JoypadState;
use std::sync::Arc;

pub struct Application {
    assets: Arc<Assets>,
    previous_joypad_state: JoypadState,
    screen: Box<dyn Screen>
}

impl Application {
    pub fn new(assets: Assets) -> Self {
        let assets = Arc::new(assets);
        Application {
            assets: assets.clone(),
            previous_joypad_state: JoypadState::empty(),
            screen: Box::new(TitleScreen::new(assets))
        }
    }

    pub fn update(&mut self, input: JoypadState, delta_time: u32) {
        let mut events = Events::new();

        while let Some(event) = events.pop() {
            self.on_event(&event, &mut events);
            self.screen.on_event(&event, &mut events);
        }

        self.previous_joypad_state = input;
    }

    pub fn draw(&self, renderer: &mut Renderer) {
        self.screen.draw(renderer);
    }

    pub fn play(&mut self, ctx: &mut AudioContext) {
    }

    fn on_event(&mut self, event: &Event, events: &mut Events) {

    }
}
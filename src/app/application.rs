use crate::assets::assets::Assets;
use crate::game::game::{Game, StartLevel};
use crate::screens::screen::Screen;
use crate::screens::title::TitleScreen;
use derive::Event;
use engine::events::dispatcher::Dispatcher;
use engine::events::event::{Event, Events};
use engine::events::input::fire_input_events;
use engine::events::spawner::Spawner;
use engine::renderer::renderer::Renderer;
use rust_libretro::contexts::AudioContext;
use rust_libretro::types::JoypadState;
use std::sync::Arc;
use std::time::Duration;

pub struct Application {
    assets: Arc<Assets>,
    previous_joypad_state: JoypadState,
    dispatcher: Arc<Dispatcher>,
    spawner: Arc<Spawner>,
    screen: Box<dyn Screen>
}

#[derive(Event)]
pub struct StartGame;

#[derive(Event)]
pub struct GameOver;

impl Application {
    pub fn new(assets: Assets) -> Self {
        let assets = Arc::new(assets);

        let mut dispatcher = Dispatcher::new();
        let mut spawner = Spawner::new();

        crate::component::register(&mut dispatcher);
        crate::entities::register(&mut dispatcher, &mut spawner);

        Application {
            assets: assets.clone(),
            dispatcher: Arc::new(dispatcher),
            spawner: Arc::new(spawner),
            previous_joypad_state: JoypadState::empty(),
            screen: Box::new(TitleScreen::new(&assets))
        }
    }

    pub fn update(&mut self, input: JoypadState, delta_time: u64, events: &mut Events) {
        let dt = Duration::from_micros(delta_time);
        events.elapse(dt);
        events.fire(dt);
        fire_input_events(input, self.previous_joypad_state, events);

        while let Some(event) = events.pop() {
            self.on_event(&event, events);
            self.screen.on_event(&event, events);
        }

        self.previous_joypad_state = input;
    }

    pub fn draw(&mut self, renderer: &mut Renderer) {
        self.screen.draw(renderer);
    }

    pub fn play(&mut self, _ctx: &mut AudioContext) {}

    fn on_event(&mut self, event: &Event, events: &mut Events) {
        event.apply(|StartGame| {
            self.screen = Box::new(Game::new(&self.assets, self.dispatcher.clone(), self.spawner.clone()));
            events.fire(StartLevel("start".to_string()));
        });
        event.apply(|GameOver| {
            self.screen = Box::new(TitleScreen::new(&self.assets))
        });
    }
}

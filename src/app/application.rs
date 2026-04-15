use crate::game::game::{Game, StartLevel};
use crate::screens::screen::Screen;
use crate::screens::title::TitleScreen;
use derive::Event;
use engine::assets::Assets;
use engine::events::dispatcher::Dispatcher;
use engine::events::event::{Event, Events};
use engine::events::input::fire_input_events;
use engine::events::spawner::Spawner;
use engine::renderer::asset_renderer::AssetRenderer;
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
pub struct StartGame();

#[derive(Event)]
pub struct GameOver();

#[derive(Event)]
pub struct BeforeUpdate();

#[derive(Event)]
pub struct AfterUpdate();

impl Application {
    pub fn new(assets: Arc<Assets>) -> Self {
        let mut dispatcher = Dispatcher::new();
        let mut spawner = Spawner::new();

        crate::component::register(&mut dispatcher);
        crate::entities::register(&mut dispatcher, &mut spawner);

        Application {
            assets: assets.clone(),
            dispatcher: Arc::new(dispatcher),
            spawner: Arc::new(spawner),
            previous_joypad_state: JoypadState::empty(),
            screen: Box::new(TitleScreen::new())
        }
    }

    pub fn update(&mut self, input: JoypadState, delta_time: u64, renderer: &mut AssetRenderer, events: &mut Events) {
        let dt = Duration::from_micros(delta_time);

        events.fire(BeforeUpdate());
        self.process_events(renderer, events);

        fire_input_events(input, self.previous_joypad_state, events);
        self.process_events(renderer, events);

        events.elapse("Game", dt);
        self.process_events(renderer, events);

        events.fire(dt);
        self.process_events(renderer, events);

        events.fire(AfterUpdate());
        self.process_events(renderer, events);

        self.previous_joypad_state = input;
    }

    fn process_events(&mut self, renderer: &mut AssetRenderer, events: &mut Events) {
        while let Some(event) = events.pop() {
            renderer.on_event(&event, events);
            self.on_event(&event, events);
            self.screen.on_event(&event, events);
        }
    }

    pub fn draw(&mut self, renderer: &mut AssetRenderer) {
        self.screen.draw(renderer);
    }

    pub fn play(&mut self, _ctx: &mut AudioContext) {}

    fn on_event(&mut self, event: &Event, events: &mut Events) {
        event.apply(|StartGame()| {
            self.screen = Box::new(Game::new(&self.assets, self.dispatcher.clone(), self.spawner.clone()));
            events.fire(StartLevel("start".to_string()));
        });
        event.apply(|GameOver()| {
            self.screen = Box::new(TitleScreen::new())
        });
    }
}

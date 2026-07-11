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
use engine::retroarch::{Application, ApplicationProperties};
use rust_libretro::contexts::AudioContext;
use rust_libretro::input_descriptors;
use rust_libretro::sys::{retro_input_descriptor, RETRO_DEVICE_ID_JOYPAD_A, RETRO_DEVICE_ID_JOYPAD_DOWN, RETRO_DEVICE_ID_JOYPAD_LEFT, RETRO_DEVICE_ID_JOYPAD_RIGHT, RETRO_DEVICE_ID_JOYPAD_START, RETRO_DEVICE_ID_JOYPAD_UP, RETRO_DEVICE_JOYPAD};
use rust_libretro::types::JoypadState;
use std::sync::Arc;
use std::time::Duration;
use tracing_appender::non_blocking::WorkerGuard;

pub struct Pandamonium {
    assets: Arc<Assets>,
    previous_joypad_state: JoypadState,
    dispatcher: Arc<Dispatcher>,
    spawner: Arc<Spawner>,
    screen: Box<dyn Screen>,
    _logger_worker: Option<WorkerGuard>
}

#[derive(Event)]
pub struct StartGame();

#[derive(Event)]
pub struct GameOver();

#[derive(Event)]
pub struct BeforeUpdate();

#[derive(Event)]
pub struct AfterUpdate();

const INPUT_DESCRIPTORS: &[retro_input_descriptor] = &input_descriptors!(
    { 0, RETRO_DEVICE_JOYPAD, 0, RETRO_DEVICE_ID_JOYPAD_UP, "Up" },
    { 0, RETRO_DEVICE_JOYPAD, 0, RETRO_DEVICE_ID_JOYPAD_DOWN, "Down" },
    { 0, RETRO_DEVICE_JOYPAD, 0, RETRO_DEVICE_ID_JOYPAD_LEFT, "Left" },
    { 0, RETRO_DEVICE_JOYPAD, 0, RETRO_DEVICE_ID_JOYPAD_RIGHT, "Right" },
    { 0, RETRO_DEVICE_JOYPAD, 0, RETRO_DEVICE_ID_JOYPAD_A, "Jump" },
    { 0, RETRO_DEVICE_JOYPAD, 0, RETRO_DEVICE_ID_JOYPAD_START, "Start" },
);

impl Application for Pandamonium {
    fn new(assets: Arc<Assets>, logger_worker: Option<WorkerGuard>) -> Self {
        let mut dispatcher = Dispatcher::new();
        let mut spawner = Spawner::new();

        crate::component::register(&mut dispatcher);
        crate::entities::register(&mut dispatcher, &mut spawner);

        Pandamonium {
            assets: assets.clone(),
            dispatcher: Arc::new(dispatcher),
            spawner: Arc::new(spawner),
            previous_joypad_state: JoypadState::empty(),
            screen: Box::new(TitleScreen::new()),
            _logger_worker: logger_worker
        }
    }
    
    fn update(&mut self, input: JoypadState, delta_time: u64, renderer: &mut AssetRenderer, events: &mut Events) {
        let dt = Duration::from_micros(delta_time);

        fire_input_events(input, self.previous_joypad_state, events);
        self.process_events(renderer, events);

        events.fire(BeforeUpdate());
        self.process_events(renderer, events);

        events.elapse("Application", dt);
        self.process_events(renderer, events);

        events.fire(dt);
        self.process_events(renderer, events);

        events.fire(AfterUpdate());
        self.process_events(renderer, events);

        self.previous_joypad_state = input;
    }
    
    fn draw(&mut self, renderer: &mut AssetRenderer) {
        self.screen.draw(renderer);
    }

    fn play(&mut self, _ctx: &mut AudioContext) {}

    fn properties() -> ApplicationProperties {
        ApplicationProperties {
            width: 360,
            height: 240,
            name: "pandamonium".to_string(),
            input_descriptors: INPUT_DESCRIPTORS,
            extensions: &["panda"],
        }
    }
}

impl Pandamonium {
    fn process_events(&mut self, renderer: &mut AssetRenderer, events: &mut Events) {
        while let Some(event) = events.pop() {
            renderer.on_event(&event, events);
            self.on_event(&event, events);
            self.screen.on_event(&event, events);
        }
    }

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

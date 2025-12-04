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
use crate::assets::map::Spawn;
use crate::component::graphics::{Animation, Phase, Sprite};
use crate::component::physics::Position;
use crate::entities::entity::entity;
use crate::entities::spawner::Spawner;
use crate::events::dispatcher::Dispatcher;
use crate::game::game::{Game, StartLevel};

pub struct Application {
    assets: Arc<Assets>,
    previous_joypad_state: JoypadState,
    dispatcher: Arc<Dispatcher>,
    spawner: Arc<Spawner<Spawn>>,
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

        dispatcher.register(|dt: &Duration, world, entities| {
            world.apply(|(Animation { sprites, period }, Phase(p))| {
                let new_phase = p + (dt.as_secs_f64() / period) % 1.0;
                let new_sprite_index = (new_phase * sprites.len() as f64) as usize % sprites.len();
                (Phase(new_phase), Sprite(sprites[new_sprite_index]))
            })
        });

        let mut spawner = Spawner::<Spawn>::new();

        spawner.register("Coin", |spawn, world|
            {
                world.spawn(entity()
                    .with(Animation {
                        sprites: vec!["coin_1", "coin_2", "coin_3", "coin_4"],
                        period: 0.5,
                    })
                    .with(Phase(0.0))
                    .with(Sprite("coin_1"))
                    .with(Position(spawn.x as f64, spawn.y as f64)));
            }
        );

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

    pub fn play(&mut self, ctx: &mut AudioContext) {}

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

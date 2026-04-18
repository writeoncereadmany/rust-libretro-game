use crate::app::application::GameOver;
use crate::component::graphics::Sprite;
use crate::component::physics::Position;
use crate::entities::coin::Coin;
use crate::entities::load_map;
use crate::game::flashlamps::setup_flashlamps;
use crate::game::hud;
use crate::game::hud::{setup_hud, update_bonus};
use crate::screens::screen::Screen;
use derive::Event;
use engine::assets::Assets;
use engine::entities::entity::Entities;
use engine::events::dispatcher::Dispatcher;
use engine::events::event::{Event, Events};
use engine::events::input::ButtonPressed;
use engine::events::spawner::Spawner;
use engine::events::timer::TimerId;
use engine::renderer::asset_renderer::AssetRenderer;
use rust_libretro::types::JoypadState;
use std::sync::Arc;
use std::time::Duration;

const GAME_WINDOW_START_X: i32 = 12;
const GAME_WINDOW_TOP_Y: i32 = 19*12;

#[derive(Event)]
pub struct StartLevel(pub String);

#[derive(Event)]
pub struct IncreaseMultiplier();

#[derive(Event)]
pub struct BuyMetamultiplier();

#[derive(Event)]
pub struct BuyBonus();

#[derive(Event)]
pub struct CompleteLevel(pub String);

#[derive(Event)]
pub struct Failed();

#[derive(Event)]
pub struct Score(pub u32);

#[derive(Event)]
pub struct Pause();

#[derive(Event)]
pub struct Unpause();

pub struct Game {
    assets: Arc<Assets>,
    world: Entities,
    dispatcher: Arc<Dispatcher>,
    spawner: Arc<Spawner>,
    bonus: u32,
    metamultiplier: u32,
    score: u32,
    paused: bool,
    game_over_timer: TimerId,
    current_level: String
}

impl Game {
    pub fn new(assets: &Arc<Assets>, dispatcher: Arc<Dispatcher>, spawner: Arc<Spawner>) -> Self {
        Game {
            assets: assets.clone(),
            world: Entities::new(),
            dispatcher,
            spawner,
            bonus: 1,
            metamultiplier: 1,
            score: 0,
            paused: false,
            game_over_timer: TimerId::MAX,
            current_level: String::new()
        }
    }

    fn load_map(&mut self, map: &String, events: &mut Events) {
        events.clear_schedule("Game");
        events.fire(Unpause());

        self.world = Entities::new();

        match self.assets.maps.get(map) {
            Some(map) => load_map(map, &self.spawner, events),
            None => panic!("Map {map} could not be found")
        };

        self.current_level = map.clone();

        setup_flashlamps(events);
        setup_hud(events, &self.score, &self.bonus);
        self.game_over_timer = events.schedule("Game", Duration::from_secs_f64(12.4), Failed());
    }
}

impl Screen for Game {
    fn on_event(&mut self, event: &Event, events: &mut Events) {
        event.apply(|ButtonPressed(button)| {
            if button == &JoypadState::SELECT {
                self.paused = !self.paused;
            }
        });

        event.apply(|Pause()| { self.paused = true; });
        event.apply(|Unpause()| { self.paused = false; });
        event.apply(|StartLevel(map)| self.load_map(map, events));

        event.apply(|ButtonPressed(button)| {
            if button == &JoypadState::START {
                events.fire(GameOver())
            }
        });

        event.apply(|Score(score)| {
            self.score += score * self.bonus;
            hud::update_score(&self.score, events);
        });

        event.apply(|Failed()| {
            events.cancel("Application", &self.game_over_timer);
            events.fire(Pause());

            if self.bonus == 1 {
                events.schedule("Application", Duration::from_secs_f64(1.0), GameOver());
            }
            else {
                self.bonus = 1;
                events.schedule("Application", Duration::from_secs_f64(1.0), StartLevel(self.current_level.clone()));
            }
        });

        event.apply(|IncreaseMultiplier()| {
            self.bonus = (self.bonus + 1).clamp(1, 5);
            update_bonus(&self.bonus, events);
        });

        event.apply(|BuyMetamultiplier()| {
            if (self.bonus == 5) {
                self.metamultiplier += 1;
            }
            self.bonus = 1;
        });

        event.apply(|BuyBonus()| {
            match self.bonus {
                5 => events.fire(Score(10_000)),
                4 => events.fire(Score(5_000)),
                3 => events.fire(Score(2_000)),
                2 => events.fire(Score(1_000)),
                _otherwise => events.fire(Score(100))
            }
            self.bonus = 1;
        });

        event.apply(|CompleteLevel(map)| {
            events.cancel("Application", &self.game_over_timer);
            events.fire(Pause());
            events.schedule("Application", Duration::from_secs_f64(1.0), StartLevel(map.clone()));
        });

        if !self.paused
        {
            event.apply(|dt| events.elapse("Game", *dt));
            event.dispatch(&self.dispatcher, &mut self.world, events);
        }
    }

    fn draw(&mut self, renderer: &mut AssetRenderer) {
        renderer.clear_sprites();
        let mut sprites : Vec<(Sprite, Position)> = self.world.collect();
        sprites.sort_by(|(Sprite(_, l1, _), _), (Sprite(_, l2, _), _)| l1.cmp(l2));
        sprites.iter()
            .for_each(|(Sprite(sprite, _, flip_x), Position(x, y))| {
                renderer.draw_sprite(sprite, x.round() as i32 + GAME_WINDOW_START_X, y.round() as i32 + GAME_WINDOW_TOP_Y, *flip_x)
            });
    }
}

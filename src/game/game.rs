use crate::app::pandamonium::GameOver;
use crate::component::graphics::Sprite;
use crate::component::{lifecycle, physics, time};
use crate::entities::{failureballs, load_map, radial};
use crate::game::flashlamps::setup_flashlamps;
use crate::game::hud;
use crate::game::hud::{setup_hud, update_bonus, update_metamultiplier};
use crate::screens::screen::Screen;
use derive::{Constant, Event};
use engine::assets::Assets;
use engine::entities::entity::{entity, Entities};
use engine::events::dispatcher::Dispatcher;
use engine::events::event::{Event, Events};
use engine::events::input::ButtonPressed;
use engine::events::spawner::Spawner;
use engine::events::timer::TimerId;
use engine::renderer::asset_renderer::AssetRenderer;
use rust_libretro::types::JoypadState;
use std::sync::Arc;
use std::time::Duration;
use crate::component::physics::Position;
use crate::entities::failureballs::SpawnFailureBall;
use crate::entities::radial::SpawnBonusBalls;

const GAME_WINDOW_START_X: i32 = 12;
const GAME_WINDOW_TOP_Y: i32 = 12;

#[derive(Event)]
pub struct StartLevel(pub String);

#[derive(Event)]
pub struct IncreaseMultiplier();

#[derive(Event)]
pub struct ApplyMultiplier(u32);

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

#[derive(Clone, Constant)]
pub enum Character {
    Blu,
    Redd
}

#[derive(Clone, Constant)]
pub struct Options {
    pub character: Character
}

pub struct Game {
    assets: Arc<Assets>,
    world: Entities,
    effects: Entities,
    dispatcher: Arc<Dispatcher>,
    effects_dispatcher: Dispatcher,
    spawner: Arc<Spawner>,
    bonus: u32,
    metamultiplier: u32,
    score: u32,
    paused: bool,
    game_over_timer: TimerId,
    current_level: String,
}

impl Game {
    pub fn new(assets: &Arc<Assets>, dispatcher: Arc<Dispatcher>, spawner: Arc<Spawner>) -> Self {
        let mut effects_dispatcher = Dispatcher::new();
        effects_dispatcher.register(physics::simple_integrate);
        effects_dispatcher.register(radial::spawn_bonus_balls);
        effects_dispatcher.register(radial::radial_events);
        lifecycle::register(&mut effects_dispatcher);
        time::register(&mut effects_dispatcher);
        failureballs::register(&mut effects_dispatcher);

        Game {
            assets: assets.clone(),
            world: Entities::new(),
            effects: Entities::new(),
            dispatcher,
            effects_dispatcher,
            spawner,
            bonus: 1,
            metamultiplier: 1,
            score: 0,
            paused: false,
            game_over_timer: TimerId::MAX,
            current_level: String::new(),
        }
    }

    fn load_map(&mut self, map: &String, events: &mut Events) {
        events.clear_schedule("Game");
        events.fire(Unpause());

        self.world = Entities::new();
        self.effects = Entities::new();
        
        self.world.spawn(entity()
            .with(Options { character: Character::Redd }));

        match self.assets.maps.get(map) {
            Some(map) => load_map(map, &self.spawner, events),
            None => panic!("Map {map} could not be found")
        };

        self.current_level = map.clone();

        setup_flashlamps(events);
        setup_hud(events, &self.score, &self.bonus, &self.metamultiplier);
        self.game_over_timer = events.schedule("Game", Duration::from_secs_f64(12.4), Failed());
    }

    fn set_bonus(&mut self, bonus: u32, events: &mut Events) {
        self.bonus = bonus;
        update_bonus(&self.bonus, events);
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

        event.apply(|Score(score)| {
            self.score += score * self.bonus * self.metamultiplier;
            hud::update_score(&self.score, events);
        });

        event.apply(|Failed()| {
            events.cancel("Application", &self.game_over_timer);
            events.fire(Pause());

            drop_miniballs(self.bonus, events);

            if self.bonus == 1 {
                events.schedule("Application", Duration::from_secs_f64(1.5), GameOver());
            } else {
                self.set_bonus(1, events);
                events.schedule("Application", Duration::from_secs_f64(1.5), StartLevel(self.current_level.clone()));
            }
        });

        event.apply(|IncreaseMultiplier()| {
            match self.bonus {
                1 => events.fire(SpawnBonusBalls(140.0, 188.0, vec!["small_ball_red"], 5)),
                2 => events.fire(SpawnBonusBalls(142.0, 191.0, vec!["small_ball_orange"], 5)),
                3 => events.fire(SpawnBonusBalls(144.0, 194.0, vec!["small_ball_yellow"], 5)),
                4 => events.fire(SpawnBonusBalls(144.0, 197.0, vec!["small_ball_green"], 5)),
                _ => {}
            }
            events.schedule("Application", Duration::from_secs_f64(1.0), ApplyMultiplier((self.bonus + 1).clamp(1, 5)));
        });

        event.apply(|ApplyMultiplier(mult)| {
            self.set_bonus(*mult, events);
        });

        event.apply(|BuyMetamultiplier()| {
            if self.bonus == 5 {
                self.metamultiplier += 1;
                update_metamultiplier(&self.metamultiplier, events)
            }
            self.set_bonus(1, events);
        });

        event.apply(|BuyBonus()| {
            match self.bonus {
                5 => events.fire(Score(10_000)),
                4 => events.fire(Score(5_000)),
                3 => events.fire(Score(2_000)),
                2 => events.fire(Score(1_000)),
                _otherwise => events.fire(Score(100))
            }
            self.set_bonus(1, events);
        });

        event.apply(|CompleteLevel(map)| {
            events.cancel("Application", &self.game_over_timer);
            events.fire(Pause());
            events.schedule("Application", Duration::from_secs_f64(1.5), StartLevel(map.clone()));
        });

        if !self.paused
        {
            event.apply(|dt| events.elapse("Game", *dt));
            event.dispatch(&self.dispatcher, &mut self.world, events);
        }
        event.dispatch(&self.effects_dispatcher, &mut self.effects, events);
    }

    fn draw(&mut self, renderer: &mut AssetRenderer) {
        renderer.clear_sprites();
        draw_sprites(&mut self.world, renderer);
        renderer.draw_hud();
        draw_sprites(&mut self.effects, renderer);
    }
}

fn draw_sprites(entities: &mut Entities, renderer: &mut AssetRenderer) {
    let mut sprites: Vec<(Sprite, Position)> = entities.collect();
    sprites.sort_by(|(Sprite(_, l1, _), _), (Sprite(_, l2, _), _)| l1.cmp(l2));
    sprites.iter()
        .for_each(|(Sprite(sprite, _, flip_x), Position(x, y))| {
            renderer.draw_sprite(sprite, x.round() as i32 + GAME_WINDOW_START_X, y.round() as i32 + GAME_WINDOW_TOP_Y, *flip_x)
        });
}

fn drop_miniballs(bonus: u32, events: &mut Events) {
    if bonus > 1 {
        for _ in 0..3 {
            events.fire(SpawnFailureBall { sprite: "small_ball_red".to_string(), dx: rand::random_range(-200.0..200.0), position: (140.0, 188.0) });
        }
    }
    if bonus > 2 {
        for _ in 0..3 {
            events.fire(SpawnFailureBall { sprite: "small_ball_orange".to_string(), dx: rand::random_range(-200.0..200.0), position: (142.0, 191.0) });
        }
    }
    if bonus > 3 {
        for _ in 0..3 {
            events.fire(SpawnFailureBall { sprite: "small_ball_yellow".to_string(), dx: rand::random_range(-200.0..200.0), position: (140.0, 194.0) });
        }
    }
    if bonus > 4 {
        for _ in 0..3 {
            events.fire(SpawnFailureBall { sprite: "small_ball_green".to_string(), dx: rand::random_range(-200.0..200.0), position: (140.0, 197.0) });
        }
    }
}
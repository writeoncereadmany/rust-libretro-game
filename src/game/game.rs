use crate::app::application::GameOver;
use crate::component::graphics::Sprite;
use crate::component::physics::Position;
use crate::entities::load_map;
use crate::game::flashlamps::setup_flashlamps;
use crate::game::hud;
use crate::game::hud::setup_hud;
use crate::screens::screen::Screen;
use derive::Event;
use engine::assets::Assets;
use engine::entities::entity::Entities;
use engine::events::dispatcher::Dispatcher;
use engine::events::event::{Event, Events};
use engine::events::input::ButtonPressed;
use engine::events::spawner::Spawner;
use engine::renderer::asset_renderer::AssetRenderer;
use rust_libretro::types::JoypadState;
use std::sync::Arc;

const GAME_WINDOW_START_X: i32 = 12;
const GAME_WINDOW_TOP_Y: i32 = 19*12;

#[derive(Event)]
pub struct StartLevel(pub String);

#[derive(Event)]
pub struct Score(pub u32);

pub struct Game {
    assets: Arc<Assets>,
    world: Entities,
    dispatcher: Arc<Dispatcher>,
    spawner: Arc<Spawner>,
    bonus: u32,
    score: u32
}

impl Game {
    pub fn new(assets: &Arc<Assets>, dispatcher: Arc<Dispatcher>, spawner: Arc<Spawner>) -> Self {
        Game {
            assets: assets.clone(),
            world: Entities::new(),
            dispatcher,
            spawner,
            bonus: 1,
            score: 0
        }
    }

    fn load_map(&mut self, map: &String, events: &mut Events) {
        events.clear_schedule();
        self.world = Entities::new();

        match self.assets.maps.get(map) {
            Some(map) => load_map(map, &self.spawner, events),
            None => panic!("Map {map} could not be found")
        };
        setup_flashlamps(events);
        setup_hud(events, self.score, self.bonus);
    }
}

impl Screen for Game {
    fn on_event(&mut self, event: &Event, events: &mut Events) {
        event.apply(|ButtonPressed(button)| {
            if button == &JoypadState::START {
                events.fire(GameOver())
            }
        });

        event.apply(|Score(score)| {
            self.score += score * self.bonus;
            hud::update_score(self.score, events);
        });

        event.apply(|StartLevel(map)| self.load_map(map, events));

        event.dispatch(&self.dispatcher, &mut self.world, events)
    }

    fn draw(&mut self, renderer: &mut AssetRenderer) {
        renderer.clear_sprites();
        let mut sprites : Vec<(Sprite, Position)> = self.world.collect();
        sprites.sort_by(|(Sprite(_, l1), _), (Sprite(_, l2), _)| l1.cmp(l2));
        sprites.iter()
            .for_each(|(Sprite(sprite, _), Position(x, y))| {
                renderer.draw_sprite(sprite, *x as i32 + GAME_WINDOW_START_X, *y as i32 + GAME_WINDOW_TOP_Y, false)
            });
    }
}

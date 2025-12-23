use crate::app::application::GameOver;
use crate::assets::assets::Assets;
use crate::assets::map::{Map, Spawn};
use crate::component::graphics::Sprite;
use crate::component::physics::Position;
use engine::entities::entity::Entities;
use engine::events::spawner::Spawner;
use engine::events::dispatcher::Dispatcher;
use engine::events::event::{Event, Events, EventTrait};
use engine::events::input::ButtonPressed;
use crate::game::flashlamps::setup_flashlamps;
use engine::renderer::renderer::Renderer;
use crate::screens::screen::Screen;
use derive::Event;
use rust_libretro::types::JoypadState;
use std::collections::VecDeque;
use std::sync::Arc;
use crate::entities::load_map;

#[derive(Event)]
pub struct StartLevel(pub String);

#[derive(Event)]
pub struct RedrawBackground;

#[derive(Event)]
pub struct UpdateBackgroundTile {
    pub x: i32,
    pub y: i32,
    pub sprite: Sprite,
}

enum RedrawBackgroundTask {
    RedrawBackground,
    UpdateBackgroundTile { x: i32, y: i32, sprite: Sprite },
}

pub struct Game {
    assets: Arc<Assets>,
    map: Option<Map>,
    world: Entities,
    dispatcher: Arc<Dispatcher>,
    spawner: Arc<Spawner>,
    render_tasks: VecDeque<RedrawBackgroundTask>,
}

impl Game {
    pub fn new(assets: &Arc<Assets>, dispatcher: Arc<Dispatcher>, spawner: Arc<Spawner>) -> Self {
        Game {
            assets: assets.clone(),
            map: None,
            world: Entities::new(),
            dispatcher,
            spawner,
            render_tasks: VecDeque::new(),
        }
    }

    fn load_map(&mut self, map: &String, events: &mut Events) {
        events.clear_schedule();

        match (self.assets.maps.get(map)) {
            Some(map) => load_map(map, &self.spawner, events),
            None => panic!("Map {map} could not be found")
        };
        events.fire(RedrawBackground);
        setup_flashlamps(&self.assets, events);
    }

    fn update_background(&mut self, renderer: &mut Renderer) {
        while let Some(task) = self.render_tasks.pop_front() {
            match task {
                RedrawBackgroundTask::RedrawBackground => {
                    self.map.as_ref().map(|map| map.draw_map(renderer, 12, 12));
                }
                RedrawBackgroundTask::UpdateBackgroundTile { x, y, sprite } => {
                    let Sprite(sprite) = sprite;
                    renderer.draw_background(self.assets.sprite(sprite), x, y);
                }
            };
        }
    }
}

impl Screen for Game {
    fn on_event(&mut self, event: &Event, events: &mut Events) {
        event.apply(|ButtonPressed(button)| {
            if button == &JoypadState::START {
                events.fire(GameOver)
            }
        });
        event.apply(|StartLevel(map)| self.load_map(map, events));
        event.apply(|UpdateBackgroundTile { x, y, sprite }| {
            self.render_tasks
                .push_back(RedrawBackgroundTask::UpdateBackgroundTile {
                    x: *x,
                    y: *y,
                    sprite: sprite.clone(),
                })
        });
        event.apply(|RedrawBackground| {
            self.render_tasks
                .push_back(RedrawBackgroundTask::RedrawBackground)
        });
        event.dispatch(&self.dispatcher, &mut self.world, events)
    }

    fn draw(&mut self, renderer: &mut Renderer) {
        self.update_background(renderer);
        renderer.clear_sprites();
        self.world
            .collect()
            .iter()
            .for_each(|(Sprite(sprite), Position(x, y))| {
                let r_sprite = self.assets.sprite(sprite);
                renderer.draw_sprite(r_sprite, *x as i32, *y as i32, false)
            });
    }
}

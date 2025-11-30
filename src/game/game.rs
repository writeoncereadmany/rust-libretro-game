use crate::app::application::GameOver;
use crate::assets::assets::Assets;
use crate::assets::map::Map;
use crate::events::event::{Event, Events};
use crate::events::input::ButtonPressed;
use crate::renderer::renderer::Renderer;
use crate::renderer::sprite::Sprite;
use crate::screens::screen::Screen;
use derive::Event;
use rust_libretro::types::JoypadState;
use std::collections::VecDeque;
use std::sync::Arc;
use std::time::Duration;
use crate::component::graphics::{Animation, Phase};
use crate::component::physics::Position;
use crate::entities::entity::{Entities, entity};
use crate::events::dispatcher::Dispatcher;
use crate::game::flashlamps::setup_flashlamps;

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
    dispatcher: Dispatcher,
    render_tasks: VecDeque<RedrawBackgroundTask>
}

impl Game {
    pub fn new(assets: &Arc<Assets>) -> Self {
        let mut dispatcher = Dispatcher::new();
        
        dispatcher.register(|dt: &Duration,  world, entities| {
            world.apply(|(Animation{ sprites, period}, Phase(p))|
                {
                    let new_phase = p + (dt.as_secs_f64() / period) % 1.0;
                    let new_sprite_index = (new_phase * sprites.len() as f64) as usize % sprites.len();
                    (Phase(new_phase), sprites[new_sprite_index].clone())
                })
        });
        
        Game {
            assets: assets.clone(),
            map: None,
            world: Entities::new(),
            dispatcher,
            render_tasks: VecDeque::new()
        }
    }

    fn load_map(&mut self, map: &String, events: &mut Events) {
        events.clear_schedule();

        self.map = self.assets.maps.get(map).map(|map| map.clone());
        events.fire(RedrawBackground);
        setup_flashlamps(&self.assets, events);

        self.map.as_mut().unwrap().spawns.iter().for_each(|spawn| {
            self.world.spawn(entity()
                .with(Animation { sprites: vec![
                    self.assets.sprite("coin_1"),
                    self.assets.sprite("coin_2"),
                    self.assets.sprite("coin_3"),
                    self.assets.sprite("coin_4"),
                ], period: 0.5}
                )
                .with(Phase(0.0))
                .with(self.assets.sprite("coin_1"))
                .with(Position(spawn.x as f64, spawn.y as f64)));
        });
    }

    fn update_background(&mut self, renderer: &mut Renderer) {
        while let Some(task) = self.render_tasks.pop_front() {
            match task {
                RedrawBackgroundTask::RedrawBackground => {
                    self.map.as_ref().map(|map| map.draw_map(renderer, 12, 12));
                }
                RedrawBackgroundTask::UpdateBackgroundTile{x, y, sprite} => {
                    renderer.draw_background(&sprite, x, y);
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
        event.apply(|UpdateBackgroundTile {x, y, sprite}|
            self.render_tasks
                .push_back(RedrawBackgroundTask::UpdateBackgroundTile {x: *x, y: *y, sprite: sprite.clone()}));
        event.apply(|RedrawBackground| {
            self.render_tasks
                .push_back(RedrawBackgroundTask::RedrawBackground)
        });
        event.dispatch(&self.dispatcher, &mut self.world, events)
    }

    fn draw(&mut self, renderer: &mut Renderer) {
        self.update_background(renderer);
        renderer.clear_sprites();
        self.world.collect().iter().for_each(|(sprite, Position(x, y))|
            renderer.draw_sprite(&sprite, *x as i32, *y as i32, false));
    }
}

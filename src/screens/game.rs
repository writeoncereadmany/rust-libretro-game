use crate::app::application::GameOver;
use crate::assets::assets::Assets;
use crate::assets::map::Map;
use crate::events::event::{Event, Events};
use crate::events::input::ButtonPressed;
use crate::events::timer::{EventFactory, ScheduleEvent, Timer};
use crate::renderer::renderer::Renderer;
use crate::renderer::sprite::Sprite;
use crate::screens::screen::Screen;
use derive::Event;
use rust_libretro::types::JoypadState;
use std::collections::VecDeque;
use std::sync::Arc;
use std::time::Duration;

#[derive(Event)]
pub struct StartLevel(pub String);

#[derive(Event)]
pub struct RedrawBackground;

#[derive(Event)]
pub struct UpdateBackgroundTile {
    x: i32,
    y: i32,
    sprite: Sprite,
}

enum RedrawBackgroundTask {
    RedrawBackground,
    UpdateBackgroundTile { x: i32, y: i32, sprite: Sprite },
}

struct LightBulb {
    x: i32,
    y: i32,
    sprite: Sprite
}

impl EventFactory for LightBulb {
    fn create(&self) -> Event {
        Event::new(UpdateBackgroundTile {
            x: self.x, y: self.y, sprite: self.sprite.clone()
        })
    }
}

pub struct Game {
    assets: Arc<Assets>,
    map: Option<Map>,
    render_tasks: VecDeque<RedrawBackgroundTask>,
    timer: Timer,
}

impl Game {
    pub fn new(assets: &Arc<Assets>) -> Self {
        Game {
            assets: assets.clone(),
            map: None,
            render_tasks: VecDeque::new(),
            timer: Timer::new(),
        }
    }

    fn load_map(&mut self, map: &String, events: &mut Events) {
        self.map = self.assets.maps.get(map).map(|map| map.clone());
        events.fire(RedrawBackground);
        self.spawn_hud(events);
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

    fn spawn_hud(&mut self, events: &mut Events) {
        let mut flashlamps: Vec<(i32, i32)> = Vec::new();
        for x in 17..30 {
            flashlamps.push((x, 0))
        }
        for y in 1..19 {
            flashlamps.push((29, y))
        }
        for x in 0..30 {
            flashlamps.push((29 - x, 19))
        }
        for y in 1..19 {
            flashlamps.push((0, 19 - y))
        }
        for x in 0..12 {
            flashlamps.push((x, 0))
        }

        for (i, (x, y)) in flashlamps.iter().enumerate() {
            let (x, y) = (x * 12, y * 12);
            let fraction_of_fulltime = i as f64 / flashlamps.len() as f64;
            let fire_in = Duration::from_secs_f64(2.4 + (10.0 * fraction_of_fulltime));
            let unlit = self.assets.tilesheets.get("Walls").unwrap().sprite(6, 4);
            let lit = self.assets.tilesheets.get("Walls").unwrap().sprite(7, 4);
            events.fire(UpdateBackgroundTile { x, y, sprite: unlit });
            events.fire(ScheduleEvent { fire_in, event: Box:: new(LightBulb {x, y, sprite: lit})});
        }
    }
}

impl Screen for Game {
    fn on_event(&mut self, event: &Event, events: &mut Events) {
        self.timer.on_event(event);
        event.apply(|dt| self.timer.elapse(dt, events));
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
    }

    fn draw(&mut self, renderer: &mut Renderer) {
        self.update_background(renderer);
        renderer.clear_sprites();
    }
}

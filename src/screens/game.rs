use crate::app::application::GameOver;
use crate::assets::assets::Assets;
use crate::assets::map::Map;
use crate::events::event::{Event, Events};
use crate::events::input::ButtonPressed;
use crate::renderer::renderer::Renderer;
use crate::screens::screen::Screen;
use derive::Event;
use rust_libretro::types::JoypadState;
use std::collections::VecDeque;
use std::sync::Arc;

#[derive(Event)]
pub struct StartLevel(pub String);

#[derive(Event)]
pub struct RedrawBackground;

enum RedrawBackgroundTask {
    RedrawBackground,
}

pub struct Game {
    assets: Arc<Assets>,
    map: Option<Map>,
    render_tasks: VecDeque<RedrawBackgroundTask>,
}

impl Game {
    pub fn new(assets: &Arc<Assets>) -> Self {
        Game {
            assets: assets.clone(),
            map: None,
            render_tasks: VecDeque::new(),
        }
    }

    fn load_map(&mut self, map: &String, events: &mut Events) {
        self.map = self.assets.maps.get(map).map(|map| map.clone());
        events.fire(RedrawBackground);
    }

    fn update_background(&mut self, renderer: &mut Renderer) {
        while let Some(task) = self.render_tasks.pop_front() {
            match task {
                RedrawBackgroundTask::RedrawBackground => {
                    self.map.as_ref().map(|map| map.draw_map(renderer, 12, 12));
                }
            };
        }
    }
}

impl Screen for Game {
    fn on_event(&mut self, event: &Event, events: &mut Events) {
        event.apply(|ButtonPressed(button)| if button == &JoypadState::START { events.fire(GameOver) });
        event.apply(|StartLevel(map)| {
            self.load_map(map, events)
        });
        event.apply(|RedrawBackground| self.render_tasks.push_back(RedrawBackgroundTask::RedrawBackground)); }

    fn draw(&mut self, renderer: &mut Renderer) {
        self.update_background(renderer);
        renderer.clear_sprites();
    }
}

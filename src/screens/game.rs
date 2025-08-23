use std::sync::Arc;
use rust_libretro::types::JoypadState;
use crate::app::application::GameOver;
use crate::assets::assets::Assets;
use crate::events::event::{Event, Events};
use crate::events::input::ButtonPressed;
use crate::renderer::renderer::Renderer;
use crate::screens::screen::Screen;

pub struct Game {
    assets: Arc<Assets>
}

impl Game {
    pub fn new(assets: &Arc<Assets>) -> Self {
        Game { assets: assets.clone() }
    }
}

impl Screen for Game {
    fn on_event(&mut self, event: &Event, events: &mut Events) {
        event.apply(|ButtonPressed(button)| if button == &JoypadState::START { events.fire(GameOver) });
    }

    fn draw(&self, renderer: &mut Renderer) {
        self.assets.maps.get("start").unwrap().draw_map(renderer, 12, 12);
        renderer.clear_sprites();

    }
}
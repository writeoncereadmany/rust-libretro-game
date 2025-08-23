use crate::assets::assets::Assets;
use crate::events::event::{Event, Events};
use crate::renderer::renderer::Renderer;
use crate::renderer::spritefont::Alignment;
use crate::renderer::spritefont::HorizontalAlignment::CENTER;
use crate::renderer::spritefont::VerticalAlignment::MIDDLE;
use crate::screens::screen::Screen;
use std::sync::Arc;
use rust_libretro::types::JoypadState;
use crate::app::application::StartGame;
use crate::events::input::ButtonPressed;

pub struct TitleScreen {
    assets: Arc<Assets>
}

impl TitleScreen {
    pub fn new(assets: &Arc<Assets>) -> Self {
        TitleScreen { assets: assets.clone() }
    }
}

impl Screen for TitleScreen {
    fn on_event(&mut self, event: &Event, events: &mut Events) {
        event.apply(|ButtonPressed(button)| if button == &JoypadState::START { events.fire(StartGame) });
    }

    fn draw(&self, renderer: &mut Renderer) {
        renderer.clear();
        renderer.draw_text(
            self.assets.fonts.get("Spritefont_Medium").unwrap(),
            "Pandamonium!",
            180,
            120,
            Alignment::aligned(CENTER, MIDDLE)
        );
    }
}
use crate::app::application::StartGame;
use crate::screens::screen::Screen;
use engine::events::event::{Event, Events};
use engine::events::input::ButtonPressed;
use engine::renderer::asset_renderer::AssetRenderer;
use engine::renderer::spritefont::Alignment;
use engine::renderer::spritefont::HorizontalAlignment::CENTER;
use engine::renderer::spritefont::VerticalAlignment::MIDDLE;
use rust_libretro::types::JoypadState;

pub struct TitleScreen {
}

impl TitleScreen {
    pub fn new() -> Self {
        TitleScreen { }
    }
}

impl Screen for TitleScreen {
    fn on_event(&mut self, event: &Event, events: &mut Events) {
        event.apply(|ButtonPressed(button)| {
            if button == &JoypadState::START {
                events.fire(StartGame())
            }
        });
    }

    fn draw(&mut self, renderer: &mut AssetRenderer) {
        renderer.clear();
        renderer.draw_text(
            "Pandamonium!",
            "Spritefont_Medium",
            180,
            120,
            Alignment::aligned(CENTER, MIDDLE),
        );
    }
}

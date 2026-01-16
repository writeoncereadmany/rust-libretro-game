use crate::app::application::StartGame;
use crate::screens::screen::Screen;
use engine::assets::Assets;
use engine::events::event::{Event, Events};
use engine::events::input::ButtonPressed;
use engine::renderer::asset_renderer::AssetRenderer;
use engine::renderer::renderer::Renderer;
use engine::renderer::spritefont::Alignment;
use engine::renderer::spritefont::HorizontalAlignment::CENTER;
use engine::renderer::spritefont::VerticalAlignment::MIDDLE;
use rust_libretro::types::JoypadState;
use std::sync::Arc;

pub struct TitleScreen {
    assets: Arc<Assets>,
}

impl TitleScreen {
    pub fn new(assets: &Arc<Assets>) -> Self {
        TitleScreen {
            assets: assets.clone(),
        }
    }
}

impl Screen for TitleScreen {
    fn on_event(&mut self, event: &Event, events: &mut Events) {
        event.apply(|ButtonPressed(button)| {
            if button == &JoypadState::START {
                events.fire(StartGame)
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

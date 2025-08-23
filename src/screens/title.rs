use crate::assets::assets::Assets;
use crate::events::event::{Event, Events};
use crate::renderer::renderer::Renderer;
use crate::renderer::spritefont::Alignment;
use crate::renderer::spritefont::HorizontalAlignment::CENTER;
use crate::renderer::spritefont::VerticalAlignment::MIDDLE;
use crate::screens::screen::Screen;
use std::sync::Arc;

pub struct TitleScreen {
    assets: Arc<Assets>
}

impl TitleScreen {
    pub fn new(assets: Arc<Assets>) -> Self {
        TitleScreen { assets: assets.clone() }
    }
}

impl Screen for TitleScreen {
    fn on_event(&mut self, _event: &Event, _events: &mut Events) {
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
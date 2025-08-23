use std::sync::Arc;
use rust_libretro::contexts::AudioContext;
use rust_libretro::types::JoypadState;
use crate::assets::assets::Assets;
use crate::renderer::renderer::Renderer;
use crate::renderer::spritefont::Alignment;
use crate::renderer::spritefont::HorizontalAlignment::CENTER;
use crate::renderer::spritefont::VerticalAlignment::MIDDLE;

pub struct Application {
    assets: Arc<Assets>
}

impl Application {
    pub fn new(assets: Assets) -> Self {
        Application {
            assets: Arc::new(assets)
        }
    }

    pub fn update(&mut self, input: JoypadState, delta_time: u32) {

    }

    pub fn draw(&self, renderer: &mut Renderer) {
        renderer.clear();
        renderer.draw_text(
            self.assets.fonts.get("Spritefont_Medium").unwrap(),
            "Pandamonium!",
            180,
            120,
            Alignment::aligned(CENTER, MIDDLE)
        );
    }

    pub fn play(&mut self, ctx: &mut AudioContext) {
    }
}
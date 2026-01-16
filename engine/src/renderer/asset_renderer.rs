use crate::assets::Assets;
use crate::renderer::renderer::Renderer;
use crate::renderer::spritefont::Alignment;
use rust_libretro::contexts::RunContext;
use std::sync::Arc;
use tiled::TileId;

pub struct AssetRenderer {
    renderer: Renderer,
    assets: Arc<Assets>,
}

impl AssetRenderer {
    pub fn new(renderer: Renderer, assets: Arc<Assets>) -> Self {
        AssetRenderer { renderer, assets }
    }

    pub fn draw_background(&mut self, tileset: &str, tile: TileId, x: i32, y: i32) {
        if let Some(tilesheet) = self.assets.tilesheets.get(tileset) {
            self.renderer.draw_background(&tilesheet.tile(tile), x, y);
        }
        else {
            self.draw_background_sprite("error", x, y);
        }
    }

    pub fn draw_background_sprite(&mut self, sprite: &str, x: i32, y: i32) {
        self.renderer.draw_background(self.assets.sprite(sprite), x, y);
    }

    pub fn draw_sprite(&mut self, sprite: &str, x: i32, y: i32, flip_x: bool) {
        self.renderer.draw_sprite(self.assets.sprite(sprite), x, y, flip_x);
    }

    pub fn draw_text(&mut self, text: &str, font: &str, x: i32, y: i32, alignment: Alignment) {
        self.renderer.draw_text(self.assets.fonts.get(font).unwrap(), text, x, y, alignment);
    }

    pub fn draw_background_text(&mut self, text: &str, font: &str, x: i32, y: i32, alignment: Alignment) {
        self.renderer.draw_background_text(self.assets.fonts.get(font).unwrap(), &text, x, y, alignment);
    }
    
    pub fn render(&mut self, ctx: &mut RunContext) {
        self.renderer.render(ctx);
    }

    pub fn clear(&mut self) {
        self.renderer.clear();
    }

    pub fn clear_sprites(&mut self) {
        self.renderer.clear_sprites();
    }
}

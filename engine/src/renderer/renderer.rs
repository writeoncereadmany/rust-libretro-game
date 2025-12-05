use crate::renderer::sprite::Sprite;
use crate::renderer::spritefont::{Alignment, SpriteFont};
use crate::renderer::texture::Texture;
use rust_libretro::contexts::RunContext;

pub struct Renderer {
    background: Texture,
    buffer: Texture
}

impl Renderer {
    pub fn new(width: u32, height: u32) -> Self {
        Renderer {
            background: Texture::new(width, height),
            buffer: Texture::new(width, height)
        }
    }

    pub fn clear(&mut self) {
        self.background.texture.fill(0);
        self.buffer.texture.fill(0);
    }

    pub fn clear_sprites(&mut self) {
        self.buffer.texture.copy_from_slice(&self.background.texture);
    }

    pub fn draw_background(&mut self, sprite: &Sprite, x: i32, y: i32) {
        sprite.draw_to(&mut self.background, x, y, false);
    }

    pub fn draw_sprite(&mut self, sprite: &Sprite, x: i32, y: i32, flip_x: bool) {
        sprite.draw_to(&mut self.buffer, x, y, flip_x);
    }

    pub fn draw_text(&mut self, font: &SpriteFont, text: &str, x: i32, y: i32, alignment: Alignment) {
        font.draw_text(&mut self.buffer, x, y, text, alignment);
    }

    pub fn render(&self, ctx: &mut RunContext) {
        self.buffer.render(ctx);
    }
}
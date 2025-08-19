use crate::renderer::tilesheet::Sprite;
use std::collections::HashMap;
use crate::renderer::texture::Texture;

pub struct SpriteFont {
    glyphs: HashMap<String, Sprite>,
    glyph_width: u32,
    glyph_height: u32,
    error_glyph: Sprite,
}

impl SpriteFont {
    pub fn new(
        glyphs: HashMap<String, Sprite>,
        glyph_width: u32,
        glyph_height: u32,
        error_glyph: Sprite,
    ) -> Self {
        SpriteFont {
            glyphs,
            glyph_width,
            glyph_height,
            error_glyph
        }
    }

    pub fn draw_text(&self, dst: &mut Texture, x: i32, y: i32, text: &str) {
        let mut next_x = x;
        for glyph in text.chars() {
            let sprite = self.glyphs.get(&glyph.to_string());
            sprite.unwrap_or(&self.error_glyph).draw_to(dst, next_x, y);
            next_x += self.glyph_width as i32;
        }
    }
}

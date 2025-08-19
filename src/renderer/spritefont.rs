use crate::renderer::tilesheet::Sprite;
use std::collections::HashMap;

pub struct SpriteFont {
    glyphs: HashMap<String, Sprite>,
    glyph_width: u32,
    glyph_height: u32,
}

impl SpriteFont {
    pub fn new(glyphs: HashMap<String, Sprite>, glyph_width: u32, glyph_height: u32) -> Self {
        SpriteFont {
            glyphs,
            glyph_width,
            glyph_height,
        }
    }
}

use std::collections::HashMap;
use crate::renderer::tilesheet::Sprite;

pub struct SpriteFont {
    glyphs: HashMap<String, Sprite>,
    glyph_width : u32,
    glyph_height: u32
}
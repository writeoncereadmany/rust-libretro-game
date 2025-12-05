use crate::renderer::sprite::Sprite;
use crate::renderer::texture::Texture;
use std::collections::HashMap;

pub enum HorizontalAlignment {
    LEFT,
    CENTER,
    RIGHT,
}

pub enum VerticalAlignment {
    TOP,
    MIDDLE,
    BOTTOM,
}

pub struct Alignment {
    horizontal_alignment: HorizontalAlignment,
    vertical_alignment: VerticalAlignment,
}

impl Alignment {
    pub fn aligned(
        horizontal_alignment: HorizontalAlignment,
        vertical_alignment: VerticalAlignment,
    ) -> Self {
        Alignment {
            horizontal_alignment,
            vertical_alignment,
        }
    }
}

pub struct SpriteFont {
    glyphs: HashMap<char, Sprite>,
    glyph_width: u32,
    glyph_height: u32,
    error_glyph: Sprite,
}

impl SpriteFont {
    pub fn new(
        glyphs: HashMap<char, Sprite>,
        glyph_width: u32,
        glyph_height: u32,
        error_glyph: Sprite,
    ) -> Self {
        SpriteFont {
            glyphs,
            glyph_width,
            glyph_height,
            error_glyph,
        }
    }

    pub fn draw_text(&self, dst: &mut Texture, x: i32, y: i32, text: &str, alignment: Alignment) {
        let text_width = (text.len() as u32 * self.glyph_width) as i32;

        let mut next_x = match alignment.horizontal_alignment {
            HorizontalAlignment::LEFT => x,
            HorizontalAlignment::CENTER => x - (text_width / 2),
            HorizontalAlignment::RIGHT => x - text_width
        };

        let y = match alignment.vertical_alignment {
            VerticalAlignment::TOP => y,
            VerticalAlignment::MIDDLE => y + (self.glyph_height / 2) as i32,
            VerticalAlignment::BOTTOM => y + self.glyph_height as i32,
        };

        for glyph in text.chars() {
            let sprite = self.glyphs.get(&glyph);
            sprite.unwrap_or(&self.error_glyph).draw_to(dst, next_x, y, false);
            next_x += self.glyph_width as i32;
        }
    }
}

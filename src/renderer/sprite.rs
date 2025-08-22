use crate::renderer::texture::Texture;
use crate::renderer::tilesheet::TileSheet;
use std::sync::Arc;

#[derive(Clone)]
pub struct Sprite {
    pub tile_sheet: Arc<TileSheet>,
    pub bounds: Bounds,
}

#[derive(Clone)]
pub struct Bounds {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl Sprite {
    pub fn draw_to(&self, dst: &mut Texture, x: i32, y: i32) {
        let sheet = &self.tile_sheet;

        let src_x = self.bounds.x as i32;
        let src_y = self.bounds.y as i32;

        let dst_x = x;
        let dst_y = y;

        let src = &sheet.tile_sheet;
        let palette = &sheet.palette;

        let min_x = 0.max(-dst_x);
        let min_y = 0.max(-dst_y);
        let max_x = (self.bounds.width as i32).min(dst.width as i32 - dst_x);
        let max_y = (self.bounds.height as i32).min(dst.height as i32 - dst_y);

        if min_y > max_y || min_x > max_x {
            return;
        }

        for y in min_y..max_y {
            let src_pixel = src_x + ((src_y + y) * sheet.width() as i32);
            let dst_pixel = dst_x + ((dst_y + y) * dst.width as i32);
            for x in min_x..max_x {
                let pixel = src[(src_pixel + x) as usize];
                if pixel != 0 {
                    dst.texture[(dst_pixel + x) as usize] = palette[pixel as usize];
                }
            }
        }
    }
}

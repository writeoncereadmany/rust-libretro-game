use rust_libretro::contexts::RunContext;
use std::slice;

pub struct Texture {
    pub texture: Vec<u16>,
    pub width: u32,
    pub height: u32
}

impl Texture {
    pub fn render(&self, ctx: &mut RunContext) {
        let pixels : &[u16] = self.texture.as_ref();
        let pixels_as_u8_ptr = pixels.as_ptr().cast::<u8>();
        let pixels_as_u8_size = (self.width * self.height * 2) as usize;
        let content = unsafe { slice::from_raw_parts(pixels_as_u8_ptr, pixels_as_u8_size) };

        ctx.draw_frame(
            content,
            self.width,
            self.height,
            self.width as usize * 2,
        );
    }
}
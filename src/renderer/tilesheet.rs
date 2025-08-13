use crate::renderer::texture::Texture;
use png::Decoder;
use tar::Entry;

pub struct TileSheet {
    palette: Vec<u16>,
    tile_sheet: Vec<u8>,
    tile_width: u32,
    tile_height: u32,
    columns: u32,
    rows: u32,
}

pub struct Sprite<'a> {
    tile_sheet: &'a TileSheet,
    bounds: Bounds,
}

pub struct Bounds {
    x: u32,
    y: u32,
    width: u32,
    height: u32
}

impl TileSheet {
    pub fn from_png(decoder: Decoder<Entry<&[u8]>>, tile_width: u32, tile_height: u32) -> Self {
        let mut reader = decoder.read_info().unwrap();
        let info = reader.info();
        let mut palette = Vec::new();
        if let Some(png_palette) = &info.palette {
            for color in png_palette.chunks_exact(3) {
                palette.push(color_xrgb565(color[0], color[1], color[2]));
            }
        }
        let mut tile_sheet: Vec<u8> = vec![0; reader.output_buffer_size()];
        let frame_info = reader.next_frame(&mut tile_sheet).unwrap();
        let columns = frame_info.width / tile_width;
        let rows = frame_info.height / tile_height;

        TileSheet { palette, tile_sheet, tile_width, tile_height, columns, rows }
    }

    pub fn width(&self) -> u32 {
        self.tile_width * self.columns
    }

    pub fn sprite(&self, column: u32, row: u32) -> Sprite {
        Sprite {
            tile_sheet: self,
            bounds: Bounds {
                x: column * self.tile_width,
                y: row * self.tile_height,
                width: self.tile_width,
                height: self.tile_height
            }
        }
    }
}

impl <'a> Sprite<'a> {
    pub fn draw_to(&'a self, dst: &mut Texture, x: u32, y: u32) {
        let sheet = self.tile_sheet;
        let frame_x = self.bounds.x;
        let frame_y = self.bounds.y;

        let start_x = x;
        let start_y = y;

        let src = &sheet.tile_sheet;
        let palette = &sheet.palette;

        let src_y = frame_y;
        let dst_y = start_y;
        for y in 0..self.bounds.height {
            let src_pixel = frame_x + ((src_y + y) * sheet.width());
            let dst_pixel = start_x + ((dst_y + y) * dst.width);
            for x in 0..self.bounds.width {
                let pixel = src[(src_pixel + x) as usize];
                if pixel != 0 {
                    dst.texture[(dst_pixel + x) as usize] = palette[pixel as usize];
                }
            }
        }
    }
}

fn color_xrgb565(red: u8, green: u8, blue: u8) -> u16 {
    (((red >> 3) as u16) << 11) + (((green >> 2) as u16) << 5) + (blue >> 3) as u16
}

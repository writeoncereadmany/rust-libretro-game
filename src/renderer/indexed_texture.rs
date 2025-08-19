use png::Decoder;
use tar::Entry;

pub struct IndexedTexture {
    pub palette: Vec<u16>,
    pub texture: Vec<u8>,
}

impl IndexedTexture {
    pub fn from_png(decoder: Decoder<Entry<&[u8]>>) -> Self {
        let mut reader = decoder.read_info().unwrap();
        let info = reader.info();
        let mut palette = Vec::new();
        if let Some(png_palette) = &info.palette {
            for color in png_palette.chunks_exact(3) {
                palette.push(color_xrgb565(color[0], color[1], color[2]));
            }
        }
        let mut texture: Vec<u8> = vec![0; reader.output_buffer_size()];
        let _frame_info = reader.next_frame(&mut texture).unwrap();

        IndexedTexture { palette, texture }

    }
}

fn color_xrgb565(red: u8, green: u8, blue: u8) -> u16 {
    (((red >> 3) as u16) << 11) + (((green >> 2) as u16) << 5) + (blue >> 3) as u16
}

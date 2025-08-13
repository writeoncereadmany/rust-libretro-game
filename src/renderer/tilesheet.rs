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
}

pub fn render(tile_sheet: &TileSheet, dst: &mut Vec<u16>, x: u32, y: u32, row: u32, col: u32, dst_width: u32) {
    let frame_x = col * tile_sheet.tile_width;
    let frame_y = row * tile_sheet.tile_height;

    let start_x = x;
    let start_y = y;

    let src  = &tile_sheet.tile_sheet;
    let palette = &tile_sheet.palette;

    let src_y = frame_y;
    let dst_y = start_y;
    for y in 0..12 {
        let src_pixel = frame_x + ((src_y + y) * (tile_sheet.tile_width * tile_sheet.columns));
        let dst_pixel = start_x + ((dst_y + y) * dst_width);
        for x in 0..12 {
            let pixel = src[(src_pixel + x) as usize];
            if pixel != 0 {
                dst[(dst_pixel + x) as usize] = palette[pixel as usize];
            }
        }
    }

}

fn color_xrgb565(red: u8, green: u8, blue: u8) -> u16 {
    (((red >> 3) as u16) << 11) + (((green >> 2) as u16) << 5) + (blue >> 3) as u16
}

const TRANSPARENT: u16 = 0;

pub fn is_transparent(color: u16) -> bool {
    color == TRANSPARENT
}

// first bit means non-transparent, then rgb bits
pub fn color_xrgb1555(red: u8, green: u8, blue: u8) -> u16 {
    (1 << 15) + (((red >> 3) as u16) << 10) + (((green >> 3) as u16) << 5) + (blue >> 3) as u16
}
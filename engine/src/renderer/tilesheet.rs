use crate::renderer::sprite::{Bounds, Sprite};

pub struct TileSheet {
    pub name: String,
    pub palette: Vec<u16>,
    pub tile_sheet: Vec<u8>,
    pub tile_width: u32,
    pub tile_height: u32,
    pub columns: u32
}


impl TileSheet {
    pub fn new(
        name: &str,
        palette: Vec<u16>,
        tile_sheet: Vec<u8>,
        tile_width: u32,
        tile_height: u32,
        columns: u32,
    ) -> Self {
        TileSheet {
            name: name.to_string(),
            palette,
            tile_sheet,
            tile_width,
            tile_height,
            columns,
        }
    }

    pub fn width(&self) -> u32 {
        self.tile_width * self.columns
    }

    pub fn sprite(self: &Self, column: u32, row: u32) -> Sprite {
        Sprite {
            tile_sheet: self.name.clone(),
            bounds: Bounds {
                x: column * self.tile_width,
                y: row * self.tile_height,
                width: self.tile_width,
                height: self.tile_height,
            },
        }
    }

    pub fn tile(self: &Self, tile_id: u32) -> Sprite {
        self.sprite(tile_id % self.columns, tile_id / self.columns)
    }
}
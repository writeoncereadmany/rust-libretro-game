use std::collections::HashMap;
use tiled::TileId;

pub struct Map {
    pub tiles: Vec<Tile>,
    pub objects: Vec<Object>
}

pub struct Tile {
    pub id: TileId,
    pub x: i32,
    pub y: i32,
    pub tile_set_name: String,
    pub user_type: Option<String>,
}

pub struct Object {
    pub x: f64,
    pub y: f64,
    pub user_type: Option<String>,
    pub properties: HashMap<String, String>
}
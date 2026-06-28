use std::collections::HashMap;
use tiled::TileId;

pub struct Map {
}

impl Map {
    pub fn layers(&self) -> Vec<Layer> {
        vec![]
    }
}

pub enum Layer {
    Tiles(TileLayer),
    Objects(ObjectLayer)
}

impl Layer {
    pub fn as_tile_layer(&self) -> Option<&TileLayer> {
        match (self) {
            Layer::Tiles(tiles) => Some(tiles),
            Layer::Objects(_) => None
        }
    }

    pub fn as_object_layer(&self) -> Option<&ObjectLayer> {
        match (self) {
            Layer::Tiles(_) => None,
            Layer::Objects(objects) => Some(objects)
        }

    }
}

pub struct TileLayer {

}

impl TileLayer {
    pub fn width(&self) -> Option<i32> { Some(0) }
    pub fn height(&self) -> Option<i32> { Some(0) }
    pub fn get_tile(&self, x: i32, y: i32) -> Option<Tile> { None }
}

pub struct Tile {
    tile_set: TileSet,
    map_tile: MapTile
}

impl Tile {
    pub fn get_tileset(&self) -> &TileSet {
        &self.tile_set
    }

    pub fn get_tile(&self) -> Option<&MapTile> {
        Some(&self.map_tile)
    }

    pub fn id(&self) -> TileId {
        0
    }
}

pub struct MapTile {
    pub user_type: Option<String>
}

pub struct TileSet {
    pub name: String
}

pub struct ObjectLayer {

}

impl ObjectLayer {
    pub fn objects(&self) -> Vec<&Object> {
        vec!()
    }
}

pub struct Object {
    pub x: f64,
    pub y: f64,
    pub user_type: Option<String>,
    pub properties: HashMap<String, String>
}

impl Object {
    pub fn user_type(&self) -> &Option<String> {
        &self.user_type
    }
}
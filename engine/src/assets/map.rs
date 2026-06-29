use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use tiled::{PropertyValue, TileId};

#[derive(Serialize, Deserialize)]
pub struct Map {
    pub tiles: Vec<Tile>,
    pub objects: Vec<Object>
}

#[derive(Serialize, Deserialize)]
pub struct Tile {
    pub id: TileId,
    pub x: i32,
    pub y: i32,
    pub tile_set_name: String,
    pub user_type: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Object {
    pub x: f64,
    pub y: f64,
    pub user_type: String,
    pub properties: HashMap<String, String>
}

pub fn load_map(map: tiled::Map) -> Map
{
    let mut tiles = Vec::new();
    let mut objects = Vec::new();
    for layer in map.layers() {
        if let Some(tile_layer) = layer.as_tile_layer() {
            if let (Some(width), Some(height)) = (tile_layer.width(), tile_layer.height()) {
                for x in 0..width as i32 {
                    for y in 0..height as i32 {
                        if let Some(tile) = tile_layer.get_tile(x, y) {
                            let user_type = tile.get_tile().map(|tile| tile.user_type.clone()).flatten();

                            tiles.push(Tile {
                                id: tile.id(),
                                x,
                                y,
                                tile_set_name: tile.get_tileset().name.clone(),
                                user_type,

                            })
                        }
                    }
                }
            }
        }

        if let Some(map_objects) = layer.as_object_layer() {
            for object in map_objects.objects() {
                let mut properties = HashMap::new();
                for (name, value) in &object.properties {
                    match value {
                        PropertyValue::StringValue(value) => { properties.insert(name.clone(), value.clone()); }
                        _ => {}
                    };
                }
                objects.push(Object {
                    x: object.x as f64,
                    y: object.y as f64,
                    user_type: get_user_type(&object).unwrap_or(String::new()),
                    properties
                });
            }
        }
    }
    Map { tiles, objects }
}

fn get_user_type(object : &tiled::Object) -> Option<String> {
    object.get_tile()?.get_tile()?.user_type.clone()
}
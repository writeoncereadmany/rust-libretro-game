use crate::renderer::renderer::Renderer;
use crate::renderer::sprite::Sprite;
use crate::renderer::tilesheet::TileSheet;
use std::collections::HashMap;
use std::sync::Arc;
use tiled::{Object, ObjectLayer, TileLayer};

#[derive(Clone)]
pub struct Map {
    pub layers: Vec<Layer>,
    pub spawns: Vec<Spawn>,
}

#[derive(Clone)]
pub struct Layer {
    pub tiles: Vec<Vec<Option<Tile>>>,
    pub width: usize,
    pub height: usize,
    pub tile_width: i32,
    pub tile_height: i32,
}

#[derive(Clone)]
pub struct Tile {
    pub sprite: Sprite,
    pub tile_type: Option<String>
}

#[derive(Clone)]
pub struct Spawn {
    pub object_type: String,
    pub x: i32,
    pub y: i32
}

impl Map {
    pub fn new(map: &tiled::Map, tilesheets: &HashMap<String, Arc<TileSheet>>) -> Self {
        let mut layers = Vec::new();
        let mut spawns = Vec::new();
        let tile_width = map.tile_width as i32;
        let tile_height = map.tile_height as i32;

        for layer in map.layers() {
            if let Some(tiles) = layer.as_tile_layer() {
                Self::add_tile_layer(&tiles, &mut layers, tilesheets, tile_width, tile_height);
            }
            if let Some(objects) = layer.as_object_layer() {
                Self::add_spawns(&objects, &mut spawns, tile_width, tile_height);
            }
        }
        Map { layers, spawns }
    }

    fn add_tile_layer(
        tiles: &TileLayer,
        layers: &mut Vec<Layer>,
        tilesheets: &HashMap<String, Arc<TileSheet>>,
        tile_width: i32,
        tile_height: i32,
    ) {
        if let (Some(width), Some(height)) = (tiles.width(), tiles.height()) {
            let mut layer = vec![vec![None; height as usize]; width as usize];
            for x in 0..width as i32 {
                for y in 0..height as i32 {
                    if let Some(tile) = tiles.get_tile(x, y) {
                        let tilesheet = tilesheets.get(&tile.get_tileset().name).unwrap();
                        let tile_id = tile.id();
                        layer[x as usize][y as usize] = Some(Tile {
                            sprite: tilesheet.tile(tile_id),
                            tile_type: tile.get_tile().map(|t| t.user_type.clone()).flatten()
                        });
                    }
                }
            }
            layers.push(Layer {
                tiles: layer,
                width: width as usize,
                height: height as usize,
                tile_width,
                tile_height,
            });
        }
    }

    fn add_spawns(objects: &ObjectLayer, spawns: &mut Vec<Spawn>, tile_width: i32, tile_height: i32) {
        for object in objects.objects() {
            if let Some(object_type) = Self::object_type(&object) {
                spawns.push(Spawn { object_type, x: object.x as i32, y: object.y as i32});
            }
        }
    }

    fn object_type(object: &Object) -> Option<String> {
        if object.user_type != "" {
            Some(object.user_type.clone())
        } else {
            object.get_tile()?.get_tile()?.user_type.clone()
        }
    }

    pub fn draw_map(&self, renderer: &mut Renderer, start_x: i32, start_y: i32) {
        for layer in &self.layers {
            for x in 0..layer.width {
                for y in 0..layer.height {
                    if let Some(tile) = &layer.tiles[x][y] {
                        renderer.draw_background(
                            &tile.sprite,
                            x as i32 * layer.tile_width + start_x,
                            y as i32 * layer.tile_height + start_y,
                        );
                    }
                }
            }
        }
    }
}

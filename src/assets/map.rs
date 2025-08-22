use crate::renderer::renderer::Renderer;
use crate::renderer::sprite::Sprite;
use crate::renderer::tilesheet::TileSheet;
use std::collections::HashMap;
use std::sync::Arc;

pub struct Map {
    pub layers: Vec<Layer>,
    pub objects: Vec<Spawn>,
}

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
}

pub struct Spawn;

impl Map {
    pub fn new(map: &tiled::Map, tilesheets: &HashMap<String, Arc<TileSheet>>) -> Self {
        let mut layers = Vec::new();
        let mut objects = Vec::new();

        for layer in map.layers() {
            if let Some(tiles) = layer.as_tile_layer() {
                if let (Some(width), Some(height)) = (tiles.width(), tiles.height()) {
                    let mut layer = vec![vec![None; height as usize]; width as usize];
                    for x in 0..width as i32 {
                        for y in 0..height as i32 {
                            if let Some(tile) = tiles.get_tile(x, y) {
                                let tilesheet = tilesheets.get(&tile.get_tileset().name).unwrap();
                                let tile_id = tile.id();
                                layer[x as usize][y as usize] = Some(Tile {
                                    sprite: tilesheet.tile(tile_id),
                                });
                            }
                        }
                    }
                    layers.push(Layer {
                        tiles: layer,
                        width: width as usize,
                        height: height as usize,
                        tile_width: map.tile_width as i32,
                        tile_height: map.tile_height as i32,
                    });
                }
            }
        }
        Map { layers, objects }
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

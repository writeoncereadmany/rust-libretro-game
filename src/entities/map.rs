use crate::component::graphics::Tile;
use crate::game::game::UpdateBackgroundTile;
use engine::events::event::Events;
use engine::events::spawner::Spawner;

pub fn load_map(map: &tiled::Map, spawner: &Spawner, events: &mut Events) {
    for layer in map.layers() {
        if let Some(tiles) = layer.as_tile_layer() {
            if let (Some(width), Some(height)) = (tiles.width(), tiles.height()) {
                for x in 0..width as i32 {
                    for y in 0..height as i32 {
                        if let Some(tile) = tiles.get_tile(x, y) {
                            let tileset_name = tile.get_tileset().name.clone();
                            let width = tile.get_tileset().tile_width as i32;
                            let height = tile.get_tileset().tile_height as i32;
                            // the map part of the screen does not include the border, which is flashlamps
                            events.fire(UpdateBackgroundTile {
                                x: (x + 1) * width,
                                y: (y + 1) * height,
                                tile: Tile(tileset_name, tile.id()),
                            })
                        }
                    }
                }
            }
        }

        if let Some(objects) = layer.as_object_layer() {
            for object in objects.objects() {
                spawner.spawn(&object, events);
            }
        }
    }
}

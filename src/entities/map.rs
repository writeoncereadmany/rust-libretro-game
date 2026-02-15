use crate::game::game::UpdateBackgroundTile;
use engine::events::event::Events;
use engine::events::spawner::Spawner;

pub fn load_map(map: &tiled::Map, spawner: &Spawner, events: &mut Events) {
    for layer in map.layers() {
        if let Some(tiles) = layer.as_tile_layer() {
            if let (Some(width), Some(height)) = (tiles.width(), tiles.height()) {
                for x in 0..width as i32 {
                    for y in 0..height as i32{
                        if let Some(tile) = tiles.get_tile(x, y) {
                            events.fire(UpdateBackgroundTile {
                                x: (x+1)*12, y: (18-y)*12, tileset: tile.get_tileset().name.clone(), tile: tile.id()
                            });
                            if let Some(_map_tile) = tile.get_tile() {
                                // add to collision data based on user type
                            }
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


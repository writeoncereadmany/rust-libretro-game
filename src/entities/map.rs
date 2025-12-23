use tiled::Object;
use engine::events::event::Events;
use engine::events::spawner::Spawner;


pub fn load_map(map: &tiled::Map, spawner: &Spawner, events: &mut Events) {
    for layer in map.layers() {
        if let Some(tiles) = layer.as_tile_layer() {
            if let (Some(width), Some(height)) = (tiles.width(), tiles.height()) {
                for x in 0..width {
                    for y in 0..height {
                        // and do the rest here
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


use crate::screens::screen::Screen;
use engine::assets::map::Map;
use engine::events::event::{Event, Events};
use engine::renderer::asset_renderer::AssetRenderer;

pub struct InfoScreen(pub Map);


impl Screen for InfoScreen {
    fn on_event(&mut self, event: &Event, events: &mut Events) {
    }

    fn draw(&mut self, renderer: &mut AssetRenderer) {
        renderer.clear();
        let InfoScreen(map) = self;
        for tile in map.tiles.iter() {
            renderer.draw_background_tile(&tile.tile_set_name, tile.id, tile.x * map.tile_width, tile.y * map.tile_height);
        }
        for object in map.objects.iter() {
            renderer.draw_background_tile(&object.tile_set_name, object.id, object.x as i32, object.y as i32);
        }
        renderer.clear_sprites();
    }
}

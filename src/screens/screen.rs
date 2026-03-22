use engine::events::event::{Event, Events};
use engine::renderer::asset_renderer::AssetRenderer;

pub trait Screen {
    fn on_event(&mut self, event: &Event, events: &mut Events);

    fn draw(&mut self, renderer: &mut AssetRenderer);
}
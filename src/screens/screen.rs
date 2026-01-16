use engine::events::event::{Event, Events};
use engine::renderer::asset_renderer::AssetRenderer;
use engine::renderer::renderer::Renderer;

pub trait Screen {
    fn on_event(&mut self, event: &Event, events: &mut Events);

    fn draw(&mut self, renderer: &mut AssetRenderer);
}
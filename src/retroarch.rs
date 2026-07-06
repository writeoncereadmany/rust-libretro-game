use std::sync::Arc;
use engine::assets::Assets;
use engine::events::event::Events;
use engine::renderer::asset_renderer::AssetRenderer;
use rust_libretro::contexts::AudioContext;
use rust_libretro::types::JoypadState;
use tracing_appender::non_blocking::WorkerGuard;

pub trait Application {
    fn new(assets: Arc<Assets>, logger_worker: Option<WorkerGuard>) -> Self;
    
    fn update(&mut self, input: JoypadState, delta_time: u64, renderer: &mut AssetRenderer, events: &mut Events);

    fn draw(&mut self, renderer: &mut AssetRenderer);

    fn play(&mut self, _ctx: &mut AudioContext);
}
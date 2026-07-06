use engine::assets::Assets;
use engine::events::event::Events;
use engine::renderer::asset_renderer::AssetRenderer;
use engine::renderer::renderer::Renderer;
use rust_libretro::contexts::{AudioContext, GenericContext, GetAvInfoContext, InitContext, LoadGameContext, RunContext, SetEnvironmentContext};
use rust_libretro::proc::CoreOptions;
use rust_libretro::sys::{retro_game_geometry, retro_game_info, retro_input_descriptor, retro_system_av_info, retro_system_timing, retro_usec_t, RETRO_DEVICE_ID_JOYPAD_A, RETRO_DEVICE_ID_JOYPAD_DOWN, RETRO_DEVICE_ID_JOYPAD_LEFT, RETRO_DEVICE_ID_JOYPAD_RIGHT, RETRO_DEVICE_ID_JOYPAD_START, RETRO_DEVICE_ID_JOYPAD_UP, RETRO_DEVICE_JOYPAD};
use rust_libretro::types::{JoypadState, PixelFormat, SystemInfo};
use rust_libretro::{
    contexts::*, core::Core, env_version, input_descriptors, proc::*, retro_core, sys::*, types::*,
};
use std::ffi::{c_uint, CString};
use std::slice;
use std::sync::Arc;
use tracing::{span, Level};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::fmt::format::FmtSpan;

pub trait Application {
    fn new(assets: Arc<Assets>, logger_worker: Option<WorkerGuard>) -> Self;
    
    fn update(&mut self, input: JoypadState, delta_time: u64, renderer: &mut AssetRenderer, events: &mut Events);

    fn draw(&mut self, renderer: &mut AssetRenderer);

    fn play(&mut self, _ctx: &mut AudioContext);
    
    fn width() -> u32;
    
    fn height() -> u32;
}

const PIXEL_FORMAT: PixelFormat = PixelFormat::XRGB1555;

const INPUT_DESCRIPTORS: &[retro_input_descriptor] = &input_descriptors!(
    { 0, RETRO_DEVICE_JOYPAD, 0, RETRO_DEVICE_ID_JOYPAD_UP, "Up" },
    { 0, RETRO_DEVICE_JOYPAD, 0, RETRO_DEVICE_ID_JOYPAD_DOWN, "Down" },
    { 0, RETRO_DEVICE_JOYPAD, 0, RETRO_DEVICE_ID_JOYPAD_LEFT, "Left" },
    { 0, RETRO_DEVICE_JOYPAD, 0, RETRO_DEVICE_ID_JOYPAD_RIGHT, "Right" },
    { 0, RETRO_DEVICE_JOYPAD, 0, RETRO_DEVICE_ID_JOYPAD_A, "Jump" },
    { 0, RETRO_DEVICE_JOYPAD, 0, RETRO_DEVICE_ID_JOYPAD_START, "Start" },
);

#[derive(CoreOptions)]
pub struct RetroarchCore<T: Application> {
    pub application: Option<T>,
    pub events: Events,
    pub renderer: Option<AssetRenderer>
}

impl<T: Application> Core for RetroarchCore<T> {
    fn get_info(&self) -> SystemInfo {
        SystemInfo {
            library_name: CString::new("PandaEngine").unwrap(),
            library_version: CString::new(env_version!("CARGO_PKG_VERSION").to_string()).unwrap(),
            valid_extensions: CString::new("").unwrap(),

            need_fullpath: false,
            block_extract: false,
        }
    }

    fn on_set_environment(&mut self, initial: bool, ctx: &mut SetEnvironmentContext) {
        if !initial {
            return;
        }

        ctx.set_support_no_game(true);
    }

    fn on_init(&mut self, ctx: &mut InitContext) {
        let gctx: GenericContext = ctx.into();
        gctx.set_input_descriptors(INPUT_DESCRIPTORS);
    }

    fn on_get_av_info(&mut self, _ctx: &mut GetAvInfoContext) -> retro_system_av_info {
        retro_system_av_info {
            geometry: retro_game_geometry {
                base_width: T::width() as c_uint,
                base_height: T::height() as c_uint,
                max_width: T::width() as c_uint,
                max_height: T::height() as c_uint,
                aspect_ratio: 0.0,
            },
            timing: retro_system_timing {
                fps: 60.0,
                sample_rate: 44100.0,
            },
        }
    }

    fn on_load_game(
        &mut self,
        info: Option<retro_game_info>,
        ctx: &mut LoadGameContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        ctx.set_pixel_format(PIXEL_FORMAT);
        ctx.set_performance_level(0);
        ctx.enable_frame_time_callback((1000000.0f64 / 60.0).round() as retro_usec_t);

        let logger_worker = if std::env::var("LOG_PANDA_TRACES").is_ok() {
            let file_appender = tracing_appender::rolling::hourly("logs", "pandamonium.log");
            let (non_blocking, logger_worker) = tracing_appender::non_blocking(file_appender);
            tracing_subscriber::fmt()
                .json()
                .with_span_events(FmtSpan::CLOSE)
                .with_writer(non_blocking)
                .init();
            Some(logger_worker)
        } else {
            None
        };

        let game_info = info.unwrap();
        let data = unsafe { slice::from_raw_parts(game_info.data as *const u8, game_info.size) };

        let assets = serde_json::from_slice::<Assets>(data).unwrap();
        let assets = Arc::new(assets);
        self.application = Some(T::new(assets.clone(), logger_worker));
        self.renderer = Some(AssetRenderer::new(Renderer::new(T::width(), T::height()), assets.clone()));

        let gctx: GenericContext = ctx.into();
        gctx.enable_audio_callback();

        Ok(())
    }

    #[inline]
    fn on_run(&mut self, ctx: &mut RunContext, delta_us: Option<i64>) {
        let gctx: GenericContext = ctx.into();

        let input = unsafe { ctx.get_joypad_bitmask(0, 0) };

        if input.contains(JoypadState::START) && input.contains(JoypadState::SELECT) {
            return gctx.shutdown();
        }

        if let Some(ref mut application) = self.application {
            if let Some(ref mut renderer) = self.renderer {
                {
                    let span = span!(Level::INFO, "Updating game state");
                    let _span = span.enter();
                    application.update(input, delta_us.unwrap_or(16_666) as u64, renderer, &mut self.events);
                }
                {
                    let span = span!(Level::INFO, "Drawing scene");
                    let _span = span.enter();
                    application.draw(renderer);
                }
            }
        }
        if let Some (ref mut renderer) = self.renderer {
            let span = span!(Level::INFO, "Rendering to context");
            let _span = span.enter();
            renderer.present(ctx);
        }
    }

    fn on_write_audio(&mut self, ctx: &mut AudioContext) {
        if let Some(ref mut application) = self.application {
            application.play(ctx);
        }
    }
}

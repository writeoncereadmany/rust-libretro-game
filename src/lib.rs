mod renderer;
mod assets;

use crate::assets::assets::Assets;
use crate::renderer::texture::Texture;
use crate::renderer::tilesheet::TileSheet;
use rand::prelude::*;
use rust_libretro::{
    contexts::*, core::Core, env_version, input_descriptors, proc::*, retro_core, sys::*, types::*,
};
use std::ffi::c_uint;
use std::ffi::CString;
use std::slice;
use tar::Archive;

const WIDTH: c_uint = 360;
const HEIGHT: c_uint = 240;
const PIXEL_FORMAT: PixelFormat = PixelFormat::RGB565;

const INPUT_DESCRIPTORS: &[retro_input_descriptor] = &input_descriptors!(
    { 0, RETRO_DEVICE_JOYPAD, 0, RETRO_DEVICE_ID_JOYPAD_UP, "Up" },
    { 0, RETRO_DEVICE_JOYPAD, 0, RETRO_DEVICE_ID_JOYPAD_DOWN, "Down" },
    { 0, RETRO_DEVICE_JOYPAD, 0, RETRO_DEVICE_ID_JOYPAD_LEFT, "Left" },
    { 0, RETRO_DEVICE_JOYPAD, 0, RETRO_DEVICE_ID_JOYPAD_RIGHT, "Right" },
    { 0, RETRO_DEVICE_JOYPAD, 0, RETRO_DEVICE_ID_JOYPAD_A, "Action" },
);

#[derive(CoreOptions)]
#[categories({
    "advanced_settings",
    "Advanced",
    "Options affecting low-level emulation performance and accuracy."
},{
    "not_so_advanced_settings",
    "Not So Advanced",
    "Options not affecting low-level emulation performance and accuracy."
})]
#[options({
    "foo_option_1",
    "Advanced > Speed hack coprocessor X",
    "Speed hack coprocessor X",
    "Setting 'Advanced > Speed hack coprocessor X' to 'true' or 'Turbo' provides increased performance at the expense of reduced accuracy",
    "Setting 'Speed hack coprocessor X' to 'true' or 'Turbo' provides increased performance at the expense of reduced accuracy",
    "advanced_settings",
    {
        { "false" },
        { "true" },
        { "unstable", "Turbo (Unstable)" },
    }
}, {
    "foo_option_2",
    "Simple > Toggle Something",
    "Toggle Something",
    "Setting 'Simple > Toggle Something' to 'true' does something.",
    "Setting 'Toggle Something' to 'true' does something.",
    "not_so_advanced_settings",
    {
        { "false" },
        { "true" },
    }
})]
struct ExampleCore {
    option_1: bool,
    option_2: bool,

    assets: Assets,
    x: f64,
    y: f64,
    texture: Texture}

retro_core!(ExampleCore {
    option_1: false,
    option_2: true,

    assets: Assets::new(),
    x: 100.0,
    y: 100.0,
    texture: Texture {
        texture: vec![0; WIDTH as usize * HEIGHT as usize],
        width: WIDTH,
        height: HEIGHT
    }
});

impl Core for ExampleCore {
    fn get_info(&self) -> SystemInfo {
        SystemInfo {
            library_name: CString::new("Example Core").unwrap(),
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
                base_width: WIDTH,
                base_height: HEIGHT,
                max_width: WIDTH,
                max_height: HEIGHT,
                aspect_ratio: 0.0,
            },
            timing: retro_system_timing {
                fps: 60.0,
                sample_rate: 0.0,
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

        let game_info = info.unwrap();
        let data = unsafe { slice::from_raw_parts(game_info.data as *const u8, game_info.size) };
        let mut archive = Archive::new(data);

        self.assets.load_assets(&mut archive);

        let gctx: GenericContext = ctx.into();
        gctx.enable_audio_callback();

        Ok(())
    }

    fn on_options_changed(&mut self, ctx: &mut OptionsChangedContext) {
        match ctx.get_variable("foo_option_1") {
            Some("true") => self.option_1 = true,
            Some("false") => self.option_1 = false,
            _ => (),
        }

        match ctx.get_variable("foo_option_2") {
            Some("true") => self.option_2 = true,
            Some("false") => self.option_2 = false,
            _ => (),
        }
    }

    #[inline]
    fn on_run(&mut self, ctx: &mut RunContext, delta_us: Option<i64>) {
        let gctx: GenericContext = ctx.into();

        let input = unsafe { ctx.get_joypad_bitmask(0, 0) };

        if input.contains(JoypadState::START) && input.contains(JoypadState::SELECT) {
            return gctx.shutdown();
        }
        let speed = 100.0;
        let delta_s = (delta_us.unwrap_or(16_666) as f64) / 1_000_000.0;
        if input.contains(JoypadState::UP) {
            self.y -= delta_s * speed
        }
        if input.contains(JoypadState::DOWN) {
            self.y += delta_s * speed
        }
        if input.contains(JoypadState::LEFT) {
            self.x -= delta_s * speed
        }
        if input.contains(JoypadState::RIGHT) {
            self.x += delta_s * speed
        }

        self.texture.texture.fill((24 << 5) + 24);

        let sprite = TileSheet::sprite(&self.assets.tilesheets.get("Sprites").unwrap(), 2, 1);
        sprite.draw_to(&mut self.texture, self.x as i32, self.y as i32);

        self.assets.fonts.get("Spritefont_Medium").unwrap().draw_text(&mut self.texture, 10, 10, "Hello, World!");
        self.assets.fonts.get("Spritefont_Small").unwrap().draw_text(&mut self.texture, 10, 20, "Hello, World!");

        self.texture.render(ctx);
    }

    fn on_write_audio(&mut self, ctx: &mut AudioContext) {
        ctx.queue_audio_sample(0, 0);
    }
}
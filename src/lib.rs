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

    sprite_sheet_height: u32,
    sprite_sheet_width: u32,
    sprite_sheet: Vec<u16>,
    pixels: Vec<u8>,
}

retro_core!(ExampleCore {
    option_1: false,
    option_2: true,

    sprite_sheet_height: 0,
    sprite_sheet_width: 0,
    sprite_sheet: Vec::new(),
    pixels: vec![0; WIDTH as usize * HEIGHT as usize * PIXEL_FORMAT.bit_per_pixel()]
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

        for entry in archive.entries().unwrap() {
            let unwrapped_entry = entry.unwrap();
            let decoder = png::Decoder::new(unwrapped_entry);
            let mut reader = decoder.read_info().unwrap();
            let info = reader.info();
            let mut palette: Vec<u16> = Vec::new();
            if let Some(png_palette) = &info.palette {
                for color in png_palette.chunks_exact(3) {
                    palette.push(color_xrgb565(color[0], color[1], color[2]));
                }
                println!("Palette loaded");
                println!("Info: {:?}", info);
            }
            let mut vec: Vec<u8> = vec![0; reader.output_buffer_size()];
            match reader.next_frame(&mut vec) {
                Ok(frame_info) => {
                    println!("Frame found");
                    for pixel in vec {
                        self.sprite_sheet.push(palette[pixel as usize]);
                    }
                    self.sprite_sheet_width = frame_info.width;
                    self.sprite_sheet_height = frame_info.height;
                    println!(
                        "Loaded a sprite sheet, with dimensions {}x{}",
                        frame_info.width, frame_info.height
                    );
                }
                Err(e) => {
                    println!("Failed to get next frame: {}", e);
                }
            }
        }

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

        self.pixels.fill(0);

        let frame_x = 24;
        let frame_y = 12;

        let start_x = 100;
        let start_y = 100;

        let src = &self.sprite_sheet;

        for x in 0.. 12 {
            for y in 0.. 12 {
                let src_x = frame_x + x;
                let src_y = frame_y + y;
                let src_pixel = src_x + (src_y * self.sprite_sheet_width);
                let dst_x = start_x + x;
                let dst_y = start_y + y;
                let dst_pixel = ((dst_x) + ((dst_y) * WIDTH)) * 2;
                let src_pixel = src[src_pixel as usize];
                self.pixels[dst_pixel as usize] = src_pixel as u8;
                self.pixels[(dst_pixel + 1) as usize] = (src_pixel >> 8) as u8;
            }
        }

        ctx.draw_frame(
            self.pixels.as_ref(),
            WIDTH,
            HEIGHT,
            WIDTH as usize * PIXEL_FORMAT.bit_per_pixel(),
        );
    }

    fn on_write_audio(&mut self, ctx: &mut AudioContext) {
        ctx.queue_audio_sample(0, 0);
    }
}

fn color_xrgb565(red: u8, green: u8, blue: u8) -> u16 {
    (((red >> 3) as u16) << 11) + (((green >> 2) as u16) << 5) + (blue >> 3) as u16
}

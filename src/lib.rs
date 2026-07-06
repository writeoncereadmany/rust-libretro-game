mod app;
mod game;
mod screens;
mod component;
mod entities;
mod export;

use crate::app::pandamonium::Pandamonium;
use engine::events::event::Events;
use engine::retroarch::RetroarchCore;
use rust_libretro::{retro_core};

retro_core!(RetroarchCore::<Pandamonium> {
    application: None,
    renderer: None,
    events: Events::new()
});

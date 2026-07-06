mod app;
mod game;
mod screens;
mod component;
mod entities;
mod export;
pub mod retroarch;

use crate::app::pandamonium::Pandamonium;
use crate::retroarch::*;
use engine::events::event::Events;
use rust_libretro::{retro_core};

retro_core!(RetroarchCore::<Pandamonium> {
    application: None,
    renderer: None,
    events: Events::new()
});

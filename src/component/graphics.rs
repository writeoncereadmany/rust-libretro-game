use crate::component::time::Phase;
use derive::{system, Constant, Variable};
use engine::entities::entity::Entities;
use engine::events::event::Events;
use std::time::Duration;
use engine::renderer::spritefont::Alignment;

#[derive(Clone, Variable)]
pub enum Sprite {
    NamedSprite(String, u32, bool),
    TileSprite(String, u32, u32),
    Text(String, String, Alignment, u32)
}

impl Sprite {
    pub fn sprite(name: &'static str, layer: u32) -> Self {
        Sprite::NamedSprite(name.to_string(), layer, false)
    }

    pub fn sprite_ex(name: String, layer: u32, flip_x: bool) -> Self {
        Sprite::NamedSprite(name, layer, flip_x)
    }

    pub fn tile(tileset: String, tile: u32, layer: u32) -> Self {
        Sprite::TileSprite(tileset, tile, layer)
    }
    
    pub fn text(text: String, tileset: String, alignment: Alignment, layer: u32) -> Self {
        Sprite::Text(text, tileset, alignment, layer)
    }

    pub fn layer(&self) -> u32 {
        match self {
            Sprite::NamedSprite(_, layer, _) => *layer,
            Sprite::TileSprite(_, _, layer) => *layer,
            Sprite::Text(_, _, _, layer) => *layer,
        }
    }
}

#[derive(Clone, Constant)]
pub struct Animation {
    pub sprites: Vec<&'static str>,
    pub layer: u32,
}

#[system]
fn update_sprite_from_phase(_dt: &Duration, world: &mut Entities, _events: &mut Events) {
    world.apply(|(Animation { sprites, layer }, Phase(phase))| {
        let new_sprite_index = (phase * sprites.len() as f64) as usize % sprites.len();
        Sprite::sprite(sprites[new_sprite_index], layer)
    })
}
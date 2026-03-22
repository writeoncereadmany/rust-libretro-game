use derive::{Constant, Variable};
use engine::entities::entity::Entities;
use engine::events::dispatcher::Dispatcher;
use engine::events::event::Events;
use std::time::Duration;
use crate::component::time::Phase;

#[derive(Clone, Variable)]
pub struct Sprite(pub &'static str, pub u32, pub bool);

impl Sprite {
    pub fn sprite(name: &'static str, layer: u32) -> Self {
        Sprite(name, layer, false)
    }

    pub fn sprite_ex(name: &'static str, layer: u32, flip_x: bool) -> Self {
        Sprite(name, layer, flip_x)
    }
}

#[derive(Clone, Constant)]
pub struct Animation {
    pub sprites: Vec<&'static str>,
    pub layer: u32,
}

pub fn register(dispatcher: &mut Dispatcher) {
    dispatcher.register(update_sprite_from_phase);
}

fn update_sprite_from_phase(_dt: &Duration, world: &mut Entities, _events: &mut Events) {
    world.apply(|(Animation { sprites, layer }, Phase(phase))| {
        let new_sprite_index = (phase * sprites.len() as f64) as usize % sprites.len();
        Sprite::sprite(sprites[new_sprite_index], layer)
    })
}
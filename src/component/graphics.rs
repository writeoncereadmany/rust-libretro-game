use derive::{Constant, Variable};
use engine::entities::entity::Entities;
use engine::events::dispatcher::Dispatcher;
use engine::events::event::Events;
use std::time::Duration;

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
    pub period: f64
}

#[derive(Clone, Variable)]
pub struct Phase(pub f64);

pub fn register(dispatcher: &mut Dispatcher) {
    dispatcher.register(update_phase);
}

fn update_phase(dt: &Duration, world: &mut Entities, _events: &mut Events) {
    world.apply(|(Animation { sprites, layer, period }, Phase(p))| {
        let new_phase = p + (dt.as_secs_f64() / period) % 1.0;
        let new_sprite_index = (new_phase * sprites.len() as f64) as usize % sprites.len();
        (Phase(new_phase), Sprite::sprite(sprites[new_sprite_index], layer))
    })
}
use std::time::Duration;
use engine::entities::entity;
use derive::{Constant, Variable};
use engine::events::dispatcher::Dispatcher;

#[derive(Clone, Variable)]
pub struct Sprite(pub &'static str);

#[derive(Clone, Constant)]
pub struct Animation {
    pub sprites: Vec<&'static str>,
    pub period: f64
}

#[derive(Clone, Variable)]
pub struct Phase(pub f64);

pub fn register(dispatcher: &mut Dispatcher) {
    dispatcher.register(|dt: &Duration, world, events| {
        world.apply(|(Animation { sprites, period }, Phase(p))| {
            let new_phase = p + (dt.as_secs_f64() / period) % 1.0;
            let new_sprite_index = (new_phase * sprites.len() as f64) as usize % sprites.len();
            (Phase(new_phase), Sprite(sprites[new_sprite_index]))
        })
    });
}
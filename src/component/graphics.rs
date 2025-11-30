use crate::entities::entity;
use derive::{Constant, Variable};
use crate::renderer::sprite::Sprite;


#[derive(Clone, Constant)]
pub struct Animation {
    pub sprites: Vec<Sprite>,
    pub period: f64
}

#[derive(Clone, Variable)]
pub struct Phase(pub f64);

use engine::entities::entity;
use derive::{Constant, Variable};


#[derive(Clone, Variable)]
pub struct Sprite(pub &'static str);

#[derive(Clone, Constant)]
pub struct Animation {
    pub sprites: Vec<&'static str>,
    pub period: f64
}

#[derive(Clone, Variable)]
pub struct Phase(pub f64);

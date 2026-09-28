use std::time::Duration;
use derive::system;
use engine::entities::entity::{entity, Entities};
use engine::events::event::Events;
use crate::component::collisions::Splash;
use crate::component::graphics::{Animation, Sprite};
use crate::component::lifecycle::Destroy;
use crate::component::physics::Position;
use crate::component::time::{Period, Phase};

#[system]
pub fn spawn_splash(&Splash {x, y, ..}: &Splash, world: &mut Entities, events: &mut Events) {
    let splash_id = world.spawn(entity()
        .with(Sprite::sprite("splash_1", 4))
        .with(Position(x - 6.0, y))
        .with(Animation{ sprites: vec!["splash_1", "splash_2", "splash_3"], layer: 4})
        .with(Phase(0.0))
        .with(Period(0.3))
    );
    events.schedule("Game", Duration::from_secs_f64(0.3), Destroy(splash_id));
}
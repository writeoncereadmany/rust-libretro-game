use engine::events::dispatcher::Dispatcher;

pub mod physics;
pub mod graphics;
pub mod collisions;
pub mod lifecycle;
pub mod time;

pub fn register(dispatcher: &mut Dispatcher) {
    graphics::register(dispatcher);
    physics::register(dispatcher);
    collisions::register(dispatcher);
    lifecycle::register(dispatcher);
    time::register(dispatcher);
}
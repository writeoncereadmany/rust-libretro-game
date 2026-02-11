use engine::events::dispatcher::Dispatcher;

pub mod physics;
pub mod graphics;

pub fn register(dispatcher: &mut Dispatcher) {
    graphics::register(dispatcher);
    physics::register(dispatcher);
}
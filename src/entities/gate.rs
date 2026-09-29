use crate::component::graphics::Sprite;
use crate::component::physics::Position;
use crate::entities::map::CollisionType;
use derive::{spawn, system, Event};
use engine::entities::entity::{entity, Entities};
use engine::events::event::Events;
use engine::events::spawner::Spawn;
use engine::shapes::shape::Shape;

#[derive(Event, Clone)]
struct SpawnGate {
    x: f64,
    y: f64,
    toll: Option<i32>,
    tileset: String,
    tile: u32
}

#[spawn("Gate")]
fn gate(spawn: Spawn, events: &mut Events) {
    let toll: Option<i32> = spawn.object.properties.get("toll").and_then(|s| s.parse().ok());

    events.fire(SpawnGate { x: spawn.x, y: spawn.y, toll, tileset: spawn.object.tile_set_name.clone(), tile: spawn.object.id});
}

#[system]
fn spawn_gate(gate: &SpawnGate, world: &mut Entities, events: &mut Events) {
    world.spawn(entity()
        .with(Position(gate.x, gate.y))
        .with(CollisionType::WALL)
        .with(Shape::bbox(0.0, 0.0, 12.0, 12.0))
        .with(Sprite::tile(gate.tileset.clone(), gate.tile, 8))
    );
}
use crate::component::graphics::Sprite;
use crate::component::physics::Position;
use crate::entities::map::CollisionType;
use derive::{Constant, Event, spawn, system};
use engine::entities::entity::{Entities, entity, EntityId, Id, Not, not};
use engine::events::event::Events;
use engine::events::spawner::Spawn;
use engine::renderer::spritefont::{Alignment, HorizontalAlignment, VerticalAlignment};
use engine::shapes::shape::Shape;
use crate::component::collisions::{Collided, Interactable};
use crate::component::lifecycle::Destroy;
use crate::entities::chest::{PickupRuby, Ruby};
use crate::entities::failureballs::SpawnFailureBall;
use crate::entities::hero::Hero;
use crate::game::game::{Pay, SetTotalScore};

#[derive(Constant, Clone)]
struct Toll(u32);

#[derive(Constant, Clone)]
struct Gate();

#[derive(Constant, Clone)]
struct Tollsign();

#[derive(Event, Clone)]
struct SpawnGate {
    x: f64,
    y: f64,
    tileset: String,
    tile: u32,
}

#[derive(Event, Clone)]
struct SpawnTollsign {
    x: f64,
    y: f64,
    toll: Option<u32>,
    tileset: String,
    tile: u32,
}

#[derive(Event, Clone)]
struct OpenTollgate();

#[derive(Event, Clone)]
struct DestroyGate();

#[spawn("Gate")]
fn gate(spawn: Spawn, events: &mut Events) {
    events.fire(SpawnGate {
        x: spawn.x,
        y: spawn.y,
        tileset: spawn.object.tile_set_name.clone(),
        tile: spawn.object.id,
    });
}

#[spawn("Tollsign")]
fn tollsign(spawn: Spawn, events: &mut Events) {
    let toll: Option<u32> = spawn
        .object
        .properties
        .get("toll")
        .and_then(|s| s.parse().ok());

    events.fire(SpawnTollsign {
        x: spawn.x,
        y: spawn.y,
        toll,
        tileset: spawn.object.tile_set_name.clone(),
        tile: spawn.object.id,
    });
}

#[system]
fn spawn_gate(gate: &SpawnGate, world: &mut Entities, events: &mut Events) {
    world.spawn(
        entity()
            .with(Gate())
            .with(Position(gate.x, gate.y))
            .with(CollisionType::WALL)
            .with(Shape::bbox(0.0, 0.0, 12.0, 12.0))
            .with(Sprite::tile(gate.tileset.clone(), gate.tile, 8)),
    );
}

#[system]
fn spawn_tollsign(tollsign: &SpawnTollsign, world: &mut Entities, events: &mut Events) {
    world.spawn(
        entity()
            .with(Tollsign())
            .with(Position(tollsign.x, tollsign.y))
            .with(Sprite::tile(tollsign.tileset.clone(), tollsign.tile, 9)),
    );

    if let Some(toll) = tollsign.toll {
        world.spawn(
            entity()
                .with(Position(tollsign.x + 12.0, tollsign.y + 6.0))
                .with(Toll(toll))
                .with(Sprite::text(
                    toll.to_string(),
                    "Spritefont_Small".to_string(),
                    Alignment::aligned(HorizontalAlignment::CENTER, VerticalAlignment::MIDDLE),
                    10,
                )),
        );
    }
}

#[system]
fn toll_afforded(&SetTotalScore(score): &SetTotalScore, world: &mut Entities, events: &mut Events) {
    world.apply(|Toll(toll)| {
        if score >= toll {
            events.fire(OpenTollgate())
        }
    })
}

#[system]
fn open_tollgate(_: &OpenTollgate, world: &mut Entities, events: &mut Events) {
    world.apply(|(Gate())| { (not::<CollisionType>(), Interactable()) });
}

#[system]
fn destroy_gate(Collided(first, second, _): &Collided, world: &mut Entities, events: &mut Events) {
    world.apply_to_pair(first, second, |(Interactable(), Gate()), Hero()| events.fire(DestroyGate()));
    world.apply_to_pair(second, first, |(Interactable(), Gate()), Hero()| events.fire(DestroyGate()));
}

#[system]
fn explode_gate(_: &DestroyGate, world: &mut Entities, events: &mut Events) {
    world.apply(|(Id(id), Gate(), Position(x, y))| {
        events.fire(Destroy(id));
        events.fire(SpawnFailureBall { sprite: "small_ball_yellow".to_string(), dx: rand::random_range(-200.0..200.0), dy: rand::random_range(0.0 .. 200.0), position: (x + 4.0, y + 4.0) });
        events.fire(SpawnFailureBall { sprite: "small_ball_red".to_string(), dx: rand::random_range(-200.0..200.0), dy: rand::random_range(0.0 .. 200.0), position: (x + 4.0, y + 8.0) });
        events.fire(SpawnFailureBall { sprite: "small_ball_green".to_string(), dx: rand::random_range(-200.0..200.0), dy: rand::random_range(0.0 .. 200.0), position: (x + 8.0, y + 4.0) });
        events.fire(SpawnFailureBall { sprite: "small_ball_blue".to_string(), dx: rand::random_range(-200.0..200.0), dy: rand::random_range(0.0 .. 200.0), position: (x + 8.0, y + 8.0) });
    });
    world.apply(|(Id(id), Tollsign())| { events.fire(Destroy(id))});
    world.apply(|(Id(id), Toll(toll))| {
        events.fire(Destroy(id));
        events.fire(Pay(toll));
    });
}
use crate::component::physics::{Position, Translation, Velocity};
use derive::{Constant, Event};
use engine::entities::entity::{Entities, EntityId};
use engine::entities::entity::Id;
use engine::events::dispatcher::Dispatcher;
use engine::events::event::Events;
use engine::shapes::collision::Collision;
use engine::shapes::shape::Shape;
use engine::shapes::vec2d::{Vec2d, UNIT_X, UNIT_Y};
use crate::entities::map::{overlapping, Tile};
use crate::entities::map::Tile::LEDGE;

#[derive(Event)]
pub struct CheckCollisions;

#[derive(Event)]
pub struct ResolveCollisions;

#[derive(Event)]
pub struct Collided(pub EntityId, pub EntityId);

#[derive(Event)]
pub struct Push(pub EntityId, pub (f64, f64));

#[derive(Constant, Clone)]
pub struct Actor;

#[derive(Constant, Clone)]
pub struct Pickup;

const EPSILON: f64 = 1e-8;

pub fn register(dispatcher: &mut Dispatcher) {
    dispatcher.register(handle_collisions);
    dispatcher.register(handle_push);
}

pub fn handle_collisions(_ : &CheckCollisions, world: &mut Entities, events: &mut Events)
{
    let tile_maps = world.collect();

    world.apply(|(Actor, Id(hero_id), hero_shape, hero_position@Position(x, y), hero_translation@Translation(tx, ty))| {
        let collidables = overlapping(&tile_maps, &hero_shape, &hero_position, &hero_translation);

        let mut mtx = tx;
        let mut mty = ty;
        let mut push_x = 0.0;
        let mut push_y = 0.0;
        let starting_shape = hero_shape.translate(&(x, y));
        while let Some(next_collision) = next_collision(&starting_shape, &collidables, &(mtx, mty)) {
            let (px, py) = next_collision.push;
            mtx += px + EPSILON.copysign(px);
            mty += py + EPSILON.copysign(py);
            push_x += px;
            push_y += py;
        }
        events.fire(Push(hero_id, (push_x, push_y)));
        Translation(mtx, mty)
    });

    world.for_each_pair(|(Actor, Id(hero_id), hero_shape, hero_position, hero_translation), 
                         (Pickup, Id(pickup_id), pickup_shape, pickup_position, pickup_translation)|
        {
            if collides(hero_shape, hero_position, hero_translation, pickup_shape, pickup_position, pickup_translation)
            {
                events.fire(Collided(*hero_id, *pickup_id));
            }
        }
    );

    events.fire(ResolveCollisions);
}

fn collides(moving: &Shape, &Position(mx, my): &Position, &Translation(mtx, mty): &Translation,
            other: &Shape, &Position(ox, oy): &Position, other_translation: &Option<Translation>) -> bool
{
    let moving = moving.translate(&(mx, my));
    let other = other.translate(&(ox, oy));
    if let Some(Translation(otx, oty)) = other_translation {
        moving.intersects_moving(&other, &(mtx - otx, mty - oty))
    }
    else {  
        moving.intersects_moving(&other, &(mtx, mty))
    }
}

fn next_collision(shape: &Shape, collidables: &Vec<(Shape, Tile)>, translation: &(f64, f64)) -> Option<Collision> {
    let mut collisions: Vec<Collision> = collidables.iter()
        .map(|(collidable, tile)| {
            if let Some(collision) = shape.collides(collidable, translation) {
                if tile == &LEDGE && (collision.push.dot(&UNIT_X).abs() > 1e-6 || collision.push.dot(&UNIT_Y) < 0.0)  {
                    None
                }
                else {
                    Some(collision)
                }
            } else {
                None
            }
        })
        .flatten()
        .collect();

    collisions.sort_unstable_by(|c1, c2| c1.dt.total_cmp(&c2.dt).reverse());
    collisions.pop()
}


fn handle_push(Push(entityId, (px, py)): &Push, world: &mut Entities, _events: &mut Events) {
    world.apply_to(entityId, |Velocity(dx, dy)| Velocity(limit(&dx, px), limit(&dy, py)));
}

fn limit (velocity: &f64, push: &f64) -> f64 {
    if (velocity < &0.0 && push > &0.0) || (velocity > &0.0 && push < &0.0) {
        0.0
    } else {
        *velocity
    }
}